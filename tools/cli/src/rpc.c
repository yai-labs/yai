// tools/cli/src/rpc.c
#include "../include/yai_rpc.h"
#include "../include/yai_paths.h"
#include "../include/yai_envelope.h"

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

#include <errno.h>
#include <string.h>
#include <stdio.h>

static const char *safe_cstr(const char *s) { return (s && s[0]) ? s : ""; }

static int write_all(int fd, const void *buf, size_t n) {
    const char *p = (const char *)buf;
    size_t off = 0;
    while (off < n) {
        ssize_t w = write(fd, p + off, n - off);
        if (w < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        off += (size_t)w;
    }
    return 0;
}

static int read_line(int fd, char *out, size_t cap) {
    if (!out || cap < 2) return -1;
    size_t i = 0;
    while (i + 1 < cap) {
        char c;
        ssize_t r = read(fd, &c, 1);
        if (r == 0) break;
        if (r < 0) {
            if (errno == EINTR) continue;
            return -2;
        }
        out[i++] = c;
        if (c == '\n') break;
    }
    out[i] = '\0';
    return (int)i;
}

int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id) {
    if (!c || !ws_id || !ws_id[0]) return -1;
    memset(c, 0, sizeof(*c));
    c->fd = -1;

    char sock_path[256];
    if (yai_path_control_sock(ws_id, sock_path, sizeof(sock_path)) != 0) return -2;

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -3;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;

    if (strlen(sock_path) >= sizeof(addr.sun_path)) {
        close(fd);
        return -4;
    }
    strncpy(addr.sun_path, sock_path, sizeof(addr.sun_path) - 1);

    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -5;
    }

    c->fd = fd;
    strncpy(c->ws_id, ws_id, sizeof(c->ws_id) - 1);
    return 0;
}

void yai_rpc_close(yai_rpc_client_t *c) {
    if (!c) return;
    if (c->fd >= 0) close(c->fd);
    c->fd = -1;
}

int yai_rpc_call(
    yai_rpc_client_t *c,
    const char *trace_id,
    const char *request_type,
    int arming,
    const char *role,
    const char *payload_json,
    char *out_line,
    size_t out_cap
) {
    if (!c || c->fd < 0) return -1;
    if (!request_type || !request_type[0]) return -2;
    if (!out_line || out_cap < 16) return -3;

    char tr[64];
    if (!trace_id || !trace_id[0]) {
        yai_make_trace_id(tr);
        trace_id = tr;
    }

    char buf[YAI_RPC_LINE_MAX];
    int rc = yai_build_request_jsonl(
        safe_cstr(c->ws_id),
        trace_id,
        arming,
        safe_cstr(role),
        request_type,
        payload_json,
        buf,
        sizeof(buf)
    );
    if (rc != 0) return -4;

    if (write_all(c->fd, buf, strlen(buf)) != 0) return -5;

    int r = read_line(c->fd, out_line, out_cap);
    if (r < 0) return -6;

    return 0;
}

int yai_rpc_handshake(yai_rpc_client_t *c, const char *client_version) {
    if (!c || c->fd < 0) return -1;

    char tr[64];
    yai_make_trace_id(tr);

    char payload[256];
    snprintf(
        payload,
        sizeof(payload),
        "{\"client_version\":\"%s\",\"capabilities\":[\"rpc.v1\",\"events.stream\"]}",
        safe_cstr(client_version)
    );

    char line[YAI_RPC_LINE_MAX];
    return yai_rpc_call(c, tr, "protocol_handshake", 0, "user", payload, line, sizeof(line));
}
