#include "../include/yai_rpc.h"
#include "../include/yai_paths.h"

#include <protocol/transport.h>
#include <protocol/yai_protocol_ids.h>
#include <protocol/roles.h>

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

static int read_all(int fd, void *buf, size_t n)
{
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
   VALIDATE WS_ID (CRITICAL)
   ============================================================ */

static int is_valid_ws_id(const char *ws_id)
{
    if (!ws_id || !ws_id[0])
        return 0;

    /* ws_id must NOT contain path separators */
    if (strchr(ws_id, '/'))
        return 0;

    /* forbid absolute paths */
    if (ws_id[0] == '~')
        return 0;

    return 1;
}

/* ============================================================
   CONNECT
   ============================================================ */

int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id)
{
    if (!c)
        return -1;

    if (!is_valid_ws_id(ws_id)) {
        fprintf(stderr,
                "[CLI ERROR] Invalid ws_id passed to yai_rpc_connect: '%s'\n",
                ws_id ? ws_id : "NULL");
        return -99;
    }

    memset(c, 0, sizeof(*c));
    c->fd = -1;

    char sock_path[512];

    /* Root plane always uses root socket */
    if (yai_path_root_sock(sock_path, sizeof(sock_path)) != 0)
        return -2;

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0)
        return -3;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;

    strncpy(addr.sun_path,
            sock_path,
            sizeof(addr.sun_path) - 1);

    socklen_t len =
        offsetof(struct sockaddr_un, sun_path) +
        strlen(addr.sun_path);

    if (connect(fd, (struct sockaddr *)&addr, len) < 0) {
        close(fd);
        return -5;
    }

    c->fd = fd;

    strncpy(c->ws_id,
            ws_id,
            sizeof(c->ws_id) - 1);

    c->ws_id[sizeof(c->ws_id) - 1] = '\0';

    return 0;
}

void yai_rpc_close(yai_rpc_client_t *c)
{
    if (c && c->fd >= 0) {
        close(c->fd);
        c->fd = -1;
    }
}

/* ============================================================
   AUTHORITY
   ============================================================ */

void yai_rpc_set_authority(
    yai_rpc_client_t *c,
    int arming,
    const char *role_str)
{
    if (!c)
        return;

    c->arming = arming ? 1 : 0;

    if (!role_str) {
        c->role = YAI_ROLE_NONE;
        return;
    }

    if (strcmp(role_str, "operator") == 0)
        c->role = YAI_ROLE_OPERATOR;
    else if (strcmp(role_str, "system") == 0)
        c->role = YAI_ROLE_SYSTEM;
    else if (strcmp(role_str, "user") == 0)
        c->role = YAI_ROLE_USER;
    else
        c->role = YAI_ROLE_NONE;
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

    yai_rpc_envelope_t env;
    memset(&env, 0, sizeof(env));

    env.magic       = YAI_FRAME_MAGIC;
    env.version     = YAI_PROTOCOL_IDS_VERSION;
    env.command_id  = command_id;
    env.payload_len = payload_len;
    env.role        = c->role;
    env.arming      = c->arming;
    env.checksum    = 0;

    strncpy(env.ws_id,
            c->ws_id,
            sizeof(env.ws_id) - 1);
    
    fprintf(stderr, "[CLI DEBUG] Sending ws_id='%s'\n", c->ws_id);

    if (write_all(c->fd, &env, sizeof(env)) != 0)
        return -3;

    if (payload_len > 0)
        if (write_all(c->fd, payload, payload_len) != 0)
            return -4;

    yai_rpc_envelope_t resp;

    if (read_all(c->fd, &resp, sizeof(resp)) != 0)
        return -5;

    if (resp.magic != YAI_FRAME_MAGIC)
        return -6;

    if (resp.version != YAI_PROTOCOL_IDS_VERSION)
        return -7;

    if (resp.payload_len > out_cap)
        return -8;

    if (resp.payload_len > 0)
        if (read_all(c->fd, out_buf, resp.payload_len) != 0)
            return -9;

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

    struct {
        uint32_t client_version;
        uint32_t capabilities;
        char     client_name[32];
    } req;

    memset(&req, 0, sizeof(req));

    req.client_version = YAI_PROTOCOL_IDS_VERSION;
    req.capabilities   = 0;
    strncpy(req.client_name, "yai-cli", sizeof(req.client_name) - 1);

    struct {
        uint32_t version;
        uint32_t capabilities;
        uint32_t session_id;
        uint32_t status;
    } ack;

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

    if (ack.version != YAI_PROTOCOL_IDS_VERSION)
        return -21;

    if (ack.status != 1)
        return -22;

    return 0;
}
