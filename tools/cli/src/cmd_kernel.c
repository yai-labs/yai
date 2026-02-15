#include "../include/yai_cli.h"
#include "../include/yai_rpc.h"
#include <stdio.h>
#include <string.h>

/**
 * L1: Kernel Dispatcher (canonico)
 * Comandi globali (no tenant isolation):
 *   yai kernel <status|ping|stop>
 *
 * Wire:
 *   bin          = "kernel"
 *   request_type = "control"
 *   payload      = {"method":"status|ping|stop","params":{}}
 */
int yai_cmd_kernel(int argc, char **argv, const yai_cli_opts_t *opt) {
    if (argc < 1) {
        printf("Kernel Control (L1)\nUsage: yai kernel <status|ping|stop>\n");
        return 1;
    }

    const char *cmd = argv[0];

    // whitelist dura (no drift)
    if (strcmp(cmd, "status") != 0 &&
        strcmp(cmd, "ping")   != 0 &&
        strcmp(cmd, "stop")   != 0) {
        fprintf(stderr, "ERR: unsupported kernel command: %s\n", cmd);
        fprintf(stderr, "Usage: yai kernel <status|ping|stop>\n");
        return 2;
    }

    yai_rpc_client_t client;
    char response[YAI_RPC_LINE_MAX];

    // L1: no ws_id
    if (yai_rpc_connect(&client, NULL) != 0) {
        fprintf(stderr, "ERR: Connection to Root Plane failed. Is yai-kernel running?\n");
        return -1;
    }

    
    yai_rpc_set_authority(&client, opt ? opt->arming : 0, opt ? opt->role : "user");
// handshake: valida protocollo/compatibilità (best effort ma se fallisce -> stop)
    const char *ver = (opt && opt->client_version && opt->client_version[0])
                        ? opt->client_version
                        : "cli-c/1.0";

    if (yai_rpc_handshake(&client, "kernel", ver) != 0) {
        fprintf(stderr, "ERR: Protocol handshake failed.\n");
        yai_rpc_close(&client);
        return 4;
    }

    // Canonical payload (sempre method+params)
    char payload[256];
    int n = snprintf(payload, sizeof(payload),
                     "{\"method\":\"%s\",\"params\":{}}",
                     cmd);
    if (n <= 0 || (size_t)n >= sizeof(payload)) {
        fprintf(stderr, "ERR: payload build failed\n");
        yai_rpc_close(&client);
        return 5;
    }

    // request_type canonico per L1
    int rc = yai_rpc_call(
        &client,
        "kernel",         // bin target
        "control",        // request type canonico L1
        payload,          // payload canonico
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
