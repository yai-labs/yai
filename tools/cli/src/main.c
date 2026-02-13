// tools/cli/src/main.c
#include "../include/yai_cli.h"
#include "../include/yai_cmd.h"

#include <stdio.h>
#include <string.h>

static int is_flag(const char *a, const char *b) { return a && b && strcmp(a,b)==0; }

int yai_cli_main(int argc, char **argv) {
    yai_cli_opts_t opt;
    memset(&opt, 0, sizeof(opt));
    opt.role = "user";
    opt.arming = 0;
    opt.json = 0;
    opt.client_version = "cli-c/0.1";

    int i = 1;
    for (; i < argc; i++) {
        const char *a = argv[i];
        if (!a) continue;

        if (is_flag(a, "--ws")) {
            if (i + 1 >= argc) { fprintf(stderr, "ERR: --ws requires value\n"); return 2; }
            opt.ws_id = argv[++i];
            continue;
        }
        if (is_flag(a, "--arming")) { opt.arming = 1; continue; }
        if (is_flag(a, "--json")) { opt.json = 1; continue; }
        if (is_flag(a, "--role")) {
            if (i + 1 >= argc) { fprintf(stderr, "ERR: --role requires value\n"); return 2; }
            opt.role = argv[++i];
            continue;
        }
        if (is_flag(a, "--client")) {
            if (i + 1 >= argc) { fprintf(stderr, "ERR: --client requires value\n"); return 2; }
            opt.client_version = argv[++i];
            continue;
        }
        // first non-flag starts command tail
        break;
    }

    if (i >= argc) {
        fprintf(stderr, "ERR: missing command. Try: yai --ws dev ping\n");
        return 2;
    }

    return yai_cmd_dispatch(argc - i, argv + i, &opt);
}

int main(int argc, char **argv) {
    return yai_cli_main(argc, argv);
}
