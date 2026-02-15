#include "../include/yai_rpc.h"

#include <protocol/transport.h>
#include <protocol/yai_protocol_ids.h>
#include <protocol/protocol.h>

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <stddef.h>

/* ============================================================
   INTERNAL IO
   ============================================================ */

static int write_all(int fd, const void *buf, size_t n)
{
    const char *p = (const char *)buf;
    size_t off = 0;

    while (off < n)
    {
        ssize_t w = write(fd, p + off, n - off);
        if (w < 0)
        {
            if (errno == EINTR) continue;
            return -1;
        }
        off += (size_t)w;
    }

    return 0;
}

static int read_all(int fd, void *buf, size_t n)
{
    char *p = (char *)buf;
    size_t off = 0;

    while (off < n)
    {
        ssize_t r = read(fd, p + off, n - off);
        if (r <= 0)
        {
            if (r < 0 && errno == EINTR) continue;
            return -1;
        }
        off += (size_t)r;
    }

    return 0;
}

/* ============================================================
   ROLE MAPPING
   ============================================================ */

static uint16_t role_to_wire(const char *role)
{
    if (!role) return 0;

    if (strcmp(role, "operator") == 0)
        return 1;

    if (strcmp(role, "sovereign") == 0)
        return 2;

    return 0; /* guest */
}

/* ============================================================
   CONNECT
   ============================================================ */

int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id)
{
    if (!c) return -1;

    memset(c, 0, sizeof(*c));
    c->fd = -1;
    c->role = 0;
    c->arming = 0;

    const char *home = getenv("HOME");
    if (!home) return -2;

    char path[512];
    snprintf(path,
             sizeof(path),
             "%s/.yai/run/default/control.sock",
             home);

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0)
        return -3;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;

    if (strlen(path) >= sizeof(addr.sun_path))
    {
        close(fd);
        return -4;
    }

    strncpy(addr.sun_path, path, sizeof(addr.sun_path) - 1);

    socklen_t len =
        offsetof(struct sockaddr_un, sun_path) +
        strlen(addr.sun_path);

    if (connect(fd, (struct sockaddr *)&addr, len) < 0)
    {
        close(fd);
        return -5;
    }

    c->fd = fd;

    strncpy(c->ws_id,
            ws_id && ws_id[0] ? ws_id : "default",
            sizeof(c->ws_id) - 1);

    return 0;
}

void yai_rpc_close(yai_rpc_client_t *c)
{
    if (c && c->fd >= 0)
    {
        close(c->fd);
        c->fd = -1;
    }
}

/* ============================================================
   AUTHORITY (REAL IMPLEMENTATION)
   ============================================================ */

void yai_rpc_set_authority(
    yai_rpc_client_t *c,
    int arming,
    const char *role_str)
{
    if (!c) return;

    c->arming = arming ? 1 : 0;
    c->role   = role_to_wire(role_str);
}

/* ============================================================
   RAW CALL
   ============================================================ */

int yai_rpc_call_raw(
    yai_rpc_client_t *c,
    uint32_t command_id,
    const void *payload,
    uint32_t payload_len,
    void *out_buf,
    size_t out_cap,
    uint32_t *out_len)
{
    if (!c || c->fd < 0)
        return -1;

    if (payload_len > YAI_MAX_PAYLOAD)
        return -2;

    yai_rpc_envelope_t env;
    memset(&env, 0, sizeof(env));

    env.magic       = YAI_FRAME_MAGIC;
    env.version     = YAI_PROTOCOL_IDS_VERSION;
    env.command_id  = command_id;
    env.payload_len = payload_len;

    env.role   = c->role;
    env.arming = c->arming;

    strncpy(env.ws_id,
            c->ws_id,
            sizeof(env.ws_id) - 1);

    /* ---- WRITE ENVELOPE ---- */

    if (write_all(c->fd, &env, sizeof(env)) != 0)
        return -3;

    /* ---- WRITE PAYLOAD ---- */

    if (payload_len > 0)
    {
        if (write_all(c->fd, payload, payload_len) != 0)
            return -4;
    }

    /* ---- READ RESPONSE ENVELOPE ---- */

    yai_rpc_envelope_t resp;

    if (read_all(c->fd, &resp, sizeof(resp)) != 0)
        return -5;

    if (resp.magic != YAI_FRAME_MAGIC)
        return -6;

    if (resp.version != YAI_PROTOCOL_IDS_VERSION)
        return -7;

    if (resp.payload_len > YAI_MAX_PAYLOAD)
        return -8;

    if (resp.payload_len > out_cap)
        return -9;

    /* ---- READ RESPONSE PAYLOAD ---- */

    if (resp.payload_len > 0)
    {
        if (read_all(c->fd, out_buf, resp.payload_len) != 0)
            return -10;
    }

    if (out_len)
        *out_len = resp.payload_len;

    return 0;
}

/* ============================================================
   HANDSHAKE
   ============================================================ */

int yai_rpc_handshake(yai_rpc_client_t *c)
{
    if (!c || c->fd < 0)
        return -1;

    yai_handshake_req_t req;
    memset(&req, 0, sizeof(req));

    req.client_version         = YAI_PROTOCOL_IDS_VERSION;
    req.capabilities_requested = 0;

    strncpy(req.client_name,
            "yai-cli",
            sizeof(req.client_name) - 1);

    yai_handshake_ack_t ack;
    uint32_t out_len = 0;

    int rc = yai_rpc_call_raw(
        c,
        YAI_CMD_HANDSHAKE,
        &req,
        sizeof(req),
        &ack,
        sizeof(ack),
        &out_len);

    if (rc != 0)
        return rc;

    if (out_len != sizeof(ack))
        return -20;

    if (ack.server_version != YAI_PROTOCOL_IDS_VERSION)
        return -21;

    if (ack.status != YAI_PROTO_STATE_READY)
        return -22;

    return 0;
}
