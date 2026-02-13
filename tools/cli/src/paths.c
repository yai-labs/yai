// tools/cli/src/paths.c
#include "../include/yai_paths.h"
#include "../include/yai_env.h"
#include <stdio.h>

int yai_path_control_sock(const char *ws_id, char *out, size_t cap) {
    if (!ws_id || !ws_id[0] || !out || cap < 8) return -1;
    const char *home = yai_env_home();
    if (!home) return -2;

    int n = snprintf(out, cap, "%s/.yai/run/%s/control.sock", home, ws_id);
    if (n <= 0 || (size_t)n >= cap) return -3;
    return 0;
}
