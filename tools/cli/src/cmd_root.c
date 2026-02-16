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
        "  yai root status\n"
        "  yai root ping\n");
}

/* ============================================================
   CONNECT ROOT (system plane)
   ============================================================ */

static int connect_root(yai_rpc_client_t *c)
{
    memset(c, 0, sizeof(*c));

    /* Root always runs in system workspace */
    if (yai_rpc_connect(c, "system") != 0) {
        fprintf(stderr, "[CLI] Failed to connect to root plane\n");
        return -1;
    }

    return 0;
}

/* ============================================================
   HANDSHAKE (Root requires operator + arming)
   ============================================================ */

static int root_handshake(yai_rpc_client_t *c)
{
    /* Root is sovereign machine control.
       We always escalate to operator + armed. */
    yai_rpc_set_authority(
        c,
        1,              /* arming */
        "operator"      /* role */
    );

    if (yai_rpc_handshake(c) != 0) {
        fprintf(stderr, "[CLI] Root handshake failed\n");
        return -1;
    }

    return 0;
}

/* ============================================================
   STATUS / PING
   ============================================================ */

static int cmd_root_ping(void)
{
    yai_rpc_client_t c;

    if (connect_root(&c) != 0)
        return -5;

    if (root_handshake(&c) != 0) {
        yai_rpc_close(&c);
        return -6;
    }

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

    if (rc == 0) {
        response[resp_len] = '\0';
        printf("%s\n", response);
    } else {
        fprintf(stderr, "[CLI] Root ping failed (rc=%d)\n", rc);
    }

    yai_rpc_close(&c);
    return rc;
}

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
    (void)opt; /* unused for root */

    if (argc < 1) {
        usage_root();
        return 1;
    }

    if (strcmp(argv[0], "status") == 0)
        return cmd_root_status();

    if (strcmp(argv[0], "ping") == 0)
        return cmd_root_ping();

    if (strcmp(argv[0], "help") == 0) {
        usage_root();
        return 0;
    }

    usage_root();
    return 2;
}
