#ifndef YAI_RPC_H
#define YAI_RPC_H

#include <stddef.h>
#include <stdint.h>

#define YAI_RPC_LINE_MAX 8192

typedef struct {
    int fd;
    char ws_id[36];      /* must match envelope */
    uint16_t role;       /* 0=guest,1=operator,2=sovereign */
    uint8_t  arming;     /* 0/1 */
} yai_rpc_client_t;

/* lifecycle */
int  yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id);
void yai_rpc_close(yai_rpc_client_t *c);

/* authority */
void yai_rpc_set_authority(
    yai_rpc_client_t *c,
    int arming,
    const char *role_str
);

/* binary */
int yai_rpc_call_raw(
    yai_rpc_client_t *c,
    uint32_t command_id,
    const void *payload,
    uint32_t payload_len,
    void *out_buf,
    size_t out_cap,
    uint32_t *out_len
);

/* strict handshake */
int yai_rpc_handshake(yai_rpc_client_t *c);

#endif
