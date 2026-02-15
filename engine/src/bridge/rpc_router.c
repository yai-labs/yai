#include "../../law/specs/protocol/yai_protocol_ids.h"
#include "../../law/specs/protocol/transport.h"

#include "storage_gate.h"
#include "provider_gate.h"

#include "cJSON.h"

#include <string.h>
#include <stdio.h>
#include <stdlib.h>



static char* json_err(const char* code) {
    char* out = (char*)malloc(128);
    if (!out) return NULL;
    snprintf(out, 128, "{\"status\":\"error\",\"code\":\"%s\"}", code);
    return out;
}

static char* dispatch_storage_rpc(const char* ws_id, const char* payload) {
    if (!payload) return json_err("ERR_MISSING_PAYLOAD");

    cJSON* root = cJSON_Parse(payload);
    if (!root) return json_err("ERR_INVALID_JSON");

    cJSON* method_j = cJSON_GetObjectItem(root, "method");
    cJSON* params_j = cJSON_GetObjectItem(root, "params");

    if (!method_j || !cJSON_IsString(method_j) || !method_j->valuestring || method_j->valuestring[0] == '\0') {
        cJSON_Delete(root);
        return json_err("ERR_MISSING_METHOD");
    }

    // params può essere assente -> {}
    char* params_str = NULL;
    if (params_j) params_str = cJSON_PrintUnformatted(params_j);
    if (!params_str) {
        params_str = strdup("{}");
        if (!params_str) { cJSON_Delete(root); return NULL; }
    }

    char* resp = yai_storage_handle_rpc(ws_id, method_j->valuestring, params_str);

    free(params_str);
    cJSON_Delete(root);
    return resp;
}

char* yai_rpc_router_dispatch(const char* ws_id, const yai_rpc_envelope_t* env, const char* payload) {
    if (!env || !ws_id) return NULL;

    switch (env->command_id) {
        // CONTROL
        case YAI_CMD_PING:
            return strdup("{\"status\":\"PONG\"}");

        // STORAGE (single command_id, methods in payload)
        case YAI_CMD_STORAGE_RPC:
            // payload atteso:
            // { "method": "put_node" | "get_node" | "...", "params": { ... } }
            return dispatch_storage_rpc(ws_id, payload);

        // PROVIDERS
        case YAI_CMD_PROVIDER_RPC:
            // provider gate non ha bisogno di ws_id (l’envelope lo porta già / oppure lo gestisce il livello sopra)
            return yai_provider_gate_dispatch(env, payload);

        case YAI_CMD_EMBEDDING_RPC:
            // se non lo hai ancora implementato, rispondi esplicitamente
            return json_err("ERR_NOT_IMPLEMENTED");

        default:
            fprintf(stderr, "[ROUTER] Unknown command_id: 0x%x for WS: %s\n", env->command_id, ws_id);
            return json_err("ERR_UNSUPPORTED_COMMAND");
    }
}
