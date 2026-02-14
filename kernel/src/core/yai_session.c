#include "yai_session.h"
#include <protocol/transport.h> // La Legge risolta tramite -I
#include <stdlib.h>
#include <fcntl.h>
#include <errno.h>
#include <string.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

// Registro globale delle sessioni attive
yai_session_t g_session_registry[MAX_SESSIONS] = {0};

/* --- LOGICA DI UTILITY --- */

static const char* yai_get_home(void) {
    const char* home = getenv("HOME");
    return (home && strlen(home) > 0) ? home : NULL;
}

static int mkdir_if_missing(const char *path, mode_t mode) {
    struct stat st;
    if (stat(path, &st) == 0) return S_ISDIR(st.st_mode) ? 0 : -1;
    if (mkdir(path, mode) == 0) return 0;
    return -1;
}

static int ensure_run_tree(const char *home) {
    char p1[MAX_PATH_LEN], p2[MAX_PATH_LEN];
    snprintf(p1, sizeof(p1), "%s/.yai", home);
    snprintf(p2, sizeof(p2), "%s/.yai/run", home);
    if (mkdir_if_missing(p1, 0755) != 0) return -1;
    if (mkdir_if_missing(p2, 0755) != 0) return -1;
    return 0;
}

bool yai_ws_validate_id(const char* ws_id) {
    if (!ws_id || strlen(ws_id) == 0 || strlen(ws_id) >= MAX_WS_ID_LEN) return false;
    for (const char *p = ws_id; *p; p++) {
        if (!((*p >= 'a' && *p <= 'z') || (*p >= 'A' && *p <= 'Z') ||
              (*p >= '0' && *p <= '9') || *p == '-' || *p == '_')) return false;
    }
    return true;
}

bool yai_ws_build_paths(yai_workspace_t* ws, const char* ws_id) {
    const char* home = yai_get_home();
    if (!ws || !home || !yai_ws_validate_id(ws_id)) return false;
    memset(ws, 0, sizeof(*ws));
    strncpy(ws->ws_id, ws_id, MAX_WS_ID_LEN - 1);
    snprintf(ws->run_dir, MAX_PATH_LEN, "%s/.yai/run/%s", home, ws_id);
    snprintf(ws->lock_file, MAX_PATH_LEN, "%s/lock", ws->run_dir);
    snprintf(ws->pid_file, MAX_PATH_LEN, "%s/kernel.pid", ws->run_dir);
    ws->state = YAI_WS_CREATED;
    return true;
}

/* --- LOGICA DI DISPATCH (RPC LOOP) --- */

/**
 * Invia una risposta JSON al client sul socket.
 */
static void send_rpc_response(int client_fd, const char *json) {
    if (client_fd < 0 || !json) return;
    ssize_t len = (ssize_t)strlen(json);
    if (write(client_fd, json, len) != len) {
        fprintf(stderr, "[SESSION] Error sending response to fd %d\n", client_fd);
    }
}

/**
 * yai_session_dispatch: Il Router Sovrano.
 * Chiamato da transport.c dopo la validazione dell'envelope.
 */
void yai_session_dispatch(int client_fd, const char* ws_id, uint32_t cmd_id, const char* payload) {
    yai_session_t *s = NULL;

    // 1. Acquisizione o creazione sessione per il Workspace richiesto
    if (!yai_session_acquire(&s, ws_id)) {
        fprintf(stderr, "[SESSION] Access Denied: Workspace %s locked or invalid.\n", ws_id);
        send_rpc_response(client_fd, "{\"status\":\"error\",\"reason\":\"session_denied\"}");
        return;
    }

    // 2. Routing basato sui Command ID definiti nella LAW
    switch (cmd_id) {
        case 0x0102u: // HANDSHAKE
            fprintf(stderr, "[SESSION] Handshake OK for WS: %s\n", ws_id);
            send_rpc_response(client_fd, "{\"status\":\"ok\",\"msg\":\"Sovereign Kernel Online\"}");
            break;

        case 0x0101u: // PING
            send_rpc_response(client_fd, "{\"status\":\"pong\"}");
            break;

        default:
            fprintf(stderr, "[SESSION] Unknown Command 0x%X\n", cmd_id);
            send_rpc_response(client_fd, "{\"status\":\"error\",\"reason\":\"unknown_cmd\"}");
            break;
    }
}

/* --- GESTIONE REGISTRY --- */

bool yai_session_acquire(yai_session_t** out, const char* ws_id) {
    if (!out || !ws_id) return false;

    // Cerca sessione esistente
    for (int i = 0; i < MAX_SESSIONS; i++) {
        if (g_session_registry[i].owner_pid != 0 && strcmp(g_session_registry[i].ws.ws_id, ws_id) == 0) {
            *out = &g_session_registry[i];
            return true;
        }
    }

    // Crea nuova sessione
    for (int i = 0; i < MAX_SESSIONS; i++) {
        if (g_session_registry[i].owner_pid == 0) {
            yai_workspace_t ws;
            if (!yai_ws_build_paths(&ws, ws_id)) return false;
            
            ensure_run_tree(yai_get_home());
            mkdir_if_missing(ws.run_dir, 0755);

            // Tenant Isolation via Lockfile
            int fd = open(ws.lock_file, O_CREAT | O_EXCL | O_RDWR, 0600);
            if (fd < 0) return false; 
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

void yai_session_release(yai_session_t* s) {
    if (!s) return;
    unlink(s->ws.lock_file);
    memset(s, 0, sizeof(*s));
}