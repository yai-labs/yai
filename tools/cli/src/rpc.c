// tools/cli/src/rpc.c
#include "../include/yai_rpc.h"
#include "../include/yai_paths.h"
#include "../include/yai_envelope.h"

#include <protocol/transport.h>
#include <protocol/yai_protocol_ids.h>

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

static const char *safe_cstr(const char *s) { return (s && s[0]) ? s : ""; }

/* ---------------- IO helpers ---------------- */

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
        if (r <= 0) {
            if (r < 0 && errno == EINTR) continue;
            break;
        }
        out[i++] = c;
        if (c == '\n') break;
    }
    out[i] = '\0';
    return (int)i;
}

/* ---------------- protocol helpers ---------------- */

/* Nota: la CLI non calcola ancora checksum (puoi fissarlo a 0 finché kernel/engine non lo validano) */
static void env_init_v1(
    yai_rpc_envelope_t *env,
    const char *ws_id,
    const char *trace_id,
    uint32_t command_id,
    uint16_t role,
    uint8_t arming,
    uint32_t payload_len
) {
    memset(env, 0, sizeof(*env));
    env->magic = YAI_FRAME_MAGIC;
    env->version = 1;

    if (ws_id && ws_id[0]) {
        strncpy(env->ws_id, ws_id, sizeof(env->ws_id) - 1);
    } else {
        /* lascia vuoto: per L1 può essere empty-string */
        env->ws_id[0] = '\0';
    }

    if (trace_id && trace_id[0]) {
        strncpy(env->trace_id, trace_id, sizeof(env->trace_id) - 1);
    } else {
        env->trace_id[0] = '\0';
    }

    env->command_id = command_id;
    env->role = role;
    env->arming = arming ? 1 : 0;
    env->payload_len = payload_len;
    env->checksum = 0; /* TODO: CRC32 envelope+payload quando lo attivi lato kernel */
}

static uint16_t role_to_wire(const char *role_str) {
    /* coerente con transport.h: 0 Guest, 1 Operator, 2 Sovereign */
    if (!role_str || !role_str[0]) return 0;
    if (strcmp(role_str, "op") == 0) return 1;
    if (strcmp(role_str, "operator") == 0) return 1;
    if (strcmp(role_str, "sovereign") == 0) return 2;
    if (strcmp(role_str, "root") == 0) return 2;
    return 0; /* default user/guest */
}

/* mapping deterministico (bin, request_type) -> command_id */
static uint32_t map_command_id(const char *bin, const char *request_type) {
    (void)bin;

    if (!request_type || !request_type[0]) return 0;

    /* control plane */
    if (strcmp(request_type, "handshake") == 0) return YAI_CMD_HANDSHAKE;
    if (strcmp(request_type, "ping") == 0)      return YAI_CMD_PING;

    /* canonical: L1 uses "control" */
    if (strcmp(request_type, "control") == 0)   return YAI_CMD_CONTROL;

    /* L2/L3 RPC */
    if (strcmp(request_type, "storage") == 0)   return YAI_CMD_STORAGE_RPC;
    if (strcmp(request_type, "provider") == 0)  return YAI_CMD_PROVIDER_RPC;
    if (strcmp(request_type, "embedding") == 0) return YAI_CMD_EMBEDDING_RPC;

    /* compat: "storage_put_node", "provider_infer", etc */
    if (strncmp(request_type, "storage_", 8) == 0)   return YAI_CMD_STORAGE_RPC;
    if (strncmp(request_type, "provider_", 9) == 0)  return YAI_CMD_PROVIDER_RPC;
    if (strncmp(request_type, "embedding_", 10) == 0) return YAI_CMD_EMBEDDING_RPC;

    /* fallback: non inventiamo routing “magico” */
    return 0;
}

static const char *strip_prefix(const char *s, const char *prefix) {
    size_t n = strlen(prefix);
    if (strncmp(s, prefix, n) == 0) return s + n;
    return s;
}

