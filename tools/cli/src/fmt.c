// tools/cli/src/fmt.c
#include "../include/yai_fmt.h"
#include <stdio.h>
#include <string.h>

static void print_line_ensure_nl(const char *s) {
    if (!s) return;
    fputs(s, stdout);
    size_t n = strlen(s);
    if (n == 0 || s[n - 1] != '\n') fputc('\n', stdout);
}

void yai_print_response(const char *json_line, int json_mode) {
    if (!json_line) return;

    if (json_mode) {
        print_line_ensure_nl(json_line);
        return;
    }

    // Human mode: prefix + line (no JSON parsing; keep deterministic)
    fputs("[yai] ", stdout);
    print_line_ensure_nl(json_line);
}
