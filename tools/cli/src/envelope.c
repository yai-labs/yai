#include "../include/yai_envelope.h"
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

void yai_make_trace_id(char out[64]) {
    static unsigned long long ctr = 0;
    unsigned long long t = (unsigned long long)time(NULL);
    unsigned long long p = (unsigned long long)getpid();
    ctr++;
    snprintf(out, 64, "cli-%llx-%llx-%llx", t, p, ctr);
}

static const char* safe_cstr(const char *s) { return (s && s[0]) ? s : ""; }

int yai_build_v1_envelope(
    const char *bin,
    const char *ws_id,
    const char *trace_id,
    const char *req_type,
    const char *payload_json,
    char *out,
    size_t out_cap
) {
    if (!bin || !bin[0] || !req_type || !req_type[0] || !out || out_cap < 64) return -1;

    const char *target_ws = (ws_id && ws_id[0]) ? ws_id : NULL;
    const char *trace = (trace_id && trace_id[0]) ? trace_id : "null";

    // payload MUST be JSON object. If missing, use {}
    const char *payload = (payload_json && payload_json[0]) ? payload_json : "{}";

    // NOTE: we assume caller passes a JSON object string.
    // If caller passes non-object, runtime should reject in strict mode later.
    int n;
    if (target_ws) {
        n = snprintf(
            out, out_cap,
            "{\"v\":1,\"bin\":\"%s\",\"ws_id\":\"%s\",\"type\":\"%s\",\"trace\":\"%s\",\"payload\":%s}\n",
            safe_cstr(bin), safe_cstr(target_ws), safe_cstr(req_type), safe_cstr(trace), payload
        );
    } else {
        n = snprintf(
            out, out_cap,
            "{\"v\":1,\"bin\":\"%s\",\"ws_id\":null,\"type\":\"%s\",\"trace\":\"%s\",\"payload\":%s}\n",
            safe_cstr(bin), safe_cstr(req_type), safe_cstr(trace), payload
        );
    }

    if (n <= 0 || (size_t)n >= out_cap) return -2;
    return 0;
}

int yai_build_request_jsonl(
    const char *ws_id,
    const char *trace_id,
    int arming,
    const char *role,
    const char *req_type,
    const char *payload_json,
    char *out,
    size_t out_cap
) {
    (void)arming;
    (void)role;
    // Legacy defaults to kernel
    return yai_build_v1_envelope("kernel", ws_id, trace_id, req_type, payload_json, out, out_cap);
}
