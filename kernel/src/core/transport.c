#include "transport.h"
#include "kernel.h"
#include "yai_kernel.h"
#include "yai_session.h"
#include <protocol/transport.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>
#include <stdlib.h>

#ifndef YAI_RUNTIME_BACKLOG
#define YAI_RUNTIME_BACKLOG 8
#endif

#define DEFAULT_RUNTIME_SOCKET_PATH "/tmp/yai_runtime.sock"
#define YAI_LEVEL_WARN "warn"

static int server_fd = -1;

static const char *yai_socket_path(void) {
    const char *env_path = getenv("YAI_RUNTIME_SOCKET");
    if (env_path && env_path[0] != '\0') return env_path;
    return DEFAULT_RUNTIME_SOCKET_PATH;
}

static ssize_t read_full(int fd, void *buf, size_t n) {
    size_t got = 0;
    while (got < n) {
        ssize_t r = read(fd, (char *)buf + got, n - got);
        if (r == 0) return (ssize_t)got; 
        if (r < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        got += (size_t)r;
    }
    return (ssize_t)got;
}

static void log_transport_event(const char *event_type, const char *ws_id, const char *trace_id, const char *reason) {
    char msg[192];
    snprintf(msg, sizeof(msg), "TRANSPORT_%s reason=%s actor=kernel", event_type, reason);
    
    yai_log_static(EV_TRANSITION_REJECTED, 
                   (ws_id && ws_id[0]) ? ws_id : "system", 
                   (trace_id && trace_id[0]) ? trace_id : "null", 
                   YAI_LEVEL_WARN, msg, "null");
}

/**
 * DISPATCHER LOGIC - Blocco 2 & 3
 */
static void handle_client_command(int client_fd) {
    yai_rpc_envelope_t env;
    memset(&env, 0, sizeof(env));

    if (read_full(client_fd, &env, sizeof(yai_rpc_envelope_t)) < (ssize_t)sizeof(yai_rpc_envelope_t)) {
        return;
    }

    // Hardening: Magic & Size
    if (env.magic != YAI_FRAME_MAGIC) {
        log_transport_event("REJECTED", "null", "null", "bad_magic");
        return;
    }
    if (env.payload_len > YAI_MAX_PAYLOAD) {
        log_transport_event("REJECTED", env.ws_id, env.trace_id, "oversize_payload");
        return;
    }

    char *payload = NULL;
    if (env.payload_len > 0) {
        payload = malloc(env.payload_len + 1);
        if (!payload) return;
        if (read_full(client_fd, payload, env.payload_len) != (ssize_t)env.payload_len) {
            log_transport_event("REJECTED", env.ws_id, env.trace_id, "truncated_body");
            free(payload);
            return;
        }
        payload[env.payload_len] = '\0';
    }

    if (env.ws_id[0] != '\0') {
        fprintf(stderr, "[KERNEL] Dispatching L2/L3 to WS [%s] CMD [%u]\n", env.ws_id, env.command_id);
        
        // AGGIUNTO client_fd come primo parametro
        yai_session_dispatch(client_fd, env.ws_id, env.command_id, payload);
    } else {
        // INSTRADAMENTO L1 (System Context)
        fprintf(stderr, "[KERNEL] Executing L1 System CMD [%u]\n", env.command_id);
        // Chiamata diretta alla logica di comando del kernel core
        // kernel_execute_system_command(env.command_id, payload);
    }

    if (payload) free(payload);
}

int yai_transport_init_at(const char *path) {
    struct sockaddr_un addr;
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -1;

    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;

    // USIAMO IL PATH PASSATO, NON QUELLO DI FALLBACK
    // Se path è NULL o vuoto, allora (e solo allora) usiamo il fallback
    const char *final_path = (path && path[0] != '\0') ? path : yai_socket_path();
    
    strncpy(addr.sun_path, final_path, sizeof(addr.sun_path) - 1);

    (void)unlink(final_path); // Pulizia socket precedente
    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("[TRANSPORT] bind failed");
        close(fd);
        return -2;
    }

    // Permessi restrittivi: solo l'utente che ha lanciato yai può parlarci
    (void)chmod(final_path, 0600);
    
    if (listen(fd, YAI_RUNTIME_BACKLOG) < 0) {
        close(fd);
        return -3;
    }

    server_fd = fd;
    fprintf(stderr, "[TRANSPORT] Hardened Root Plane UDS Ready: %s\n", final_path);
    return 0;
}

void yai_transport_serve_once(void) {
    if (server_fd < 0) return;
    int client_fd = accept(server_fd, NULL, NULL);
    if (client_fd >= 0) {
        handle_client_command(client_fd);
        close(client_fd);
    }
}