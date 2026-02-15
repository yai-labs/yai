#include "../include/yai_cli.h"
#include "../include/yai_rpc.h"

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/**
 * L2: Engine Gates Dispatcher (canonico)
 * Usage:
 *   yai engine [--ws id] <storage|provider|embedding> <method> [params_json]
 *
 * Wire:
 *   command_id = YAI_CMD_STORAGE_RPC / YAI_CMD_PROVIDER_RPC / YAI_CMD_EMBEDDING_RPC
 *   payload    = {"method":"X","params":{...}}
 */
int yai_cmd_engine(int argc, char **argv, const yai_cli_opts_t *opt) {
    if (argc < 2) {
        printf("Engine Gates (L2)\n");
        printf("Usage: yai engine [--ws id] <storage|provider|embedding> <method> [params_json]\n");
        printf("Examples:\n");
        printf("  yai engine --ws default storage put_node '{\"id\":\"n1\",\"label\":\"hello\"}'\n");
        printf("  yai engine --ws default provider infer '{\"prompt\":\"ciao\"}'\n");
        return 1;
    }

    const char *gate   = argv[0]; // storage|provider|embedding
    const char *method = argv[1];
    const char *params = (argc > 2 && argv[2] && argv[2][0]) ? argv[2] : "{}";

    if (strcmp(gate, "storage") != 0 && strcmp(gate, "provider") != 0 && strcmp(gate, "embedding") != 0) {
        fprintf(stderr, "ERR: gate must be one of: storage | provider | embedding\n");
        return 2;
    }
    if (!method || !method[0]) {
        fprintf(stderr, "ERR: missing method\n");
        return 3;
    }

    yai_rpc_client_t client;
    char response[YAI_RPC_LINE_MAX];

    if (yai_rpc_connect(&client, opt ? opt->ws_id : NULL) != 0) {
        fprintf(stderr, "ERR: Could not connect to Root Plane.\n");
        return -1;
    }

    
    yai_rpc_set_authority(&client, opt ? opt->arming : 0, opt ? opt->role : "user");
// Canonical payload: {"method":"...","params":{...}}
    // NOTA: params deve essere JSON valido (object/array/primitive). Per engine ci va bene così.
    char payload[8192];
    int n = snprintf(payload, sizeof(payload),
                     "{\"method\":\"%s\",\"params\":%s}",
                     method, params);

    if (n <= 0 || (size_t)n >= sizeof(payload)) {
        fprintf(stderr, "ERR: payload too large\n");
        yai_rpc_close(&client);
        return -2;
    }

    int rc = yai_rpc_call(
        &client,
        "engine",
        gate,        // <- CANON: storage/provider/embedding
        payload,     // <- CANON: wrapper method+params
        response,
        sizeof(response)
    );

    if (rc == 0) {
        printf("%s\n", response);
    } else {
        fprintf(stderr, "RPC_ERROR: %d\n", rc);
    }

    yai_rpc_close(&client);
    return rc;
}
