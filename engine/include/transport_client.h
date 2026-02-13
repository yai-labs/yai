// engine/include/transport_client.h  (RPC v1 JSONL client for Kernel control-plane)
#ifndef YAI_TRANSPORT_CLIENT_H
#define YAI_TRANSPORT_CLIENT_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// RPC v1
#ifndef YAI_RPC_V1
#define YAI_RPC_V1 1
#endif

#ifndef YAI_RPC_LINE_MAX
#define YAI_RPC_LINE_MAX 4096
#endif

typedef struct {
    int fd;
    char ws_id[64]; // strict, copied from connect()
} yai_rpc_client_t;

// Connect to kernel control.sock for a workspace (~/.yai/run/<ws>/control.sock)
int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id);

// Close connection
void yai_rpc_close(yai_rpc_client_t *c);

// Handshake MUST be first request on a connection (kernel-side enforced)
int yai_rpc_handshake(yai_rpc_client_t *c, const char *client_version);

// Call a request type (ping/status/...) with JSON payload (object) or "null".
// Returns 0 on success and writes one JSONL response line in out_line.
int yai_rpc_call(
    yai_rpc_client_t *c,
    const char *trace_id,
    const char *request_type,
    int arming,
    const char *role,            // "user" or "operator"
    const char *request_json,    // JSON object string OR "null"
    char *out_line,
    size_t out_cap
);

// Small helper: generate a trace_id string.
void yai_make_trace_id(char out[64]);

#ifdef __cplusplus
}
#endif

#endif
