#pragma once

#include <stdbool.h>
#include <stdint.h>

#include <yai/protocol/transport_contract.h>

typedef struct yai_rpc_client {
  int fd;
  char ws_id[36];
  uint32_t session_id;
  bool connected;
} yai_rpc_client_t;

int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id);
void yai_rpc_close(yai_rpc_client_t *c);
int yai_rpc_handshake(yai_rpc_client_t *c, uint32_t capabilities);
int yai_rpc_call(yai_rpc_client_t *c,
                 uint32_t command_id,
                 const void *payload,
                 uint32_t payload_len,
                 void *out_buf,
                 uint32_t out_cap,
                 uint32_t *out_len);
void yai_make_trace_id(char out[36]);
