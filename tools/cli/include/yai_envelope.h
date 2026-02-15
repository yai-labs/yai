#ifndef YAI_ENVELOPE_H
#define YAI_ENVELOPE_H

#include <stddef.h>

#define YAI_RPC_V1 1

void yai_make_trace_id(char out[64]);

/**
 * ADR-004 (CLI -> Root Control Plane) JSONL builder.
 *
 * Canonical line format (single line, newline-terminated):
 * {
 *   "v":1,
 *   "bin":"kernel|engine|mind",
 *   "ws_id":"<id>" | null,
 *   "type":"control|storage|provider|cognition",
 *   "trace":"...",
 *   "payload": { ... }   // MUST be JSON object
 * }
 */
int yai_build_v1_envelope(
    const char *bin,
    const char *ws_id,
    const char *trace_id,
    const char *req_type,
    const char *payload_json,
    char *out,
    size_t out_cap
);

/**
 * Legacy compatibility wrapper (kept to avoid breaking old cmd_*)
 * NOTE: arming/role are ignored here (migrate callers to canonical payload).
 */
int yai_build_request_jsonl(
    const char *ws_id,
    const char *trace_id,
    int arming,
    const char *role,
    const char *req_type,
    const char *payload_json,
    char *out,
    size_t out_cap
);

#endif
