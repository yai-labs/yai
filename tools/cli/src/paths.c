// tools/cli/src/paths.c
#include "../include/yai_paths.h"
#include "../include/yai_env.h"
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

static int file_exists(const char *p) {
    struct stat st;
    return (p && p[0] && stat(p, &st) == 0 && S_ISSOCK(st.st_mode));
}

/**
 * Root Control Plane socket (ADR-002).
 * Transitional compatibility:
 * 1) If YAI_ROOT_SOCK is set, use it.
 * 2) Prefer ~/.yai/run/root.sock (target ADR)
 * 3) Fallback to ~/.yai/run/default/control.sock (current kernel)
 */
int yai_path_root_sock(char *out, size_t cap) {
    if (!out || cap < 8) return -1;

    const char *override = yai_env_get("YAI_ROOT_SOCK", NULL);
    if (override && override[0]) {
        int n = snprintf(out, cap, "%s", override);
        return (n > 0 && (size_t)n < cap) ? 0 : -2;
    }

    const char *home = yai_env_home();
    if (!home) return -3;

    char cand1[512];
    char cand2[512];

    // ADR target
    snprintf(cand1, sizeof(cand1), "%s/.yai/run/root.sock", home);
    // Current kernel
    snprintf(cand2, sizeof(cand2), "%s/.yai/run/default/control.sock", home);

    const char *pick = NULL;
    if (file_exists(cand1)) pick = cand1;
    else                    pick = cand2; // fallback (even if not present yet)

    int n = snprintf(out, cap, "%s", pick);
    return (n > 0 && (size_t)n < cap) ? 0 : -4;
}

int yai_path_ws_db(const char *ws_id, char *out, size_t cap) {
    if (!ws_id || !ws_id[0] || !out || cap < 8) return -1;
    const char *home = yai_env_home();
    if (!home) return -2;

    int n = snprintf(out, cap, "%s/.yai/run/%s/semantic.sqlite", home, ws_id);
    if (n <= 0 || (size_t)n >= cap) return -3;
    return 0;
}

int yai_path_control_sock(const char *ws_id, char *out, size_t cap) {
    (void)ws_id;
    return yai_path_root_sock(out, cap);
}
