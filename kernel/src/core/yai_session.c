// kernel/src/core/yai_session.c
#include "yai_session.h"
#include <stdlib.h>
#include <fcntl.h>
#include <errno.h>
#include <string.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

yai_session_t g_session_registry[MAX_SESSIONS] = {0};

static const char* yai_get_home(void) {
    const char* home = getenv("HOME");
    if (!home || strlen(home) == 0) return NULL;
    return home;
}

static int mkdir_if_missing(const char *path, mode_t mode) {
    struct stat st;
    if (stat(path, &st) == 0) return S_ISDIR(st.st_mode) ? 0 : -1;
    if (mkdir(path, mode) == 0) return 0;
    return -1;
}

// mkdir -p for ~/.yai/run/<ws>
static int ensure_run_tree(const char *home) {
    char p1[MAX_PATH_LEN];
    char p2[MAX_PATH_LEN];

    snprintf(p1, sizeof(p1), "%s/.yai", home);
    snprintf(p2, sizeof(p2), "%s/.yai/run", home);

    if (mkdir_if_missing(p1, 0700) != 0) return -1;
    if (mkdir_if_missing(p2, 0700) != 0) return -1;
    return 0;
}

bool yai_ws_validate_id(const char* ws_id) {
    if (!ws_id) return false;

    size_t len = strlen(ws_id);
    if (len == 0 || len >= MAX_WS_ID_LEN) return false;

    for (size_t i = 0; i < len; ++i) {
        char c = ws_id[i];
        if (!(c >= 'a' && c <= 'z') &&
            !(c >= 'A' && c <= 'Z') &&
            !(c >= '0' && c <= '9') &&
            c != '-' && c != '_') {
            return false;
        }
    }
    return true;
}

bool yai_ws_build_paths(yai_workspace_t* ws, const char* ws_id) {
    if (!ws || !yai_ws_validate_id(ws_id)) return false;

    const char* home = yai_get_home();
    if (!home) return false;

    memset(ws, 0, sizeof(*ws));
    strncpy(ws->ws_id, ws_id, MAX_WS_ID_LEN - 1);

    // canonical: ~/.yai/run/<ws>/
    snprintf(ws->run_dir, MAX_PATH_LEN, "%s/.yai/run/%s", home, ws_id);
    snprintf(ws->control_sock, MAX_PATH_LEN, "%s/control.sock", ws->run_dir);
    snprintf(ws->lock_file, MAX_PATH_LEN, "%s/lock", ws->run_dir);
    snprintf(ws->pid_file, MAX_PATH_LEN, "%s/daemon.pid", ws->run_dir);

    ws->state = YAI_WS_CREATED;
    return true;
}

bool yai_session_ensure_run_dir(const yai_workspace_t* ws) {
    if (!ws) return false;

    const char* home = yai_get_home();
    if (!home) return false;

    if (ensure_run_tree(home) != 0) return false;

    struct stat st = {0};
    if (stat(ws->run_dir, &st) == -1) {
        if (mkdir(ws->run_dir, 0700) != 0) return false;
    } else if (!S_ISDIR(st.st_mode)) {
        return false;
    }

    return true;
}

bool yai_workspace_try_lock(const yai_workspace_t* ws) {
    if (!ws) return false;

    int fd = open(ws->lock_file, O_CREAT | O_EXCL | O_RDWR, 0600);
    if (fd < 0) {
        if (errno == EEXIST) return false;
        return false;
    }
    close(fd);
    return true;
}

void yai_workspace_unlock(const yai_workspace_t* ws) {
    if (!ws) return;
    (void)unlink(ws->lock_file);
}

bool yai_workspace_write_pid(const yai_workspace_t* ws) {
    if (!ws) return false;
    FILE* f = fopen(ws->pid_file, "w");
    if (!f) return false;
    fprintf(f, "%d\n", getpid());
    fclose(f);
    return true;
}

static yai_session_t* find_free_slot(void) {
    for (int i = 0; i < MAX_SESSIONS; i++) {
        if (g_session_registry[i].owner_pid == 0) return &g_session_registry[i];
    }
    return NULL;
}

bool yai_session_acquire(yai_session_t** out, const char* ws_id) {
    if (!out || !ws_id) return false;

    yai_workspace_t ws;
    if (!yai_ws_build_paths(&ws, ws_id)) return false;

    yai_session_t* s = find_free_slot();
    if (!s) return false;

    memset(s, 0, sizeof(*s));
    s->session_id = (uint32_t)(s - g_session_registry);
    s->run_id = 1; // Phase-1: monotonic semplice (incrementerai in futuro)
    s->ws = ws;
    s->owner_pid = (uint32_t)getpid();

    // Phase-1 caps
    s->caps = YAI_CAP_RPC_PING | YAI_CAP_RPC_HANDSHAKE | YAI_CAP_RPC_STATUS;

    *out = s;
    return true;
}

void yai_session_release(yai_session_t* s) {
    if (!s) return;
    memset(s, 0, sizeof(*s));
}
