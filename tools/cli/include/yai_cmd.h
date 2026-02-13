// tools/cli/include/yai_cmd.h
#ifndef YAI_CMD_H
#define YAI_CMD_H

#include "yai_cli.h"

// top-level dispatch
int yai_cmd_dispatch(int argc, char **argv, const yai_cli_opts_t *opt);

#endif
