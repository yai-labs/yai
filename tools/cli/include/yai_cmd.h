#ifndef YAI_CMD_H
#define YAI_CMD_H

#include "yai_cli.h"

// Top-level dispatcher (root)
int yai_cmd_dispatch(const char *binary, int argc, char **argv, const yai_cli_opts_t *opt);

// L1: Kernel (global)
int yai_cmd_kernel(int argc, char **argv, const yai_cli_opts_t *opt);

// L1: Workspace management (still kernel authority, but acts on ws registry)
int yai_cmd_ws(int argc, char **argv, const yai_cli_opts_t *opt);

// L2: Engine gates
int yai_cmd_engine(int argc, char **argv, const yai_cli_opts_t *opt);

// L3: Mind
int yai_cmd_mind(int argc, char **argv, const yai_cli_opts_t *opt);

// LAW: specs / validation / printing
int yai_cmd_law(int argc, char **argv, const yai_cli_opts_t *opt);

// Orchestrator: bring up runtime (boot + kernel + engine)
int yai_cmd_up(int argc, char **argv, const yai_cli_opts_t *opt);

#endif
