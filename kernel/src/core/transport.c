#include <protocol/transport.h>
#include <protocol/yai_protocol_ids.h>

#include "kernel.h"
#include "yai_kernel.h"
#include "yai_session.h"

#include <sys/socket.h>
#include <sys/un.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>
#include <stdlib.h>

#ifndef YAI_RUNTIME_BACKLOG
#define YAI_RUNTIME_BACKLOG 8
#endif

#define YAI_LEVEL_WARN "warn"
#define DEFAULT_WS "default"

static int server_fd = -1;

/* ============================================================
   SOCKET PATH RESOLUTION (UNIFICATA)
   ============================================================ */

static const char *resolve_socket_path(void) {

    const char *env = getenv("YAI_RUNTIME_SOCKET");
    if (env && env[0] != '\0') {
        return env;
    }

    static char path[256];
    const char *home = getenv("HOME");

    if (!home) {
        return "/tmp/yai_fallback.sock";
    }

    snprintf(path, sizeof(path),
             "%s/.yai/run/%s/control.sock",
             home, DEFAULT_WS);

    return path;
}

/* ============================================================
   IO UTILS
   ============================================================ */

static ssize_t read_full(int fd, void *buf, size_t n) {
    size_t got = 0;

    while (got < n) {
        ssize_t r = read(fd, (char *)buf + got, n - got);
        if (r == 0) return (ssize_t)got;
        if (r < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        got += (size_t)r;
    }

    return (ssize_t)got;
}

/* ============================================================
   LOGGING
   ============================================================ */

static void log_transport_event(const char *event_type,
                                const char *ws_id,
                                const char *trace_id,
                                const char *reason) {

    char msg[192];

    snprintf(msg, sizeof(msg),
             "TRANSPORT_%s reason=%s actor=kernel",
             event_type,
             reason);

    yai_log_static(EV_TRANSITION_REJECTED,
                   (ws_id && ws_id[0]) ? ws_id : "system",
                   (trace_id && trace_id[0]) ? trace_id : "null",
                   YAI_LEVEL_WARN,
                   msg,
                   "null");
}

/* ============================================================
   RESPONSE WRITER
   ============================================================ */

static void write_ok_response(int fd,
                              const yai_rpc_envelope_t *req,
                              uint32_t cmd) {

    const char *payload = "{\"status\":\"ok\"}";

    yai_rpc_envelope_t resp;
    memset(&resp, 0, sizeof(resp));

    resp.magic   = YAI_FRAME_MAGIC;
    resp.version = YAI_PROTOCOL_IDS_VERSION;

    strncpy(resp.ws_id, req->ws_id, sizeof(resp.ws_id) - 1);
    strncpy(resp.trace_id, req->trace_id, sizeof(resp.trace_id) - 1);

    resp.command_id  = cmd;
    resp.role        = 0;
    resp.arming      = 0;
    resp.payload_len = (uint32_t)strlen(payload);
    resp.checksum    = 0;

    write(fd, &resp, sizeof(resp));
    write(fd, payload, resp.payload_len);
}

/* ============================================================
   COMMAND HANDLER
   ============================================================ */

static void handle_client_command(int client_fd) {

    yai_rpc_envelope_t env;
    memset(&env, 0, sizeof(env));

    if (read_full(client_fd, &env, sizeof(env)) != sizeof(env)) {
        close(client_fd);
        return;
    }

    if (env.magic != YAI_FRAME_MAGIC) {
        log_transport_event("REJECTED", "null", "null", "bad_magic");
        close(client_fd);
        return;
    }

    if (env.version != YAI_PROTOCOL_IDS_VERSION) {
        log_transport_event("REJECTED", env.ws_id, env.trace_id, "bad_version");
        close(client_fd);
        return;
    }

    if (env.payload_len > YAI_MAX_PAYLOAD) {
        log_transport_event("REJECTED", env.ws_id, env.trace_id, "oversize_payload");
        close(client_fd);
        return;
    }

    char *payload = NULL;

    if (env.payload_len > 0) {
        payload = malloc(env.payload_len + 1);
        if (!payload) {
            close(client_fd);
            return;
        }

        if (read_full(client_fd, payload, env.payload_len) != (ssize_t)env.payload_len) {
            free(payload);
            close(client_fd);
            return;
        }

        payload[env.payload_len] = '\0';
    }

    switch (env.command_id) {

        case YAI_CMD_HANDSHAKE:
            write_ok_response(client_fd, &env, YAI_CMD_HANDSHAKE);
            break;

        case YAI_CMD_PING:
            write_ok_response(client_fd, &env, YAI_CMD_PING);
            break;

        default:
            write_ok_response(client_fd, &env, env.command_id);
            break;
    }

    if (payload) free(payload);
    close(client_fd);
}

/* ============================================================
   INIT
   ============================================================ */

int yai_transport_init(void) {

    struct sockaddr_un addr;
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -1;

    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;

    const char *path = resolve_socket_path();

    strncpy(addr.sun_path, path, sizeof(addr.sun_path) - 1);

    unlink(path);

    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("[TRANSPORT] bind failed");
        close(fd);
        return -2;
    }

    chmod(path, 0600);

    if (listen(fd, YAI_RUNTIME_BACKLOG) < 0) {
        close(fd);
        return -3;
    }

    server_fd = fd;

    fprintf(stderr,
            "[TRANSPORT] Root Plane UDS Ready: %s\n",
            path);

    return 0;
}

/* ============================================================
   SERVE LOOP
   ============================================================ */

void yai_transport_serve_once(void) {

    if (server_fd < 0) return;

    int client_fd = accept(server_fd, NULL, NULL);

    if (client_fd >= 0) {
        handle_client_command(client_fd);
    }
}
