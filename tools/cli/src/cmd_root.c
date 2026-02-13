#include "../include/yai_cmd.h"
#include "../include/yai_rpc.h"
#include "../include/yai_fmt.h"
#include "../include/yai_envelope.h"
#include "../include/yai_cmd_law.h"   // <-- ADD

#include <stdio.h>
#include <string.h>

// ... usage() aggiungi queste righe:
static void usage(void) {
    fprintf(stderr,
        "USAGE:\n"
        "  yai --ws <id> ping\n"
        "  yai --ws <id> status\n"
        "  yai --ws <id> call <type> [payload_json]\n"
        "  yai law check|tree|paths\n"
        "FLAGS:\n"
        "  --ws <id>           workspace id\n"
        "  --arming            set arming=true\n"
        "  --role <user|operator>\n"
        "  --json              print raw json line\n"
        "  --client <ver>      client version string\n"
    );
}

int yai_cmd_dispatch(int argc, char **argv, const yai_cli_opts_t *opt) {
    if (argc < 1) { usage(); return 2; }

    const char *cmd = argv[0];

    
    if (!strcmp(cmd, "law")) {
        if (argc < 2) { fprintf(stderr, "ERR: law requires subcommand\n"); return 2; }
        return yai_cmd_law(argc - 1, argv + 1, opt);
    }

    if (!strcmp(cmd, "help") || !strcmp(cmd, "--help") || !strcmp(cmd, "-h")) {
        usage();
        return 0;
    }

    if (!opt || !opt->ws_id || !opt->ws_id[0]) {
        fprintf(stderr, "ERR: --ws is required for now (root control-plane not implemented yet)\n");
        return 2;
    }

    yai_rpc_client_t c;
    int rc = yai_rpc_connect(&c, opt->ws_id);
    if (rc != 0) { fprintf(stderr, "ERR: rpc connect failed (rc=%d)\n", rc); return 3; }

    // handshake first
    rc = yai_rpc_handshake(&c, opt->client_version ? opt->client_version : "cli-c/0.1");
    if (rc != 0) { fprintf(stderr, "ERR: handshake failed (rc=%d)\n", rc); yai_rpc_close(&c); return 4; }

    char tr[64];
    yai_make_trace_id(tr);

    char out[YAI_RPC_LINE_MAX];

    if (!strcmp(cmd, "ping")) {
        rc = yai_rpc_call(&c, tr, "ping", 0, opt->role, "null", out, sizeof(out));
    } else if (!strcmp(cmd, "status")) {
        rc = yai_rpc_call(&c, tr, "status", opt->arming, opt->role, "null", out, sizeof(out));
    } else if (!strcmp(cmd, "call")) {
        if (argc < 2) { fprintf(stderr, "ERR: call requires <type> [payload_json]\n"); yai_rpc_close(&c); return 2; }
        const char *type = argv[1];
        const char *payload = (argc >= 3) ? argv[2] : "null";
        rc = yai_rpc_call(&c, tr, type, opt->arming, opt->role, payload, out, sizeof(out));
    } else {
        fprintf(stderr, "ERR: unknown command: %s\n", cmd);
        yai_rpc_close(&c);
        usage();
        return 2;
    }

    yai_rpc_close(&c);

    if (rc != 0) {
        fprintf(stderr, "ERR: rpc call failed (rc=%d)\n", rc);
        return 5;
    }

    yai_print_response(out, opt->json);
    return 0;
}
