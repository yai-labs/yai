// tools/cli/src/paths.c
#include "../include/yai_paths.h"
#include "../include/yai_env.h"

#include <stdio.h>
#include <ctype.h>

static int ws_id_is_safe(const char *ws_id) {
    if (!ws_id || !ws_id[0]) return 0;
    for (const unsigned char *p = (const unsigned char *)ws_id; *p; p++) {
        if (isalnum(*p) || *p == '_' || *p == '-' || *p == '.') continue;
        return 0;
    }
    return 1;
}

/**
 * ADR-002 — Root Control Plane Per Machine (Entry Point)
 *
 * CLI/cockpit MUST connect only to the machine root socket first.
 * Workspace routing happens via protocol (attach/list) not via socket path selection.
 *
 * Default:  $HOME/.yai/run/root.sock
 * Override: YAI_ROOT_SOCK=/abs/path/to/root.sock
 */
int yai_path_root_sock(char *out, size_t cap) {
    if (!out || cap < 8) return -1;

    const char *override = yai_env_get("YAI_ROOT_SOCK", NULL);
    if (override && override[0]) {
        int n = snprintf(out, cap, "%s", override);
        if (n <= 0 || (size_t)n >= cap) return -2;
        return 0;
    }

    const char *home = yai_env_home();
    if (!home) return -3;

    int n = snprintf(out, cap, "%s/.yai/run/root.sock", home);
    if (n <= 0 || (size_t)n >= cap) return -4;
    return 0;
}

/**
 * Legacy compatibility: historically a per-workspace control.sock existed.
 * ADR-002 forbids selecting a workspace by socket path in the CLI.
 * So we hard-map to root.sock.
 */
int yai_path_control_sock(const char *ws_id, char *out, size_t cap) {
    (void)ws_id;
    return yai_path_root_sock(out, cap);
}

/**
 * ADR-001 — Per-workspace storage exists, but the CLI MUST NOT use this to bypass runtime.
 * This helper is allowed only for LOCAL diagnostics / tests / tooling.
 *
 * Default:  $HOME/.yai/run/<ws_id>/semantic.sqlite
 * Override: YAI_WS_DB_NAME=semantic.sqlite
 */
int yai_path_ws_db(const char *ws_id, char *out, size_t cap) {
    if (!out || cap < 8) return -1;
    if (!ws_id_is_safe(ws_id)) return -2;

    const char *home = yai_env_home();
    if (!home) return -3;

    const char *db_name = yai_env_get("YAI_WS_DB_NAME", "semantic.sqlite");

    int n = snprintf(out, cap, "%s/.yai/run/%s/%s", home, ws_id, db_name);
    if (n <= 0 || (size_t)n >= cap) return -4;
    return 0;
}
