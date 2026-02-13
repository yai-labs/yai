// tools/cli/include/yai_envelope.h
#ifndef YAI_ENVELOPE_H
#define YAI_ENVELOPE_H

#include <stddef.h>

#ifndef YAI_RPC_V1
#define YAI_RPC_V1 1
#endif

#ifndef YAI_LINE_MAX
#define YAI_LINE_MAX 4096
#endif

// trace_id generator
void yai_make_trace_id(char out[64]);

// build envelope v1 JSONL into out buffer (includes trailing '\n')
int yai_build_request_jsonl(
    const char *ws_id,
    const char *trace_id,
    int arming,
    const char *role,
    const char *req_type,
    const char *payload_json, // object or "null"
    char *out,
    size_t out_cap
);

#endif
