// tools/cli/include/yai_cli.h
#ifndef YAI_CLI_H
#define YAI_CLI_H

#include <stddef.h>

typedef struct {
    const char *ws_id;          // workspace target
    const char *role;           // "user" | "operator"
    int arming;                 // 0|1
    int json;                   // 0 human | 1 raw json line
    const char *client_version; // e.g. "cli-c/0.1"
} yai_cli_opts_t;

int yai_cli_main(int argc, char **argv);

#endif
