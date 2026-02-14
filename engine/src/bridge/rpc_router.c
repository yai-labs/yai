#include "../../include/rpc_router.h"
#include "../../include/storage_gate.h"
#include "../../include/provider_gate.h"
#include "../external/cJSON.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

/* * NOTA: Abbiamo rimosso add_number_to_object e l'uso di cJSON per la risposta 
 * perché la versione locale di cJSON è incompleta.
 */

/* --- Handler per l'inference (Mock con snprintf per evitare bug di cJSON) --- */
char* handle_provider_inference(const char* ws_id, const yai_rpc_envelope_t* env, const char* payload) {
    (void)payload; // Ignoriamo il payload nel mock
    
    fprintf(stderr, "[ROUTER] Processing inference for WS: %s, Trace: %s\n", ws_id, env->trace_id);

    // Allocazione manuale della risposta per evitare cJSON_CreateNumber/Print
    char *out = (char*)malloc(512);
    if (!out) return NULL;

    snprintf(out, 512, 
        "{\"status\":\"success\",\"content\":\"YAI Engine Sovereign L2: Inference Mock Active\",\"tokens\":42,\"ws_id\":\"%s\"}", 
        ws_id);
    
    return out;
}

/* --- DISPATCHER UNICO --- */
char* yai_rpc_router_dispatch(const char* ws_id, const yai_rpc_envelope_t* env, const char* payload) {
    if (!env || !ws_id) return NULL;

    switch (env->command_id) {
        
        // Comandi STORAGE (L2 Graph)
        case 0x0201: // STORAGE_PUT
            return yai_storage_handle_rpc(ws_id, "put_node", payload);
            
        case 0x0202: // STORAGE_GET
            return yai_storage_handle_rpc(ws_id, "get_node", payload);

        // Comandi COGNITION (L3 Inference)
        case 0x0301: // YAI_CMD_INFERENCE
            return handle_provider_inference(ws_id, env, payload);
            
        case 0x0101: // PING
            return strdup("{\"status\":\"PONG\"}"); 

        default:
            fprintf(stderr, "[ROUTER] Unknown command_id: 0x%x for WS: %s\n", env->command_id, ws_id);
            char *err_msg = (char*)malloc(128);
            if (err_msg) {
                snprintf(err_msg, 128, "{\"error\":\"unsupported_command\",\"id\":%u}", env->command_id);
            }
            return err_msg;
    }
}