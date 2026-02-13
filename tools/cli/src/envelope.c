// tools/cli/src/envelope.c
#include "../include/yai_envelope.h"
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

static const char *safe_cstr(const char *s) { return (s && s[0]) ? s : ""; }

void yai_make_trace_id(char out[64]) {
    static unsigned long long ctr = 0;
    unsigned long long t = (unsigned long long)time(NULL);
    unsigned long long p = (unsigned long long)getpid();
    ctr++;
    snprintf(out, 64, "cli-%llx-%llx-%llx", t, p, ctr);
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
    if (!req_type || !req_type[0] || !out || out_cap < 32) return -1;

    const char *ws = ws_id ? ws_id : "";
    const char *tr = safe_cstr(trace_id);
    const char *rl = safe_cstr(role);
    const char *pj = payload_json ? payload_json : "null";

    // Strict-ish: payload must be object or null
    // (we don't parse; kernel will validate strictly)
    int n = snprintf(
        out,
        out_cap,
        "{\"v\":%d,"
        "\"ws_id\":%s,"
        "\"trace_id\":\"%s\","
        "\"arming\":%s,"
        "\"role\":\"%s\","
        "\"request\":{"
            "\"type\":\"%s\","
            "\"payload\":%s"
        "}}"
        "\n",
        YAI_RPC_V1,
        (ws && ws[0]) ? "\"" : "null",
        tr,
        arming ? "true" : "false",
        rl,
        req_type,
        pj
    );

    // patch ws_id quoting if non-null
    // we used ws_id as: "ws_id": "   OR null then patch:
    // If ws present, we need "ws_id":"<ws>"
    if (ws && ws[0]) {
        // rewrite the `"ws_id":"` portion in place: easiest: rebuild correctly
        n = snprintf(
            out,
            out_cap,
            "{\"v\":%d,"
            "\"ws_id\":\"%s\","
            "\"trace_id\":\"%s\","
            "\"arming\":%s,"
            "\"role\":\"%s\","
            "\"request\":{"
                "\"type\":\"%s\","
                "\"payload\":%s"
            "}}"
            "\n",
            YAI_RPC_V1,
            ws,
            tr,
            arming ? "true" : "false",
            rl,
            req_type,
            pj
        );
    }

    if (n <= 0 || (size_t)n >= out_cap) return -2;
    return 0;
}
