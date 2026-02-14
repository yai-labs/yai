// kernel/src/core/envelope_v1.c
#include "kernel.h"
#include <string.h>

static const char *skip_ws(const char *p) {
    while (p && (*p == ' ' || *p == '\t' || *p == '\r' || *p == '\n')) p++;
    return p;
}

static int extract_string_field_from(const char *json, const char *key, char *out, size_t cap) {
    if (!json || !key || !out || cap < 2) return 0;

    const char *p = strstr(json, key);
    if (!p) return 0;

    p += strlen(key);
    p = strchr(p, ':');
    if (!p) return 0;
    p++;
    p = skip_ws(p);

    if (*p != '"') return 0;
    p++;

    const char *end = strchr(p, '"');
    if (!end) return 0;

    size_t len = (size_t)(end - p);
    if (len == 0 || len >= cap) return 0;

    memcpy(out, p, len);
    out[len] = '\0';
    return 1;
}

static int extract_bool_field_from(const char *json, const char *key, int *out_bool) {
    if (!json || !key || !out_bool) return 0;

    const char *p = strstr(json, key);
    if (!p) return 0;

    p += strlen(key);
    p = strchr(p, ':');
    if (!p) return 0;
    p++;
    p = skip_ws(p);

    if (strncmp(p, "true", 4) == 0) { *out_bool = 1; return 1; }
    if (strncmp(p, "false", 5) == 0) { *out_bool = 0; return 1; }
    return 0;
}

static int contains_token(const char *s, const char *token) {
    return s && token && strstr(s, token) != NULL;
}

static int is_allowed_type_phase1(const char *t) {
    if (!t || t[0] == '\0') return 0;
    return (strcmp(t, "ping") == 0) ||
           (strcmp(t, "protocol_handshake") == 0) ||
           (strcmp(t, "status") == 0);
}

int yai_validate_envelope_v1(
    const char *line,
    const char *expected_ws,
    char *out_request_type,
    size_t req_cap
) {
    if (!line || !out_request_type || req_cap < 2) return YAI_E_BAD_ARG;

    // 1) Version: v=1
    if (!contains_token(line, "\"v\":1") && !contains_token(line, "\"v\": 1")) {
        return YAI_E_BAD_VERSION;
    }

    // 2) ws_id required
    char ws_buf[64] = {0};
    if (!extract_string_field_from(line, "\"ws_id\"", ws_buf, sizeof(ws_buf))) {
        return YAI_E_MISSING_WS;
    }
    if (expected_ws && expected_ws[0] != '\0') {
        if (strcmp(ws_buf, expected_ws) != 0) return YAI_E_WS_MISMATCH;
    }

    // 3) request.type (canonical shape: request:{type:"..."})
    char type_buf[64] = {0};

    const char *req_pos = strstr(line, "\"request\"");
    if (req_pos) {
        if (!extract_string_field_from(req_pos, "\"type\"", type_buf, sizeof(type_buf))) {
            (void)extract_string_field_from(line, "\"type\"", type_buf, sizeof(type_buf));
        }
    } else {
        (void)extract_string_field_from(line, "\"type\"", type_buf, sizeof(type_buf));
    }

    if (type_buf[0] == '\0') return YAI_E_MISSING_TYPE;

    size_t tlen = strlen(type_buf);
    if (tlen + 1 > req_cap) return YAI_E_MISSING_TYPE;
    memcpy(out_request_type, type_buf, tlen + 1);

    // 4) Phase-1 allowlist
    if (!is_allowed_type_phase1(out_request_type)) {
        return YAI_E_TYPE_NOT_ALLOWED;
    }

    // 5) Privileged gate scaffold: if arming=true => role="operator"
    int arming = 0;
    if (extract_bool_field_from(line, "\"arming\"", &arming) && arming) {
        char role_buf[32] = {0};
        if (!extract_string_field_from(line, "\"role\"", role_buf, sizeof(role_buf))) {
            return YAI_E_ROLE_REQUIRED;
        }
        if (strcmp(role_buf, "operator") != 0) return YAI_E_ROLE_REQUIRED;
    }

    return YAI_E_OK;
}
