// tools/cli/src/cmd_mind.c
#include "../include/yai_cmd.h"
#include "../include/yai_rpc.h"

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* Escape minimale per stringhe JSON (quote, backslash, newline, tab, CR). */
static int json_escape_into(const char *in, char *out, size_t cap) {
    if (!out || cap < 2) return -1;
    size_t o = 0;
    if (!in) in = "";

    for (size_t i = 0; in[i] != '\0'; i++) {
        unsigned char c = (unsigned char)in[i];
        const char *rep = NULL;
        char tmp[7];

        switch (c) {
            case '\"': rep = "\\\""; break;
            case '\\': rep = "\\\\"; break;
            case '\n': rep = "\\n"; break;
            case '\r': rep = "\\r"; break;
            case '\t': rep = "\\t"; break;
            default:
                if (c < 0x20) {
                    // control chars -> \u00XX
                    snprintf(tmp, sizeof(tmp), "\\u%04x", (unsigned)c);
                    rep = tmp;
                }
                break;
        }

        if (rep) {
            size_t rn = strlen(rep);
            if (o + rn + 1 >= cap) return -2;
            memcpy(out + o, rep, rn);
            o += rn;
        } else {
            if (o + 2 >= cap) return -3;
            out[o++] = (char)c;
        }
    }

    out[o] = '\0';
    return 0;
}

/**
 * L3: Mind & Cognition Dispatcher (canonico)
 * Usage:
 *   yai mind <chat|think|query> <prompt>
 *
 * Wire (canonico, layer-agnostic):
 *   request_type = "provider" (o "provider_<method>" legacy)
 *   payload      = {"method":"chat|think|query","params":{"prompt":"...","stream":false}}
 *
 * NOTE:
 * - yai_rpc_call() impacchetta in envelope bin/ws_id/trace + command_id (via map_command_id in rpc.c)
 */
int yai_cmd_mind(int argc, char **argv, const yai_cli_opts_t *opt) {
    if (argc < 2) {
        printf("Mind Intelligence Interface (L3)\n");
        printf("Usage: yai mind <chat|think|query> <prompt>\n");
        printf("Examples:\n");
        printf("  yai mind --ws default chat \"ciao\"\n");
        printf("  yai mind --ws default think \"riassumi questo\"\n");
        return 1;
    }

    const char *method = argv[0]; // chat|think|query
    const char *prompt = argv[1];

    if (strcmp(method, "chat") != 0 &&
        strcmp(method, "think") != 0 &&
        strcmp(method, "query") != 0) {
        fprintf(stderr, "ERR: method must be one of: chat | think | query\n");
        return 2;
    }

    if (!prompt || !prompt[0]) {
        fprintf(stderr, "ERR: missing prompt\n");
        return 3;
    }

    // escape prompt -> safe JSON string
    char esc[4096];
    if (json_escape_into(prompt, esc, sizeof(esc)) != 0) {
        fprintf(stderr, "ERR: prompt too large (or contains unsupported control chars)\n");
        return 4;
    }

    yai_rpc_client_t client;
    char response[YAI_RPC_LINE_MAX];

    // L3 richiede ws_id (enforced già nel root dispatcher; qui best-effort)
    if (yai_rpc_connect(&client, opt ? opt->ws_id : NULL) != 0) {
        fprintf(stderr, "ERR: Could not connect to Root Plane.\n");
        return -1;
    }

    
    yai_rpc_set_authority(&client, opt ? opt->arming : 0, opt ? opt->role : "user");
// Canonical payload (method+params)
    char payload[8192];
    int n = snprintf(payload, sizeof(payload),
                     "{\"method\":\"%s\",\"params\":{\"prompt\":\"%s\",\"stream\":false}}",
                     method, esc);

    if (n <= 0 || (size_t)n >= sizeof(payload)) {
        fprintf(stderr, "ERR: payload build failed (too large)\n");
        yai_rpc_close(&client);
        return 5;
    }

    // NB: request_type per L3 -> "provider" (rpc.c mapperà a YAI_CMD_PROVIDER_RPC)
    int rc = yai_rpc_call(
        &client,
        "mind",           // bin target
        "provider",       // request_type canonico (L3 -> provider rpc)
        payload,          // payload canonico
        response,
        sizeof(response)
    );

    if (rc == 0) {
        // risposta già line-based dal root plane
        printf("%s\n", response);
    } else {
        fprintf(stderr, "ERR: Mind RPC call failed (rc=%d)\n", rc);
    }

    yai_rpc_close(&client);
    return rc;
}
