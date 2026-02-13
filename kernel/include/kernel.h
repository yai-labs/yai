#ifndef KERNEL_H
#define KERNEL_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#include "yai_vault.h"
#include "yai_kernel.h"
#include "yai_events.h"

// -------------------------
// Phase-1 Gate error codes (stable)
// -------------------------
#define YAI_E_OK                0
#define YAI_E_BAD_ARG          -1
#define YAI_E_BAD_VERSION      -2
#define YAI_E_MISSING_WS       -3
#define YAI_E_WS_MISMATCH      -4
#define YAI_E_MISSING_TYPE     -5
#define YAI_E_TYPE_NOT_ALLOWED -6
#define YAI_E_PRIV_REQUIRED    -7
#define YAI_E_ROLE_REQUIRED    -8

// New: handshake enforcement (Phase-1 strict)
#define YAI_E_HANDSHAKE_REQUIRED -9

// -------------------------
// Error model (rpc.v1)
// -------------------------
#define YAI_RPC_V1 1

// Max JSON error response length (keep small; detail can be truncated)
#ifndef YAI_RPC_ERRBUF
#define YAI_RPC_ERRBUF 512
#endif

// Kernel error response (json-line) writer
// detail_json MUST be a valid JSON value (object/string/null), not a raw string.
// Example: "{}" or "\"extra info\"" or "null"
int yai_rpc_write_error_v1(
    int fd,
    const char *ws_id,
    const char *trace_id,
    const char *code,
    const char *message,
    const char *detail_json
);

// -------------------------
// Event discipline (kernel events)
// -------------------------
#ifndef YAI_KERNEL_EVENT_SCHEMA_ID
#define YAI_KERNEL_EVENT_SCHEMA_ID "yai.kernel.event.v1"
#endif

#ifndef YAI_KERNEL_EVENT_VERSION
#define YAI_KERNEL_EVENT_VERSION 1
#endif

// Minimal event writer: always includes schema_id + event_version.
// msg is human-readable; data_json must be valid JSON (object/string/null).
// New (disciplinato, JSON event)
void yai_log_static(
    yai_event_type_t type,
    const char *ws_id,
    const char *trace_id,
    const char *level,
    const char *msg,
    const char *data_json
);

// Legacy shim (per compatibilità interna durante migrazione Phase-1)
static inline void yai_log_static_legacy(yai_event_type_t type, const char *msg) {
    yai_log_static(type, "", "", "info", msg, "null");
}


// Envelope validator (rpc.v1, jsonl frame)
int yai_validate_envelope_v1(
    const char *line,
    const char *expected_ws,     // NULL / "" se non “attached”
    char *out_request_type,
    size_t req_cap
);


#endif
