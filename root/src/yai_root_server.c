/* SPDX-License-Identifier: Apache-2.0 */
#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <limits.h>
#include <time.h>
#include <errno.h>
#include <sys/socket.h>
#include <sys/un.h>

#include <transport.h>
#include <yai_protocol_ids.h>
#include <roles.h>
#include <errors.h>
#include <protocol.h> /* yai_handshake_req_t / yai_handshake_ack_t */

#include "control_transport.h"
#include "ws_id.h"

static FILE *root_log = NULL;

/* ============================================================
   LOGGING
   ============================================================ */

#define LOG(fmt, ...)                                     \
    do {                                                  \
        fprintf(stdout, fmt "\n", ##__VA_ARGS__);         \
        if (root_log && root_log != stdout)               \
            fprintf(root_log, fmt "\n", ##__VA_ARGS__);   \
        fflush(stdout);                                   \
        if (root_log && root_log != stdout)               \
            fflush(root_log);                             \
    } while (0)

static void log_init(const char *home)
{
    char path[PATH_MAX];

    snprintf(path, sizeof(path),
             "%s/.yai/run/root/root.log", home);

    root_log = fopen(path, "a");

    if (!root_log) {
        fprintf(stderr,
                "[ROOT] Failed to open log file: %s (%s)\n",
                path, strerror(errno));
        root_log = stdout;
    } else {
        setvbuf(root_log, NULL, _IOLBF, 0);
    }

    time_t now = time(NULL);
    LOG("\n=== ROOT START %ld ===", now);
}

/* ============================================================
   SEND RESPONSE (STRICT & SYMMETRIC)
   ============================================================ */

static int send_response(int fd,
                         const yai_rpc_envelope_t *req,
                         uint32_t cmd,
                         const void *payload,
                         uint32_t payload_len)
{
    yai_rpc_envelope_t resp;
    memset(&resp, 0, sizeof(resp));

    resp.magic       = YAI_FRAME_MAGIC;
    resp.version     = YAI_PROTOCOL_IDS_VERSION;
    resp.command_id  = cmd;
    resp.payload_len = payload_len;

    /* Preserve identity */
    snprintf(resp.ws_id, sizeof(resp.ws_id), "%s", req->ws_id);
    snprintf(resp.trace_id, sizeof(resp.trace_id), "%s", req->trace_id);

    /* Mirror authority fields (optional but consistent) */
    resp.role     = req->role;
    resp.arming   = req->arming;
    resp.checksum = 0;

    if (yai_control_write_frame(fd, &resp, payload) != 0) {
        LOG("[ROOT] write_frame failed");
        return -1;
    }

    return 0;
}

static int send_error_response(int fd,
                               const yai_rpc_envelope_t *req,
                               uint32_t code,
                               const char *reason)
{
    char payload[256];
    int n = snprintf(payload,
                     sizeof(payload),
                     "{\"status\":\"error\",\"code\":%u,\"reason\":\"%s\"}",
                     code,
                     reason ? reason : "unknown");
    if (n <= 0 || (size_t)n >= sizeof(payload))
        return -1;

    yai_rpc_envelope_t safe_req;
    memset(&safe_req, 0, sizeof(safe_req));
    if (req)
        safe_req = *req;

    return send_response(fd,
                         &safe_req,
                         safe_req.command_id ? safe_req.command_id : YAI_CMD_CONTROL,
                         payload,
                         (uint32_t)n);
}

static int is_valid_role(uint16_t role)
{
    return role == YAI_ROLE_NONE ||
           role == YAI_ROLE_USER ||
           role == YAI_ROLE_OPERATOR ||
           role == YAI_ROLE_SYSTEM;
}


static int connect_kernel_socket(void)
{
    const char *home = getenv("HOME");
    if (!home || !home[0])
        home = "/tmp";

    char path[PATH_MAX];
    snprintf(path, sizeof(path), "%s/.yai/run/kernel/control.sock", home);

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0)
        return -1;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    snprintf(addr.sun_path, sizeof(addr.sun_path), "%s", path);

    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -1;
    }

    return fd;
}

