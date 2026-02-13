// engine/src/transport_client.c  (RPC v1 JSONL client for Kernel control-plane)
#include "../include/transport_client.h"

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

static const char *safe_cstr(const char *s) { return (s && s[0]) ? s : ""; }

static int is_ws_id_safe(const char *ws) {
    // Keep it strict: allow [A-Za-z0-9_-] only, len 1..63
    if (!ws) return 0;
    size_t n = 0;
    for (; ws[n]; n++) {
        unsigned char c = (unsigned char)ws[n];
        if (!( (c >= 'a' && c <= 'z') ||
               (c >= 'A' && c <= 'Z') ||
               (c >= '0' && c <= '9') ||
               c == '_' || c == '-' )) {
            return 0;
        }
        if (n >= 63) return 0;
    }
    return n > 0;
}

static const char *yai_get_home(void) {
    const char *home = getenv("HOME");
    return (home && home[0]) ? home : NULL;
}

static int build_control_sock_path(const char *ws_id, char *out, size_t cap) {
    const char *home = yai_get_home();
    if (!home) return -1;
    if (!is_ws_id_safe(ws_id)) return -2;

    int n = snprintf(out, cap, "%s/.yai/run/%s/control.sock", home, ws_id);
    if (n <= 0 || (size_t)n >= cap) return -3;
    return 0;
}

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
    // Read until '\n' or EOF. Always NUL-terminate.
    if (!out || cap < 2) return -1;
    size_t i = 0;

    while (i + 1 < cap) {
        char c;
        ssize_t r = read(fd, &c, 1);
        if (r == 0) break;                 // EOF
        if (r < 0) {
            if (errno == EINTR) continue;
            return -2;
        }
        out[i++] = c;
        if (c == '\n') break;
    }

    out[i] = '\0';
    if (i == 0) return 0; // empty (EOF)
    return (int)i;
}

// -------- public helpers --------

void yai_make_trace_id(char out[64]) {
    static uint64_t ctr = 0;
    uint64_t t = (uint64_t)time(NULL);
    uint64_t p = (uint64_t)getpid();
    ctr++;
    snprintf(out, 64, "tr-%llx-%llx-%llx",
             (unsigned long long)t,
             (unsigned long long)p,
             (unsigned long long)ctr);
}

int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id) {
    if (!c || !ws_id || !ws_id[0]) return -1;
    memset(c, 0, sizeof(*c));
    c->fd = -1;

    char sock_path[256];
    int prc = build_control_sock_path(ws_id, sock_path, sizeof(sock_path));
    if (prc != 0) return -2;

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -3;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;

    size_t slen = strlen(sock_path);
    if (slen == 0 || slen >= sizeof(addr.sun_path)) {
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
    c->ws_id[sizeof(c->ws_id) - 1] = '\0';
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
    const char *role,            // "user" or "operator"
    const char *request_json,    // JSON object string OR "null"
    char *out_line,
    size_t out_cap
) {
    if (!c || c->fd < 0) return -1;
    if (!request_type || !request_type[0]) return -2;
    if (!out_line || out_cap < 2) return -3;

    const char *ws = safe_cstr(c->ws_id);
    const char *tr = safe_cstr(trace_id);
    const char *rl = safe_cstr(role);

    // payload: must be JSON (object) or "null"
    const char *payload = request_json ? request_json : "null";

    // Envelope v1 JSONL: { v, trace_id, ws_id, arming, role, type, payload }
    char buf[YAI_RPC_LINE_MAX];
    int n = snprintf(
        buf, sizeof(buf),
        "{"
          "\"v\":%d,"
          "\"trace_id\":\"%s\","
          "\"ws_id\":\"%s\","
          "\"arming\":%s,"
          "\"role\":\"%s\","
          "\"type\":\"%s\","
          "\"payload\":%s"
        "}\n",
        YAI_RPC_V1,
        tr,
        ws,
        arming ? "true" : "false",
        rl,
        request_type,
        payload
    );

    if (n <= 0 || (size_t)n >= sizeof(buf)) return -4;

    if (write_all(c->fd, buf, (size_t)n) != 0) return -5;

    int r = read_line(c->fd, out_line, out_cap);
    if (r < 0) return -6;

    return 0;
}

int yai_rpc_handshake(yai_rpc_client_t *c, const char *client_version) {
    if (!c || c->fd < 0) return -1;

    char tr[64];
    yai_make_trace_id(tr);

    // payload is a JSON object. Keep it tiny and explicit.
    // Capabilities is informational; kernel can ignore.
    char payload[384];
    const char *cv = safe_cstr(client_version);
    int n = snprintf(
        payload, sizeof(payload),
        "{"
          "\"client_version\":\"%s\","
          "\"capabilities\":[\"ping\",\"status\"],"
          "\"ws_id\":\"%s\""
        "}",
        cv,
        safe_cstr(c->ws_id)
    );
    if (n <= 0 || (size_t)n >= sizeof(payload)) return -2;

    char resp[YAI_RPC_LINE_MAX];
    int rc = yai_rpc_call(
        c,
        tr,
        "protocol_handshake",
        0,          // engine is not armed
        "user",     // engine is not operator
        payload,
        resp,
        sizeof(resp)
    );
    if (rc != 0) return rc;

    // Phase-2 minimal: we don't parse JSON, we just require a non-empty response line.
    // If you want: detect `"code":` / `"ok":true` here later.
    if (resp[0] == '\0') return -7;
    return 0;
}
