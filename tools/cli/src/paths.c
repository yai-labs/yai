#include "../include/yai_paths.h"
#include "../include/yai_env.h"

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

/* ============================================================
   INTERNAL VALIDATION
   ============================================================ */

static int is_valid_ws_id(const char *ws_id)
{
    if (!ws_id || !ws_id[0])
        return 0;

    /* no path separators */
    if (strchr(ws_id, '/'))
        return 0;

    /* no home shortcut */
    if (ws_id[0] == '~')
        return 0;

    /* no parent traversal */
    if (strstr(ws_id, ".."))
        return 0;

    return 1;
}

/* ============================================================
   ROOT SOCKET (Machine Plane)
   ============================================================ */

int yai_path_root_sock(char *out, size_t cap)
{
    if (!out || cap < 64)
        return -1;

    const char *override = yai_env_get("YAI_ROOT_SOCK", NULL);
    if (override && override[0])
    {
        int n = snprintf(out, cap, "%s", override);
        return (n > 0 && (size_t)n < cap) ? 0 : -2;
    }

    const char *home = yai_env_home();
    if (!home)
        return -3;

    int n = snprintf(out,
                     cap,
                     "%s/.yai/run/root.sock",
                     home);

    return (n > 0 && (size_t)n < cap) ? 0 : -4;
}

/* ============================================================
   WORKSPACE SOCKET (Tenant Plane)
   ============================================================ */

int yai_path_ws_sock(const char *ws_id, char *out, size_t cap)
{
    if (!is_valid_ws_id(ws_id))
        return -1;

    if (!out || cap < 64)
        return -2;

    const char *home = yai_env_home();
    if (!home)
        return -3;

    int n = snprintf(out,
                     cap,
                     "%s/.yai/run/%s/control.sock",
                     home,
                     ws_id);

    return (n > 0 && (size_t)n < cap) ? 0 : -4;
}

/* ============================================================
   WORKSPACE RUN DIR
   ============================================================ */

int yai_path_ws_run_dir(const char *ws_id, char *out, size_t cap)
{
    if (!is_valid_ws_id(ws_id))
        return -1;

    if (!out || cap < 64)
        return -2;

    const char *home = yai_env_home();
    if (!home)
        return -3;

    int n = snprintf(out,
                     cap,
                     "%s/.yai/run/%s",
                     home,
                     ws_id);

    return (n > 0 && (size_t)n < cap) ? 0 : -4;
}

/* ============================================================
   WORKSPACE DB
   ============================================================ */

int yai_path_ws_db(const char *ws_id, char *out, size_t cap)
{
    if (!is_valid_ws_id(ws_id))
        return -1;

    if (!out || cap < 64)
        return -2;

    const char *home = yai_env_home();
    if (!home)
        return -3;

    int n = snprintf(out,
                     cap,
                     "%s/.yai/run/%s/semantic.sqlite",
                     home,
                     ws_id);

    return (n > 0 && (size_t)n < cap) ? 0 : -4;
}
