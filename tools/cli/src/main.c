// tools/cli/src/main.c
#include "../include/yai_cli.h"
#include "../include/yai_cmd.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef YAI_CLI_VERSION
#define YAI_CLI_VERSION "yai-cli/1.0"
#endif

static void print_usage(void) {
    printf("YAI Sovereign CLI\n\n");
    printf("Usage:\n");
    printf("  yai <bin> [options] <command> [args...]\n");
    printf("  yai --help | -h\n");
    printf("  yai --version | -V\n\n");
    printf("Bins (Levels):\n");
    printf("  kernel   L1: Root control + workspaces\n");
    printf("  engine   L2: Gates (storage/network/resource/provider)\n");
    printf("  mind     L3: Cognition (requires --ws)\n");
    printf("  law      Local checks (no RPC)\n\n");
    printf("Options:\n");
    printf("  -w, --ws <id>     Target workspace (required for engine/mind)\n");
    printf("  -j, --json        Raw JSON output\n");
    printf("  -a, --arming      Enable privileged/destructive commands\n");
    printf("  -r, --role <...>  Role (default: user)\n\n");
    printf("Examples:\n");
    printf("  yai law check\n");
    printf("  yai kernel status\n");
    printf("  yai kernel ws list\n");
    printf("  yai engine --ws dev storage put_node '{\"id\":\"n1\"}'\n");
    printf("  yai mind   --ws dev chat \"ciao\"\n");
}

static void print_version(void) {
    printf("%s\n", YAI_CLI_VERSION);
}

static int is_flag(const char *a, const char *s, const char *l) {
    return (a && ((s && strcmp(a, s) == 0) || (l && strcmp(a, l) == 0)));
}

static int is_global_help(const char *a) {
    return is_flag(a, "-h", "--help") || (a && strcmp(a, "help") == 0);
}

static int is_global_version(const char *a) {
    return is_flag(a, "-V", "--version") || (a && strcmp(a, "version") == 0);
}

int main(int argc, char **argv) {
    // --- global: no args / help / version ---
    if (argc < 2) {
        print_usage();
        return 1;
    }

    if (is_global_help(argv[1])) {
        print_usage();
        return 0;
    }
    if (is_global_version(argv[1])) {
        print_version();
        return 0;
    }

    // --- init opts ---
    yai_cli_opts_t opt;
    memset(&opt, 0, sizeof(opt));
    opt.role = "user";
    opt.client_version = YAI_CLI_VERSION;

    // default workspace from env
    const char *env_ws = getenv("YAI_WORKSPACE");
    opt.ws_id = env_ws;

    // --- parse bin ---
    const char *target_bin = argv[1];
    int i = 2; // parse flags starting after <bin>

    // allow: yai <bin> help   OR  yai <bin> --help
    if (i < argc && is_global_help(argv[i])) {
        // route to dispatcher so it can show per-bin usage if you want
        // but if you prefer global usage, just print_usage() here.
        return yai_cmd_dispatch(target_bin, 1, (char*[]){"help"}, &opt);
    }

    // --- parse flags for that bin ---
    for (; i < argc; i++) {
        const char *a = argv[i];
        if (!a || a[0] != '-') break;

        if (is_flag(a, "-w", "--ws")) {
            if (++i >= argc) { fprintf(stderr, "ERR: --ws needs value\n"); return 2; }
            opt.ws_id = argv[i];
        } else if (is_flag(a, "-j", "--json")) {
            opt.json = 1;
        } else if (is_flag(a, "-a", "--arming")) {
            opt.arming = 1;
        } else if (is_flag(a, "-r", "--role")) {
            if (++i >= argc) { fprintf(stderr, "ERR: --role needs value\n"); return 2; }
            opt.role = argv[i];
        } else if (is_flag(a, "-h", "--help")) {
            // help scoped to bin
            return yai_cmd_dispatch(target_bin, 1, (char*[]){"help"}, &opt);
        } else if (is_flag(a, "-V", "--version")) {
            print_version();
            return 0;
        } else {
            fprintf(stderr, "ERR: unknown option: %s\n", a);
            return 2;
        }
    }

    // --- ADR validation: engine & mind always require ws_id ---
    if (strcmp(target_bin, "engine") == 0 || strcmp(target_bin, "mind") == 0) {
        if (!opt.ws_id || !opt.ws_id[0]) {
            fprintf(stderr, "FATAL: '%s' requires --ws <id> (or YAI_WORKSPACE).\n", target_bin);
            return 3;
        }
    }

    // If no command, show usage (bin scoped would be nicer, but ok)
    if (i >= argc) {
        fprintf(stderr, "ERR: Missing command for binary '%s'.\n", target_bin);
        fprintf(stderr, "Try: yai %s --help\n", target_bin);
        return 2;
    }

    // dispatch remainder
    return yai_cmd_dispatch(target_bin, argc - i, argv + i, &opt);
}
