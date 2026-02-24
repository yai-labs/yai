#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/socket.h>

#include "yai_kernel.h"
#include "control_transport.h"
#include "yai_session.h"

#include <transport.h>
#include <yai_protocol_ids.h>
#include <protocol.h>
#include <errors.h>
#include <roles.h>

#define YAI_BINARY_PAYLOAD_MAX 65536

/* ============================================================
   Internal: send a binary frame (envelope + payload)
============================================================ */
static void send_frame(
    int fd,
    const yai_rpc_envelope_t *req,
    uint32_t command_id,
    const void *payload,
    uint32_t payload_len
) {
    yai_rpc_envelope_t resp;
    memset(&resp, 0, sizeof(resp));

    resp.magic       = YAI_FRAME_MAGIC;
    resp.version     = YAI_PROTOCOL_IDS_VERSION;
    resp.command_id  = command_id;
    resp.payload_len = payload_len;

    strncpy(resp.ws_id, req->ws_id, sizeof(resp.ws_id) - 1);
    strncpy(resp.trace_id, req->trace_id, sizeof(resp.trace_id) - 1);

    resp.role      = req->role;
    resp.arming    = req->arming;
    resp.checksum  = 0;

    yai_control_write_frame(fd, &resp, payload);
}

static void send_error(
    int fd,
    const yai_rpc_envelope_t *req,
    uint32_t code,
    const char *reason
) {
    char payload[256];
    int n = snprintf(payload,
                     sizeof(payload),
                     "{\"status\":\"error\",\"code\":%u,\"reason\":\"%s\"}",
                     code,
                     reason ? reason : "unknown");
    if (n <= 0 || (size_t)n >= sizeof(payload))
        return;

    yai_rpc_envelope_t safe_req;
    memset(&safe_req, 0, sizeof(safe_req));
    if (req)
        safe_req = *req;

    send_frame(fd,
               &safe_req,
               safe_req.command_id ? safe_req.command_id : YAI_CMD_CONTROL,
               payload,
               (uint32_t)n);
}

static int valid_role(uint16_t role)
{
    return role == YAI_ROLE_NONE ||
           role == YAI_ROLE_USER ||
           role == YAI_ROLE_OPERATOR ||
           role == YAI_ROLE_SYSTEM;
}

/* ============================================================
   Binary connection handler (one-shot, Root or WS)
============================================================ */
void yai_kernel_handle_binary_connection(int cfd)
{
    yai_rpc_envelope_t env;
    char payload[YAI_BINARY_PAYLOAD_MAX + 1];

    ssize_t r = yai_control_read_frame(
        cfd,
        &env,
        payload,
        sizeof(payload) - 1
    );

    if (r < 0) {
        if (r == YAI_CTL_ERR_OVERFLOW)
            send_error(cfd, &env, YAI_E_PAYLOAD_TOO_BIG, "payload_too_big");
        close(cfd);
        return;
    }

    printf("[KERNEL] RECV cmd=%u len=%u role=%u arming=%u\n",
           env.command_id,
           env.payload_len,
           env.role,
           env.arming);

    /* -------- Strict protocol validation -------- */
    if (env.magic != YAI_FRAME_MAGIC) {
        send_error(cfd, &env, YAI_E_BAD_MAGIC, "bad_magic");
        close(cfd);
        return;
    }

    if (env.version != YAI_PROTOCOL_IDS_VERSION) {
        send_error(cfd, &env, YAI_E_BAD_VERSION, "bad_version");
        close(cfd);
        return;
    }

    if (env.payload_len > YAI_MAX_PAYLOAD) {
        send_error(cfd, &env, YAI_E_PAYLOAD_TOO_BIG, "payload_too_big");
        close(cfd);
        return;
    }

    if (env.checksum != 0) {
        send_error(cfd, &env, YAI_E_BAD_CHECKSUM, "bad_checksum");
        close(cfd);
        return;
    }

    if (env.arming > 1) {
        send_error(cfd, &env, YAI_E_ARMING_REQUIRED, "arming_flag_invalid");
        close(cfd);
        return;
    }

    if (!valid_role(env.role)) {
        send_error(cfd, &env, YAI_E_ROLE_REQUIRED, "role_invalid");
        close(cfd);
        return;
    }

    if (!yai_ws_validate_id(env.ws_id)) {
        send_error(cfd, &env, YAI_E_BAD_WS_ID, "bad_ws_id");
        close(cfd);
        return;
    }

    /* -------- HANDSHAKE -------- */
    if (env.command_id == YAI_CMD_HANDSHAKE) {

        if ((size_t)r != sizeof(yai_handshake_req_t)) {
            send_error(cfd, &env, YAI_E_PAYLOAD_TOO_BIG, "bad_handshake_payload_size");
            close(cfd);
            return;
        }

        yai_handshake_req_t *req = (yai_handshake_req_t *)payload;
        yai_handshake_ack_t ack;
        memset(&ack, 0, sizeof(ack));

        ack.server_version       = YAI_PROTOCOL_IDS_VERSION;
        ack.capabilities_granted = req->capabilities_requested;
        ack.session_id           = 1;
        ack.status               = YAI_PROTO_STATE_READY;

        send_frame(cfd, &env, YAI_CMD_HANDSHAKE, &ack, sizeof(ack));
        printf("[KERNEL] Handshake OK\n");
        close(cfd);
        return;
    }

    /* -------- Require authority for non-handshake calls -------- */
    if (env.role != YAI_ROLE_OPERATOR) {
        send_error(cfd, &env, YAI_E_ROLE_REQUIRED, "role_required");
        close(cfd);
        return;
    }

    if (!env.arming) {
        send_error(cfd, &env, YAI_E_ARMING_REQUIRED, "arming_required");
        close(cfd);
        return;
    }

    /* -------- PING / Root Status -------- */
    if (env.command_id == YAI_CMD_PING) {

        const char *pong;
        if (strcmp(env.ws_id, "root") == 0) {
            pong = "{\"status\":\"pong\",\"plane\":\"root\"}";
        } else {
            pong = "{\"status\":\"pong\"}";
        }

        send_frame(cfd, &env, YAI_CMD_PING, pong, (uint32_t)strlen(pong));
        printf("[KERNEL] Pong sent\n");
        close(cfd);
        return;
    }

    /* -------- DEFAULT RESPONSE -------- */
    const char *ok = "{\"status\":\"ok\"}";
    send_frame(cfd, &env, env.command_id, ok, (uint32_t)strlen(ok));
    close(cfd);
}
