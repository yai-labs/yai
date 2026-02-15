#ifndef YAI_TRANSPORT_CLIENT_H
#define YAI_TRANSPORT_CLIENT_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "../../law/specs/protocol/transport.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int fd;
    char ws_id[36];
    uint32_t session_id;
    bool connected;
} yai_rpc_client_t;

/* Connection */
int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id);
void yai_rpc_close(yai_rpc_client_t *c);

/* Handshake binario reale */
int yai_rpc_handshake(yai_rpc_client_t *c, uint32_t capabilities);

/* Hard binary RPC */
int yai_rpc_call(
    yai_rpc_client_t *c,
    uint32_t command_id,
    const void *payload,
    uint32_t payload_len,
    void *out_buf,
    uint32_t out_cap,
    uint32_t *out_len
);

/* Trace helper */
void yai_make_trace_id(char out[36]);

#ifdef __cplusplus
}
#endif

#endif
