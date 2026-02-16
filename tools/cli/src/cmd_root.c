#define _POSIX_C_SOURCE 200809L

#include "../include/yai_cmd.h"
#include "../include/yai_rpc.h"
#include "../include/yai_fmt.h"

#include <protocol/yai_protocol_ids.h>

#include <stdio.h>
#include <string.h>

/* ============================================================
   USAGE
   ============================================================ */

static void usage_root(void)
{
    fprintf(stderr,
        "Machine Root Plane\n"
        "Usage:\n"
        "  yai-cli root status\n"
        "  yai-cli root ping\n");
}

/* ============================================================
   CONNECT + HANDSHAKE
   ============================================================ */

static int root_connect_and_handshake(yai_rpc_client_t *c)
{
    if (!c)
        return -1;

    memset(c, 0, sizeof(*c));

    /* Root plane always runs on system workspace */
    if (yai_rpc_connect(c, "system") != 0) {
        fprintf(stderr, "[CLI] Failed to connect to root plane\n");
        return -2;
    }

    /* Escalate authority */
    yai_rpc_set_authority(c, 1, "operator");

    int rc = yai_rpc_handshake(c);
    if (rc != 0) {
        fprintf(stderr, "[CLI] Root handshake failed (rc=%d)\n", rc);
        yai_rpc_close(c);
        return -3;
    }

    return 0;
}

/* ============================================================
   ROOT PING
   ============================================================ */

static int cmd_root_ping(void)
{
    yai_rpc_client_t c;

    if (root_connect_and_handshake(&c) != 0)
        return -10;

    char response[YAI_RPC_LINE_MAX];
    uint32_t resp_len = 0;

    int rc = yai_rpc_call_raw(
        &c,
        YAI_CMD_PING,
        NULL,
        0,
        response,
        sizeof(response) - 1,
        &resp_len
    );

    if (rc != 0) {
        fprintf(stderr, "[CLI] Root ping failed (rc=%d)\n", rc);
        yai_rpc_close(&c);
        return -20;
    }

    response[resp_len] = '\0';
    printf("%s\n", response);

    yai_rpc_close(&c);
    return 0;
}

/* ============================================================
   ROOT STATUS (alias ping)
   ============================================================ */

static int cmd_root_status(void)
{
    return cmd_root_ping();
}

/* ============================================================
   ENTRY
   ============================================================ */

int yai_cmd_root(int argc,
                 char **argv,
                 const yai_cli_opts_t *opt)
{
    (void)opt;

    if (argc < 1) {
        usage_root();
        return 1;
    }

    if (strcmp(argv[0], "ping") == 0)
        return cmd_root_ping();

    if (strcmp(argv[0], "status") == 0)
        return cmd_root_status();

    if (strcmp(argv[0], "help") == 0) {
        usage_root();
        return 0;
    }

    usage_root();
    return 2;
}