static int forward_to_kernel_and_relay(int client_fd,
                                       const yai_rpc_envelope_t *env,
                                       const void *payload,
                                       uint32_t payload_len)
{
    int kfd = connect_kernel_socket();
    if (kfd < 0) {
        LOG("[ROOT] Reject internal: cannot connect kernel");
        (void)send_error_response(client_fd, env, YAI_E_INTERNAL_ERROR, "kernel_connect_failed");
        return -1;
    }

    yai_rpc_envelope_t kreq;
    memset(&kreq, 0, sizeof(kreq));
    kreq.magic = YAI_FRAME_MAGIC;
    kreq.version = YAI_PROTOCOL_IDS_VERSION;
    kreq.command_id = YAI_CMD_HANDSHAKE;
    kreq.payload_len = (uint32_t)sizeof(yai_handshake_req_t);
    snprintf(kreq.ws_id, sizeof(kreq.ws_id), "%s", env->ws_id);
    snprintf(kreq.trace_id, sizeof(kreq.trace_id), "%s", env->trace_id);
    kreq.role = env->role;
    kreq.arming = env->arming;
    kreq.checksum = 0;

    yai_handshake_req_t hs;
    memset(&hs, 0, sizeof(hs));
    hs.client_version = YAI_PROTOCOL_IDS_VERSION;
    hs.capabilities_requested = 0;
    snprintf(hs.client_name, sizeof(hs.client_name), "yai-root");

    if (yai_control_write_frame(kfd, &kreq, &hs) != 0) {
        LOG("[ROOT] Reject internal: kernel handshake write failed");
        close(kfd);
        (void)send_error_response(client_fd, env, YAI_E_INTERNAL_ERROR, "kernel_handshake_write_failed");
        return -1;
    }

    yai_rpc_envelope_t kresp;
    char kpayload[YAI_MAX_PAYLOAD];
    ssize_t hr = yai_control_read_frame(kfd, &kresp, kpayload, sizeof(kpayload));
    if (hr < 0 || kresp.command_id != YAI_CMD_HANDSHAKE) {
        LOG("[ROOT] Reject internal: kernel handshake read failed");
        close(kfd);
        (void)send_error_response(client_fd, env, YAI_E_INTERNAL_ERROR, "kernel_handshake_read_failed");
        return -1;
    }

    if (yai_control_write_frame(kfd, env, payload_len ? payload : NULL) != 0) {
        LOG("[ROOT] Reject internal: write to kernel failed");
        close(kfd);
        (void)send_error_response(client_fd, env, YAI_E_INTERNAL_ERROR, "kernel_write_failed");
        return -1;
    }

    yai_rpc_envelope_t resp;
    char resp_payload[YAI_MAX_PAYLOAD];
    ssize_t r = yai_control_read_frame(kfd, &resp, resp_payload, sizeof(resp_payload));
    close(kfd);

    if (r < 0) {
        LOG("[ROOT] Reject internal: read from kernel failed");
        (void)send_error_response(client_fd, env, YAI_E_INTERNAL_ERROR, "kernel_read_failed");
        return -1;
    }

    if (yai_control_write_frame(client_fd, &resp, resp_payload) != 0) {
        LOG("[ROOT] Relay write to client failed");
        return -1;
    }

    return 0;
}

/* ============================================================
   HANDLE CLIENT
   ============================================================ */

