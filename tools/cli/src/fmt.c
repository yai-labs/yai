// tools/cli/src/fmt.c
#include "../include/yai_fmt.h"
#include <stdio.h>
#include <string.h>

void yai_print_response(const char *json_line, int json_mode) {
    if (!json_line) return;

    if (json_mode) {
        fputs(json_line, stdout);
        if (json_line[0] && json_line[strlen(json_line) - 1] != '\n') fputc('\n', stdout);
        return;
    }

    // Human mode: tiny heuristic (no JSON parser)
    // Print the whole line but with a prefix.
    fprintf(stdout, "[yai] %s", json_line);
    if (json_line[0] && json_line[strlen(json_line) - 1] != '\n') fputc('\n', stdout);
}
