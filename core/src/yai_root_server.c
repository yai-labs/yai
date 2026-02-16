#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <limits.h>
#include <time.h>
#include <sys/socket.h>
#include <sys/un.h>

#include <protocol/transport.h>
#include <protocol/yai_protocol_ids.h>
#include <protocol/errors.h>
#include <protocol/roles.h>

#include "control_transport.h"

#define YAI_STATE_READY 1
#define YAI_CAP_NONE    0

static FILE *root_log = NULL;

/* ============================================================
   LOGGING
   ============================================================ */

static void log_init(const char *home)
{
    char log_path[PATH_MAX];

    snprintf(log_path, sizeof(log_path),
         "%s/.yai/run/root.log", home);


    root_log = fopen(log_path, "a");

    if (!root_log) {
        fprintf(stderr, "[ROOT] Failed to open log file\n");
        root_log = stderr;
    } else {
        setvbuf(root_log, NULL, _IOLBF, 0);
    }

    time_t now = time(NULL);
    fprintf(root_log, "\n=== ROOT START %ld ===\n", now);
    fflush(root_log);
}

#define LOG(fmt, ...)                                   \
    do {                                                \
        fprintf(stdout, fmt "\n", ##__VA_ARGS__);       \
        if (root_log)                                   \
            fprintf(root_log, fmt "\n", ##__VA_ARGS__); \
    } while (0)

/* ============================================================
   SEND FRAME
   ============================================================ */

static void send_frame(int fd,
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

    strncpy(resp.ws_id, req->ws_id, sizeof(resp.ws_id) - 1);
    strncpy(resp.trace_id, req->trace_id, sizeof(resp.trace_id) - 1);

    yai_control_write_frame(fd, &resp, payload);
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

        ssize_t r = yai_control_read_frame(
            cfd, &env, payload, sizeof(payload));

        if (r < 0)
            break;

        /* ----------------------------------------------------
           Structured logging (clear separation handshake/cmd)
           ---------------------------------------------------- */
        if (env.command_id == YAI_CMD_HANDSHAKE)
            LOG("[ROOT] HANDSHAKE role=%u arming=%u ws='%s'",
                env.role,
                env.arming,
                env.ws_id);
        else
            LOG("[ROOT] CMD=%u role=%u arming=%u ws='%s'",
                env.command_id,
                env.role,
                env.arming,
                env.ws_id);

        /* ----------------------------------------------------
           Protocol validation
           ---------------------------------------------------- */
        if (env.magic != YAI_FRAME_MAGIC ||
            env.version != YAI_PROTOCOL_IDS_VERSION) {
            LOG("[ROOT] Invalid magic/version");
            break;
        }

        /* Basic ws sanity */
        if (strchr(env.ws_id, '/')) {
            LOG("[ROOT] Invalid ws_id");
            break;
        }

        /* ----------------------------------------------------
           HANDSHAKE
           ---------------------------------------------------- */
        if (env.command_id == YAI_CMD_HANDSHAKE) {

            struct {
                uint32_t version;
                uint32_t capabilities;
                uint32_t session_id;
                uint32_t status;
            } ack;

            memset(&ack, 0, sizeof(ack));
            ack.version      = YAI_PROTOCOL_IDS_VERSION;
            ack.capabilities = YAI_CAP_NONE;
            ack.session_id   = 1;
            ack.status       = YAI_STATE_READY;

            send_frame(cfd,
                       &env,
                       YAI_CMD_HANDSHAKE,
                       &ack,
                       sizeof(ack));

            handshake_done = 1;
            continue;
        }

        /* ----------------------------------------------------
           Require handshake before any command
           ---------------------------------------------------- */
        if (!handshake_done) {
            LOG("[ROOT] Command before handshake");
            break;
        }

        /* ----------------------------------------------------
           Authority enforcement
           ---------------------------------------------------- */
        if (env.role != YAI_ROLE_OPERATOR || !env.arming) {
            LOG("[ROOT] Unauthorized command");
            break;
        }

        /* ----------------------------------------------------
           PING
           ---------------------------------------------------- */
        if (env.command_id == YAI_CMD_PING) {
            const char *pong = "{\"pong\":true}";
            send_frame(cfd,
                       &env,
                       YAI_CMD_PING,
                       pong,
                       (uint32_t)strlen(pong));
            continue;
        }

        /* ----------------------------------------------------
           Default stub response
           ---------------------------------------------------- */
        const char *ok = "{\"status\":\"ok\"}";
        send_frame(cfd,
                   &env,
                   env.command_id,
                   ok,
                   (uint32_t)strlen(ok));
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
    if (!home) home = "/tmp";

    log_init(home);

    char sock_path[PATH_MAX];
    snprintf(sock_path, sizeof(sock_path),
             "%s/.yai/run/root.sock", home);

    int sfd = yai_control_listen_at(sock_path);
    if (sfd < 0) {
        LOG("[ROOT] Failed to bind root socket");
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
