#include "../../include/transport_client.h"

#include "protocol.h"
#include "transport.h"
#include "yai_protocol_ids.h"

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

/* ============================================================
   IO HELPERS (IDENTICI ALLA CLI)
   ============================================================ */

static int write_all(int fd, const void *buf, size_t n) {
    const char *p = buf;
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

static int read_all(int fd, void *buf, size_t n) {
    char *p = buf;
    size_t off = 0;

    while (off < n) {
        ssize_t r = read(fd, p + off, n - off);
        if (r <= 0) {
            if (r < 0 && errno == EINTR) continue;
            return -1;
        }
        off += (size_t)r;
    }
    return 0;
}

/* ============================================================
   TRACE
   ============================================================ */

void yai_make_trace_id(char out[36]) {
    static uint32_t ctr = 0;
    snprintf(out, 36, "tr-%lx-%u", (unsigned long)time(NULL), ctr++);
}

/* ============================================================
   CONNECT
   ============================================================ */

static int build_control_sock_path(const char *ws_id, char *out, size_t cap) {
    const char *home = getenv("HOME");
    if (!home) return -1;
    snprintf(out, cap, "%s/.yai/run/%s/control.sock", home, ws_id);
    return 0;
}

int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id) {
    if (!c || !ws_id) return -1;

    memset(c, 0, sizeof(*c));

    char sock_path[256];
    if (build_control_sock_path(ws_id, sock_path, sizeof(sock_path)) < 0)
        return -2;

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0)
        return -3;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, sock_path, sizeof(addr.sun_path) - 1);

    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -4;
    }

    c->fd = fd;
    strncpy(c->ws_id, ws_id, sizeof(c->ws_id) - 1);
    c->connected = true;

    return 0;
}

/* ============================================================
   HARD BINARY RPC
   ============================================================ */

int yai_rpc_call(
    yai_rpc_client_t *c,
    uint32_t command_id,
    const void *payload,
    uint32_t payload_len,
    void *out_buf,
    uint32_t out_cap,
    uint32_t *out_len
) {
    if (!c || !c->connected) return -1;

    if (payload_len > YAI_MAX_PAYLOAD)
        return -2;

    yai_rpc_envelope_t env;
    memset(&env, 0, sizeof(env));

    env.magic   = YAI_FRAME_MAGIC;
    env.version = YAI_PROTOCOL_VERSION;
    env.command_id  = command_id;
    env.payload_len = payload_len;

    strncpy(env.ws_id, c->ws_id, sizeof(env.ws_id) - 1);
    yai_make_trace_id(env.trace_id);

    if (write_all(c->fd, &env, sizeof(env)) != 0)
        return -3;

    if (payload_len > 0 && payload) {
        if (write_all(c->fd, payload, payload_len) != 0)
            return -4;
    }

    /* --- READ RESPONSE ENVELOPE --- */

    yai_rpc_envelope_t resp;
    if (read_all(c->fd, &resp, sizeof(resp)) != 0)
        return -5;

    if (resp.magic != YAI_FRAME_MAGIC)
        return -6;

    if (resp.version != YAI_PROTOCOL_VERSION)
        return -7;

    if (resp.payload_len > out_cap)
        return -8;

    if (resp.payload_len > 0 && out_buf) {
        if (read_all(c->fd, out_buf, resp.payload_len) != 0)
            return -9;
    }

    if (out_len)
        *out_len = resp.payload_len;

    return 0;
}

/* ============================================================
   HANDSHAKE (BINARIO VERO)
   ============================================================ */
int yai_rpc_handshake(yai_rpc_client_t *c, uint32_t capabilities)
{
    if (!c || !c->connected)
        return -1;

    yai_handshake_req_t hs;
    memset(&hs, 0, sizeof(hs));

    hs.client_version = YAI_PROTOCOL_VERSION;
    hs.capabilities_requested = capabilities;
    strncpy(hs.client_name, "yai-engine", sizeof(hs.client_name) - 1);

    uint8_t resp_buf[64];
    uint32_t resp_len = 0;

    int rc = yai_rpc_call(
        c,
        YAI_CMD_HANDSHAKE,
        &hs,
        sizeof(hs),
        resp_buf,
        sizeof(resp_buf),
        &resp_len
    );

    if (rc != 0)
        return rc;

    if (resp_len != sizeof(yai_handshake_ack_t))
        return -2;

    yai_handshake_ack_t *ack = (yai_handshake_ack_t *)resp_buf;

    if (ack->status != YAI_PROTO_STATE_READY)
        return -3;

    return 0;
}



void yai_rpc_close(yai_rpc_client_t *c) {
    if (c && c->connected) {
        close(c->fd);
        c->connected = false;
    }
}
