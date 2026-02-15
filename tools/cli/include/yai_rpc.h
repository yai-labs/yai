#ifndef YAI_RPC_H
#define YAI_RPC_H

#include <stddef.h>
#include <stdint.h>

#define YAI_RPC_LINE_MAX 4096

typedef struct {
    int fd;
    char ws_id[64];

    /* ADR-004: authority context travels in the envelope */
    uint16_t role;   /* 0 guest, 1 operator, 2 sovereign */
    uint8_t  arming; /* 0/1 */
} yai_rpc_client_t;

int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id);
void yai_rpc_close(yai_rpc_client_t *c);

/* helper: set role/arming on the client context */
void yai_rpc_set_authority(yai_rpc_client_t *c, int arming, const char *role_str);

/* MUST be 3 args */
int yai_rpc_handshake(yai_rpc_client_t *c, const char *bin, const char *cv);

int yai_rpc_call(
    yai_rpc_client_t *c,
    const char *bin,
    const char *request_type,
    const char *payload_json,
    char *out_line,
    size_t out_cap
);

#endif
