#define _POSIX_C_SOURCE 200809L

#include <transport.h>
#include <yai_protocol_ids.h>
#include <errors.h>
#include <roles.h>

#include "kernel.h"
#include "yai_kernel.h"
#include "yai_session.h"
#include "control_transport.h"

#include <sys/socket.h>
#include <sys/un.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <stdlib.h>

#ifndef YAI_RUNTIME_BACKLOG
#define YAI_RUNTIME_BACKLOG 8
#endif

static int server_fd = -1;

/* ============================================================
   SOCKET PATH (ENGINE RUNTIME PLANE)
============================================================ */

static int resolve_socket_path(char *out, size_t cap)
{
    if (!out || cap < 32)
        return -1;

    const char *home = getenv("HOME");
    if (!home || !home[0])
        home = "/tmp";

    int n = snprintf(out,
                     cap,
                     "%s/.yai/run/engine/control.sock",
                     home);

    if (n <= 0 || (size_t)n >= cap)
        return -2;

    return 0;
}

static void send_transport_error(int fd,
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
        return;

    yai_rpc_envelope_t resp;
    memset(&resp, 0, sizeof(resp));

    resp.magic      = YAI_FRAME_MAGIC;
    resp.version    = YAI_PROTOCOL_IDS_VERSION;
    resp.command_id = req && req->command_id ? req->command_id : YAI_CMD_CONTROL;
    resp.payload_len = (uint32_t)n;

    if (req) {
        strncpy(resp.ws_id, req->ws_id, sizeof(resp.ws_id) - 1);
        strncpy(resp.trace_id, req->trace_id, sizeof(resp.trace_id) - 1);
        resp.role = req->role;
        resp.arming = req->arming;
    }

    resp.checksum = 0;

    (void)yai_control_write_frame(fd, &resp, payload);
}

static int valid_role(uint16_t role)
{
    return role == YAI_ROLE_NONE ||
           role == YAI_ROLE_USER ||
           role == YAI_ROLE_OPERATOR ||
           role == YAI_ROLE_SYSTEM;
}

/* ============================================================
   HANDLE CLIENT (STRICT BINARY FRAME)
============================================================ */

static void handle_client(int client_fd)
{
    yai_rpc_envelope_t env;
    char payload[YAI_MAX_PAYLOAD];

    ssize_t plen =
        yai_control_read_frame(client_fd,
                               &env,
                               payload,
                               sizeof(payload));

    if (plen < 0) {
        if (plen == YAI_CTL_ERR_OVERFLOW)
            send_transport_error(client_fd, &env, YAI_E_PAYLOAD_TOO_BIG, "payload_too_big");
        close(client_fd);
        return;
    }

    if (env.magic != YAI_FRAME_MAGIC) {
        send_transport_error(client_fd, &env, YAI_E_BAD_MAGIC, "bad_magic");
        close(client_fd);
        return;
    }

    if (env.version != YAI_PROTOCOL_IDS_VERSION) {
        send_transport_error(client_fd, &env, YAI_E_BAD_VERSION, "bad_version");
        close(client_fd);
        return;
    }

    if (env.payload_len > YAI_MAX_PAYLOAD) {
        send_transport_error(client_fd, &env, YAI_E_PAYLOAD_TOO_BIG, "payload_too_big");
        close(client_fd);
        return;
    }

    if (env.checksum != 0) {
        send_transport_error(client_fd, &env, YAI_E_BAD_CHECKSUM, "bad_checksum");
        close(client_fd);
        return;
    }

    if (env.arming > 1) {
        send_transport_error(client_fd, &env, YAI_E_ARMING_REQUIRED, "arming_flag_invalid");
        close(client_fd);
        return;
    }

    if (!valid_role(env.role)) {
        send_transport_error(client_fd, &env, YAI_E_ROLE_REQUIRED, "role_invalid");
        close(client_fd);
        return;
    }

    if (!yai_ws_validate_id(env.ws_id)) {
        send_transport_error(client_fd, &env, YAI_E_BAD_WS_ID, "bad_ws_id");
        close(client_fd);
        return;
    }

    yai_session_dispatch(client_fd, &env, payload);

    close(client_fd);
}

/* ============================================================
   INIT
============================================================ */

int yai_transport_init(void)
{
    char path[256];

    if (resolve_socket_path(path, sizeof(path)) != 0)
        return -1;

    unlink(path);

    server_fd = yai_control_listen_at(path);
    if (server_fd < 0)
        return -2;

    fprintf(stderr,
        "[ENGINE] Runtime Plane online (%s)\n",
        path);

    return 0;
}

/* ============================================================
   SERVE LOOP
============================================================ */

void yai_transport_serve_once(void)
{
    if (server_fd < 0)
        return;

    int client_fd = accept(server_fd, NULL, NULL);

    if (client_fd < 0)
        return;

    handle_client(client_fd);
}
