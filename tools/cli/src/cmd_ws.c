#include "../include/yai_cli.h"
#include "../include/yai_rpc.h"

#include <protocol/yai_protocol_ids.h>

#include <stdio.h>
#include <string.h>

static void ws_usage(void) {
    printf("Workspace Management (L1)\n");
    printf("Usage:\n");
    printf("  yai kernel ws create  <id>\n");
    printf("  yai kernel ws list\n");
    printf("  yai kernel ws destroy <id>   (requires --arming)\n");
}

int yai_cmd_ws(int argc, char **argv, const yai_cli_opts_t *opt)
{
    if (argc < 1)
        return 1;

    const char *sub = argv[0];

    yai_rpc_client_t client;
    char response[YAI_RPC_LINE_MAX];
    uint32_t resp_len = 0;

    if (yai_rpc_connect(&client, NULL) != 0)
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

    char payload[512];

    if (strcmp(sub, "list") == 0) {
        snprintf(payload, sizeof(payload),
                 "{\"method\":\"ws_list\",\"params\":{}}");
    } else {
        if (argc < 2) {
            yai_rpc_close(&client);
            return -3;
        }

        const char *method =
            (strcmp(sub, "create") == 0)
            ? "ws_create"
            : "ws_destroy";

        snprintf(payload, sizeof(payload),
                 "{\"method\":\"%s\",\"params\":{\"ws_id\":\"%s\"}}",
                 method, argv[1]);
    }

    int rc = yai_rpc_call_raw(
        &client,
        YAI_CMD_CONTROL,
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
