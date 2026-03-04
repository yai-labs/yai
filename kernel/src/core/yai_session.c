#define _POSIX_C_SOURCE 200809L

#include "yai_session.h"
#include "control_transport.h"
#include "ws_id.h"

#include <transport.h>
#include <yai_protocol_ids.h>
#include <protocol.h>

#include <stdlib.h>
#include <fcntl.h>
#include <errno.h>
#include <string.h>
#include <stdio.h>
#include <signal.h>
#include <unistd.h>
#include <sys/stat.h>
#include <unistd.h>
#include <signal.h>

/* ============================================================
   GLOBAL REGISTRY
   ============================================================ */

yai_session_t g_session_registry[MAX_SESSIONS] = {0};

/* ============================================================
   INTERNAL UTIL
   ============================================================ */

static const char *yai_get_home(void)
{
    const char *home = getenv("HOME");
    return (home && strlen(home) > 0) ? home : NULL;
}

static int mkdir_if_missing(const char *path, mode_t mode)
{
    struct stat st;
    if (stat(path, &st) == 0)
        return S_ISDIR(st.st_mode) ? 0 : -1;

    return mkdir(path, mode);
}

static int ensure_run_tree(const char *home)
{
    char p1[MAX_PATH_LEN], p2[MAX_PATH_LEN];

    snprintf(p1, sizeof(p1), "%s/.yai", home);
    snprintf(p2, sizeof(p2), "%s/.yai/run", home);

    if (mkdir_if_missing(p1, 0755) != 0)
        return -1;
    if (mkdir_if_missing(p2, 0755) != 0)
        return -1;

    return 0;
}

/* ============================================================
   WORKSPACE
   ============================================================ */

bool yai_ws_validate_id(const char *ws_id)
{
    return yai_ws_id_is_valid(ws_id);
}

bool yai_ws_build_paths(yai_workspace_t *ws, const char *ws_id)
{
    const char *home = yai_get_home();
    if (!ws || !home || !yai_ws_validate_id(ws_id))
        return false;

    memset(ws, 0, sizeof(*ws));

    strncpy(ws->ws_id, ws_id, MAX_WS_ID_LEN - 1);

    snprintf(ws->run_dir, MAX_PATH_LEN,
             "%s/.yai/run/%s", home, ws_id);

    snprintf(ws->lock_file, MAX_PATH_LEN,
             "%s/lock", ws->run_dir);

    snprintf(ws->pid_file, MAX_PATH_LEN,
             "%s/kernel.pid", ws->run_dir);

    ws->state = YAI_WS_CREATED;
    return true;
}

/* ============================================================
   SESSION ACQUIRE
   ============================================================ */

bool yai_session_acquire(yai_session_t **out, const char *ws_id)
{
    if (!out || !ws_id)
        return false;

    /* 1️⃣ Already active */

    for (int i = 0; i < MAX_SESSIONS; i++)
    {
        if (g_session_registry[i].owner_pid != 0 &&
            strcmp(g_session_registry[i].ws.ws_id, ws_id) == 0)
        {
            *out = &g_session_registry[i];
            return true;
        }
    }

    /* 2️⃣ Allocate new slot */

    for (int i = 0; i < MAX_SESSIONS; i++)
    {
        if (g_session_registry[i].owner_pid == 0)
        {
            yai_workspace_t ws;

            if (!yai_ws_build_paths(&ws, ws_id))
                return false;

            ensure_run_tree(yai_get_home());
            mkdir_if_missing(ws.run_dir, 0755);

            int fd = open(ws.lock_file,
                          O_CREAT | O_EXCL | O_RDWR,
                          0600);

            if (fd < 0)
            {
                if (errno == EEXIST)
                {
                    FILE *f = fopen(ws.lock_file, "r");
                    if (!f)
                        return false;

                    pid_t old_pid = 0;
                    fscanf(f, "%d", &old_pid);
                    fclose(f);

                    if (old_pid > 0 && kill(old_pid, 0) == 0)
                        return false;

                    unlink(ws.lock_file);

                    fd = open(ws.lock_file,
                              O_CREAT | O_EXCL | O_RDWR,
                              0600);

                    if (fd < 0)
                        return false;
                }
                else
                    return false;
            }

            dprintf(fd, "%d\n", getpid());
            close(fd);

            g_session_registry[i].ws = ws;
            g_session_registry[i].owner_pid = (uint32_t)getpid();
            g_session_registry[i].session_id = (uint32_t)i;

            *out = &g_session_registry[i];
            return true;
        }
    }

    return false;
}

void yai_session_release(yai_session_t *s)
{
    if (!s)
        return;

    unlink(s->ws.lock_file);
    memset(s, 0, sizeof(*s));
}

/* ============================================================
   RESPONSE HELPER
   ============================================================ */

static void send_binary_response(
    int fd,
    const yai_rpc_envelope_t *req,
    uint32_t command_id,
    const char *json_payload)
{
    yai_rpc_envelope_t resp;
    memset(&resp, 0, sizeof(resp));

    resp.magic = YAI_FRAME_MAGIC;
    resp.version = YAI_PROTOCOL_IDS_VERSION;
    resp.command_id = command_id;

    strncpy(resp.ws_id, req->ws_id, sizeof(resp.ws_id) - 1);
    strncpy(resp.trace_id, req->trace_id, sizeof(resp.trace_id) - 1);

    resp.payload_len = (uint32_t)strlen(json_payload);

    yai_control_write_frame(fd, &resp, json_payload);
}

/* ============================================================
   SESSION DISPATCH
   ============================================================ */

void yai_session_dispatch(
    int client_fd,
    const yai_rpc_envelope_t *env,
    const char *payload)
{
    (void)payload;

    if (!env)
        return;

    if (env->ws_id[0] == '\0' || strlen(env->ws_id) == 0) {
        send_binary_response(
            client_fd,
            env,
            env->command_id,
            "{\"status\":\"error\",\"reason\":\"ws_required\"}");
        return;
    }

    yai_session_t *s = NULL;

    if (!yai_session_acquire(&s, env->ws_id))
    {
        send_binary_response(
            client_fd,
            env,
            env->command_id,
            "{\"status\":\"error\",\"reason\":\"session_denied\"}");
        return;
    }

    switch (env->command_id)
    {
    case YAI_CMD_PING:
        send_binary_response(
            client_fd,
            env,
            YAI_CMD_PING,
            "{\"status\":\"pong\"}");
        break;

    default:
        send_binary_response(
            client_fd,
            env,
            env->command_id,
            "{\"status\":\"ok\"}");
        break;
    }
}
