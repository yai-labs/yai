#include "../include/yai_cli.h"
#include "../include/yai_rpc.h"

#include <protocol/yai_protocol_ids.h>

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/*
 * L2: Engine Gates Dispatcher (BINARY PROTOCOL)
 *
 * Usage:
 *   yai engine [--ws id] <storage|provider|embedding> <method> [params_json]
 */

int yai_cmd_engine(int argc, char **argv, const yai_cli_opts_t *opt)
{
    if (argc < 2) {
        printf("Engine Gates (L2)\n");
        printf("Usage: yai engine [--ws id] <storage|provider|embedding> <method> [params_json]\n");
        return 1;
    }

    const char *gate   = argv[0];
    const char *method = argv[1];
    const char *params = (argc > 2 && argv[2] && argv[2][0]) ? argv[2] : "{}";

    uint32_t command_id = 0;

    if (strcmp(gate, "storage") == 0)
        command_id = YAI_CMD_STORAGE_RPC;
    else if (strcmp(gate, "provider") == 0)
        command_id = YAI_CMD_PROVIDER_RPC;
    else if (strcmp(gate, "embedding") == 0)
        command_id = YAI_CMD_EMBEDDING_RPC;
    else {
        fprintf(stderr, "ERR: invalid gate\n");
        return 2;
    }

    yai_rpc_client_t client;
    char response[YAI_RPC_LINE_MAX];
    uint32_t resp_len = 0;

    if (yai_rpc_connect(&client, opt ? opt->ws_id : NULL) != 0)
        return -1;

    /* ---------- AUTHORITY ---------- */

    if (opt && opt->arming)
        yai_rpc_set_authority(&client, 1, "operator");
    else
        yai_rpc_set_authority(&client, 0, "guest");

    if (yai_rpc_handshake(&client) != 0) {
        yai_rpc_close(&client);
        return -2;
    }

    char payload[8192];
    int n = snprintf(payload, sizeof(payload),
                     "{\"method\":\"%s\",\"params\":%s}",
                     method, params);

    if (n <= 0 || (size_t)n >= sizeof(payload)) {
        yai_rpc_close(&client);
        return -3;
    }

    int rc = yai_rpc_call_raw(
        &client,
        command_id,
        payload,
        (uint32_t)strlen(payload),
        response,
        sizeof(response) - 1,
        &resp_len
    );

    if (rc == 0) {
        response[resp_len] = '\0';
        printf("%s\n", response);
    }

    yai_rpc_close(&client);
    return rc;
}
