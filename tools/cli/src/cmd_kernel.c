#include "../include/yai_cli.h"
#include "../include/yai_rpc.h"

#include <protocol/yai_protocol_ids.h>

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/*
 * L1: Kernel Dispatcher (Binary Protocol)
 *
 * Usage:
 *   yai kernel <ping|status|stop|...>
 *
 * Behavior:
 *   - One connection per command
 *   - Authority set before handshake
 *   - Strict binary envelope
 */

int yai_cmd_kernel(int argc, char **argv, const yai_cli_opts_t *opt)
{
    if (argc < 1)
        return 1;

    const char *cmd = argv[0];

    uint32_t command_id =
        (strcmp(cmd, "ping") == 0)
        ? YAI_CMD_PING
        : YAI_CMD_CONTROL;

    yai_rpc_client_t client;
    char response[YAI_RPC_LINE_MAX];
    uint32_t resp_len = 0;

    /* ---------------- CONNECT ---------------- */

    if (yai_rpc_connect(&client, opt ? opt->ws_id : NULL) != 0)
        return -1;

    /* ---------------- AUTHORITY ---------------- */

    yai_rpc_set_authority(&client,
                          opt ? opt->arming : 0,
                          opt ? opt->role : "user");

    /* ---------------- HANDSHAKE ---------------- */

    if (yai_rpc_handshake(&client) != 0)
    {
        yai_rpc_close(&client);
        return -2;
    }

    /* ---------------- PAYLOAD ---------------- */

    char payload[256];

    int n = snprintf(payload,
                     sizeof(payload),
                     "{\"method\":\"%s\",\"params\":{}}",
                     cmd);

    if (n <= 0 || (size_t)n >= sizeof(payload))
    {
        yai_rpc_close(&client);
        return -3;
    }

    /* ---------------- RPC CALL ---------------- */

    int rc = yai_rpc_call_raw(
        &client,
        command_id,
        payload,
        (uint32_t)strlen(payload),
        response,
        sizeof(response) - 1,
        &resp_len
    );

    if (rc == 0)
    {
        response[resp_len] = '\0';
        printf("%s\n", response);
    }

    /* ---------------- CLOSE ---------------- */

    yai_rpc_close(&client);
    return rc;
}
