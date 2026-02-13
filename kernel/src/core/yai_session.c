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
    if (!home || strlen(home) == 0) {
        return NULL;
    }
    return home;
}


bool yai_ws_validate_id(const char* ws_id) {
    if (!ws_id) return false;

    size_t len = strlen(ws_id);
    if (len == 0 || len >= MAX_WS_ID_LEN)
        return false;

    for (size_t i = 0; i < len; ++i) {
        char c = ws_id[i];
        if (!(c >= 'a' && c <= 'z') &&
            !(c >= 'A' && c <= 'Z') &&
            !(c >= '0' && c <= '9') &&
            c != '-' && c != '_')
            return false;
    }

    return true;
}

bool yai_ws_build_paths(yai_workspace_t* ws, const char* ws_id) {
    if (!ws || !yai_ws_validate_id(ws_id))
        return false;

    const char* home = yai_get_home();
    if (!home)
        return false;

    memset(ws, 0, sizeof(*ws));
    strncpy(ws->ws_id, ws_id, MAX_WS_ID_LEN - 1);

    snprintf(ws->run_dir, MAX_PATH_LEN,
             "%s/.yai/run/%s", home, ws_id);

    snprintf(ws->control_sock, MAX_PATH_LEN,
             "%s/control.sock", ws->run_dir);

    snprintf(ws->lock_file, MAX_PATH_LEN,
             "%s/lock", ws->run_dir);

    snprintf(ws->pid_file, MAX_PATH_LEN,
             "%s/daemon.pid", ws->run_dir);

    ws->state = YAI_WS_CREATED;

    return true;
}

bool yai_session_ensure_run_dir(const yai_workspace_t* ws) {
    if (!ws) return false;

    struct stat st = {0};
    if (stat(ws->run_dir, &st) == -1) {
        if (mkdir(ws->run_dir, 0700) != 0) {
            return false;
        }
    }

    return true;
}

bool yai_workspace_try_lock(const yai_workspace_t* ws) {
    if (!ws) return false;

    int fd = open(ws->lock_file, O_CREAT | O_EXCL | O_RDWR, 0600);
    if (fd < 0) {
        if (errno == EEXIST) {
            return false; // già running
        }
        return false;
    }

    close(fd);
    return true;
}

void yai_workspace_unlock(const yai_workspace_t* ws) {
    if (!ws) return;
    unlink(ws->lock_file);
}

bool yai_workspace_write_pid(const yai_workspace_t* ws) {
    if (!ws) return false;

    FILE* f = fopen(ws->pid_file, "w");
    if (!f) return false;

    fprintf(f, "%d\n", getpid());
    fclose(f);
    return true;
}