/*
  Payload canonico:
    {"method":"X","params":{...}}
  - Se payload_json contiene già "method", lo lasciamo invariato.
  - Se request_type è tipo "storage_put_node" e payload_json è params, wrappiamo.
*/
static int build_canonical_payload(
    uint32_t cmd_id,
    const char *request_type,
    const char *payload_json,
    char *out,
    size_t out_cap
) {
    const char *params = (payload_json && payload_json[0]) ? payload_json : "{}";

    /* handshake/ping/control: pass-through (caller già costruisce canonico se vuole) */
    if (cmd_id == YAI_CMD_HANDSHAKE || cmd_id == YAI_CMD_PING || cmd_id == YAI_CMD_CONTROL) {
        int n = snprintf(out, out_cap, "%s", params);
        return (n > 0 && (size_t)n < out_cap) ? 0 : -1;
    }

    /* se è già wrapper con method */
    if (strstr(params, "\"method\"")) {
        int n = snprintf(out, out_cap, "%s", params);
        return (n > 0 && (size_t)n < out_cap) ? 0 : -1;
    }

    const char *method = request_type ? request_type : "call";

    if (cmd_id == YAI_CMD_STORAGE_RPC) {
        method = strip_prefix(method, "storage_");
        method = strip_prefix(method, "storage.");
    } else if (cmd_id == YAI_CMD_PROVIDER_RPC) {
        method = strip_prefix(method, "provider_");
        method = strip_prefix(method, "provider.");
    } else if (cmd_id == YAI_CMD_EMBEDDING_RPC) {
        method = strip_prefix(method, "embedding_");
        method = strip_prefix(method, "embedding.");
    }

    int n = snprintf(out, out_cap, "{\"method\":\"%s\",\"params\":%s}", safe_cstr(method), params);
    return (n > 0 && (size_t)n < out_cap) ? 0 : -2;
}

/* ---------------- public API ---------------- */

int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id) {
    if (!c) return -1;
    memset(c, 0, sizeof(*c));
    c->fd = -1;

    /* default authority context */
    c->arming = 0;
    c->role = 0;

    char sock_path[512];
    if (yai_path_root_sock(sock_path, sizeof(sock_path)) != 0) return -2;

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -3;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, sock_path, sizeof(addr.sun_path) - 1);

    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -5; /* kernel/root plane offline */
    }

    c->fd = fd;
    if (ws_id && ws_id[0]) strncpy(c->ws_id, ws_id, sizeof(c->ws_id) - 1);
    return 0;
}

void yai_rpc_close(yai_rpc_client_t *c) {
    if (c && c->fd >= 0) {
        close(c->fd);
        c->fd = -1;
    }
}

int yai_rpc_call(
    yai_rpc_client_t *c,
    const char *bin,
    const char *request_type,
    const char *payload_json,
    char *out_line,
    size_t out_cap
) {
    if (!c || c->fd < 0) return -1;
    if (!bin || !bin[0] || !request_type || !request_type[0]) return -2;

    uint32_t cmd_id = map_command_id(bin, request_type);
    if (cmd_id == 0) return -3; /* unknown request_type */

    /* canonical payload */
    char payload[8192];
    payload[0] = '\0';

    if (build_canonical_payload(cmd_id, request_type, payload_json, payload, sizeof(payload)) != 0) {
        return -4;
    }

    uint32_t payload_len = (uint32_t)strlen(payload);
    if (payload_len > YAI_MAX_PAYLOAD) return -5;

    char trace_id[64];
    yai_make_trace_id(trace_id);

    yai_rpc_envelope_t env;
    env_init_v1(
        &env,
        safe_cstr(c->ws_id),
        trace_id,
        cmd_id,
        c->role,
        c->arming,
        payload_len
    );

    if (write_all(c->fd, &env, sizeof(env)) != 0) return -6;
    if (payload_len > 0) {
        if (write_all(c->fd, payload, payload_len) != 0) return -7;
    }

    /* compat: risposta line-based (kernel oggi ti risponde a riga) */
    int r = read_line(c->fd, out_line, out_cap);
    return (r > 0) ? 0 : -8;
}

int yai_rpc_handshake(yai_rpc_client_t *c, const char *bin, const char *cv) {
    if (!c || c->fd < 0) return -1;

    char payload[256];
    int n = snprintf(payload, sizeof(payload),
                     "{\"version\":\"%s\"}",
                     safe_cstr(cv));
    if (n <= 0 || (size_t)n >= sizeof(payload)) return -2;

    char resp[1024];
    /* request_type = "handshake" -> command_id = YAI_CMD_HANDSHAKE */
    return yai_rpc_call(c, safe_cstr(bin), "handshake", payload, resp, sizeof(resp));
}

/* helper opzionale: set authority context from CLI opts */
void yai_rpc_set_authority(yai_rpc_client_t *c, int arming, const char *role_str) {
    if (!c) return;
    c->arming = arming ? 1 : 0;
    c->role = role_to_wire(role_str);
}
