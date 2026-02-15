#include "../include/yai_cmd.h"
#include "../include/yai_rpc.h"

#include <protocol/yai_protocol_ids.h>

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* ============================================================
   L3: Mind Dispatcher (BINARY)
   ============================================================ */

int yai_cmd_mind(int argc, char **argv, const yai_cli_opts_t *opt)
{
    if (argc < 2)
        return 1;

    const char *method = argv[0];
    const char *prompt = argv[1];

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
    snprintf(payload, sizeof(payload),
             "{\"method\":\"%s\",\"params\":{\"prompt\":\"%s\",\"stream\":false}}",
             method, prompt);

    int rc = yai_rpc_call_raw(
        &client,
        YAI_CMD_PROVIDER_RPC,
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
