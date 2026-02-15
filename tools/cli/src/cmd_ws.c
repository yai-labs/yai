// tools/cli/src/cmd_ws.c
#include "../include/yai_cli.h"
#include "../include/yai_rpc.h"

#include <stdio.h>
#include <string.h>

static void ws_usage(void) {
    printf("Workspace Management (L1)\n");
    printf("Usage:\n");
    printf("  yai kernel ws create  <id>\n");
    printf("  yai kernel ws list\n");
    printf("  yai kernel ws destroy <id>   (requires --arming)\n");
    printf("\nNotes:\n");
    printf("  - L1 is global (no --ws required)\n");
    printf("  - destroy requires --arming (ADR-004)\n");
}

static int ws_is_supported(const char *sub) {
    return (sub &&
            (strcmp(sub, "create") == 0 ||
             strcmp(sub, "list") == 0 ||
             strcmp(sub, "destroy") == 0 ||
             strcmp(sub, "help") == 0));
}

int yai_cmd_ws(int argc, char **argv, const yai_cli_opts_t *opt) {
    if (argc < 1) { ws_usage(); return 1; }

    const char *sub = argv[0];

    if (!ws_is_supported(sub)) {
        fprintf(stderr, "ERR: unsupported ws command: %s\n", sub ? sub : "(null)");
        ws_usage();
        return 2;
    }

    if (strcmp(sub, "help") == 0) {
        ws_usage();
        return 0;
    }

    // hard gate: destroy requires --arming
    if (strcmp(sub, "destroy") == 0) {
        if (!opt || !opt->arming) {
            fprintf(stderr, "ERR: ws destroy requires --arming\n");
            ws_usage();
            return 2;
        }
    }

    yai_rpc_client_t client;
    char response[YAI_RPC_LINE_MAX];

    // L1: connect root plane (no ws required)
    if (yai_rpc_connect(&client, NULL) != 0) {
        fprintf(stderr, "ERR: Could not connect to Root Plane. Is yai-kernel running?\n");
        return -1;
    }

    
    yai_rpc_set_authority(&client, opt ? opt->arming : 0, opt ? opt->role : "user");
// handshake always (ADR-004)
    const char *ver = (opt && opt->client_version && opt->client_version[0])
                        ? opt->client_version
                        : "cli-c/1.0";

    if (yai_rpc_handshake(&client, "kernel", ver) != 0) {
        fprintf(stderr, "ERR: Protocol handshake failed.\n");
        yai_rpc_close(&client);
        return 4;
    }

    // Canonical request:
    //   bin="kernel"
    //   request_type="control"
    //   payload={"method":"ws.list|ws.create|ws.destroy","params":{...}}
    char payload[512];

    if (strcmp(sub, "list") == 0) {
        int n = snprintf(payload, sizeof(payload),
                         "{\"method\":\"ws_list\",\"params\":{}}");
        if (n <= 0 || (size_t)n >= sizeof(payload)) {
            fprintf(stderr, "ERR: payload build failed\n");
            yai_rpc_close(&client);
            return 5;
        }

        int rc = yai_rpc_call(&client, "kernel", "control", payload, response, sizeof(response));
        if (rc == 0) {
            if (opt && opt->json) printf("%s\n", response);
            else                  printf("%s\n", response); // human mode: raw (deterministic)
        } else {
            fprintf(stderr, "RPC_ERROR: %d\n", rc);
        }

        yai_rpc_close(&client);
        return rc;
    }

    // create/destroy require id
    if (argc < 2 || !argv[1] || !argv[1][0]) {
        fprintf(stderr, "ERR: missing workspace id\n");
        ws_usage();
        yai_rpc_close(&client);
        return 2;
    }

    const char *ws_id = argv[1];
    const char *method = (strcmp(sub, "create") == 0) ? "ws_create" : "ws_destroy";

    int n = snprintf(payload, sizeof(payload),
                     "{\"method\":\"%s\",\"params\":{\"ws_id\":\"%s\"}}",
                     method, ws_id);

    if (n <= 0 || (size_t)n >= sizeof(payload)) {
        fprintf(stderr, "ERR: payload build failed\n");
        yai_rpc_close(&client);
        return 5;
    }

    int rc = yai_rpc_call(&client, "kernel", "control", payload, response, sizeof(response));
    if (rc == 0) {
        // human deterministic: print response (or a friendly line + json if --json)
        if (opt && opt->json) {
            printf("%s\n", response);
        } else {
            printf("%s\n", response);
        }
    } else {
        fprintf(stderr, "RPC_ERROR: %d\n", rc);
    }

    yai_rpc_close(&client);
    return rc;
}
