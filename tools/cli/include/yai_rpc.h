// tools/cli/include/yai_rpc.h
#ifndef YAI_RPC_H
#define YAI_RPC_H

#include <stddef.h>

#ifndef YAI_RPC_LINE_MAX
#define YAI_RPC_LINE_MAX 4096
#endif

typedef struct {
    int fd;
    char ws_id[64];
} yai_rpc_client_t;

// connect to ~/.yai/run/<ws>/control.sock
int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id);

// close
void yai_rpc_close(yai_rpc_client_t *c);

// handshake must be first (kernel enforces)
int yai_rpc_handshake(yai_rpc_client_t *c, const char *client_version);

// call generic request (returns 0 ok, out_line contains 1 json line)
int yai_rpc_call(
    yai_rpc_client_t *c,
    const char *trace_id,
    const char *request_type,
    int arming,
    const char *role,
    const char *payload_json,
    char *out_line,
    size_t out_cap
);

#endif