static void handle_client(int cfd)
{
    LOG("[ROOT] Client connected");

    int handshake_done = 0;

    for (;;) {

        yai_rpc_envelope_t env;
        char payload[YAI_MAX_PAYLOAD];

        /* IMPORTANT:
           yai_control_read_frame() returns payload_len.
           payload_len can be 0 and that is VALID.
        */
        ssize_t plen = yai_control_read_frame(cfd, &env, payload, sizeof(payload));

        if (plen < 0) {
            if (plen == YAI_CTL_ERR_OVERFLOW) {
                LOG("[ROOT] Reject overflow payload");
                (void)send_error_response(cfd,
                                          &env,
                                          YAI_E_PAYLOAD_TOO_BIG,
                                          "payload_too_big");
            }
            break;
        }


        /* ---- Frame validation ---- */
        if (env.magic != YAI_FRAME_MAGIC) {
            LOG("[ROOT] Reject bad magic");
            (void)send_error_response(cfd, &env, YAI_E_BAD_MAGIC, "bad_magic");
            break;
        }

        if (env.version != YAI_PROTOCOL_IDS_VERSION) {
            LOG("[ROOT] Reject bad version");
            (void)send_error_response(cfd, &env, YAI_E_BAD_VERSION, "bad_version");
            break;
        }

        if (env.payload_len > YAI_MAX_PAYLOAD) {
            LOG("[ROOT] Reject payload too big");
            (void)send_error_response(cfd,
                                      &env,
                                      YAI_E_PAYLOAD_TOO_BIG,
                                      "payload_too_big");
            break;
        }

        if (env.checksum != 0) {
            LOG("[ROOT] Reject bad checksum=%u", env.checksum);
            (void)send_error_response(cfd,
                                      &env,
                                      YAI_E_BAD_CHECKSUM,
                                      "bad_checksum");
            break;
        }

        if (env.arming > 1) {
            LOG("[ROOT] Reject invalid arming=%u", env.arming);
            (void)send_error_response(cfd,
                                      &env,
                                      YAI_E_ARMING_REQUIRED,
                                      "arming_flag_invalid");
            break;
        }

        if (!is_valid_role(env.role)) {
            LOG("[ROOT] Reject invalid role=%u", env.role);
            (void)send_error_response(cfd,
                                      &env,
                                      YAI_E_ROLE_REQUIRED,
                                      "role_invalid");
            break;
        }

        if (!yai_ws_id_is_valid(env.ws_id)) {
            LOG("[ROOT] Reject bad ws_id");
            (void)send_error_response(cfd, &env, YAI_E_BAD_WS_ID, "bad_ws_id");
            break;
        }

        /* =====================================================
           HANDSHAKE
           ===================================================== */
        if (env.command_id == YAI_CMD_HANDSHAKE) {

            LOG("[ROOT] HANDSHAKE role=%u arming=%u ws='%s'",
                env.role, env.arming, env.ws_id);

            /* Optional strict size check */
            if ((size_t)plen != sizeof(yai_handshake_req_t)) {
                LOG("[ROOT] Bad handshake payload size: %ld", (long)plen);
                (void)send_error_response(cfd,
                                          &env,
                                          YAI_E_PAYLOAD_TOO_BIG,
                                          "bad_handshake_payload_size");
                break;
            }

            yai_handshake_ack_t ack;
            memset(&ack, 0, sizeof(ack));

            ack.server_version       = YAI_PROTOCOL_IDS_VERSION;
            ack.capabilities_granted = 0;
            ack.session_id           = 1;
            ack.status               = (uint8_t)YAI_PROTO_STATE_READY;
            ack._pad                 = 0;

            if (send_response(cfd,
                              &env,
                              YAI_CMD_HANDSHAKE,
                              &ack,
                              (uint32_t)sizeof(ack)) != 0)
                break;

            handshake_done = 1;
            continue;
        }

        /* =====================================================
           REQUIRE HANDSHAKE
           ===================================================== */
        if (!handshake_done) {
            LOG("[ROOT] Command before handshake");
            (void)send_error_response(cfd,
                                      &env,
                                      YAI_E_NEED_HANDSHAKE,
                                      "need_handshake");
            break;
        }

        /* =====================================================
           AUTHORITY ENFORCEMENT
           ===================================================== */
        if (env.role != YAI_ROLE_OPERATOR || !env.arming) {
            LOG("[ROOT] Unauthorized command");
            if (env.role != YAI_ROLE_OPERATOR) {
                (void)send_error_response(cfd,
                                          &env,
                                          YAI_E_ROLE_REQUIRED,
                                          "role_required");
            } else {
                (void)send_error_response(cfd,
                                          &env,
                                          YAI_E_ARMING_REQUIRED,
                                          "arming_required");
            }
            break;
        }

        /* =====================================================
           FORWARD + RELAY (byte-level)
           ===================================================== */
        LOG("[ROOT] FORWARD cmd=%u", env.command_id);
        if (forward_to_kernel_and_relay(cfd,
                                        &env,
                                        payload,
                                        (uint32_t)plen) != 0) {
            break;
        }
    }

    close(cfd);
    LOG("[ROOT] Client disconnected");
}

/* ============================================================
   MAIN
   ============================================================ */

int main(void)
{
    const char *home = getenv("HOME");
    if (!home)
        home = "/tmp";

    log_init(home);

    char sock_path[PATH_MAX];

    snprintf(sock_path, sizeof(sock_path),
             "%s/.yai/run/root/root.sock", home);

    unlink(sock_path);

    int sfd = yai_control_listen_at(sock_path);
    if (sfd < 0) {
        LOG("[ROOT] Failed to bind socket: %s (%s)",
            sock_path, strerror(errno));
        return 1;
    }

    LOG("[ROOT] Listening on %s", sock_path);

    for (;;) {
        int cfd = accept(sfd, NULL, NULL);
        if (cfd >= 0)
            handle_client(cfd);
    }

    return 0;
}
