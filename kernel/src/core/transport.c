// kernel/src/core/transport.c
#include "transport.h"
#include "kernel.h"

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

// Phase-1: trace_id non ancora wired da envelope->runtime transport
#define YAI_TRACE_PLACEHOLDER "tr-0"
#define YAI_LEVEL_WARN "warn"

static const char *yai_socket_path(void) {
    const char *env_path = getenv("YAI_RUNTIME_SOCKET");
    if (env_path && env_path[0] != '\0') return env_path;
    return DEFAULT_RUNTIME_SOCKET_PATH;
}

static const char *safe_cstr(const char *s) { return (s && s[0]) ? s : ""; }

// Best-effort ws_id: se non abbiamo vault qui, restiamo vuoti.
// (non inventiamo ws_id; per Phase-2 lo passiamo esplicitamente dal kernel/engine bridge)
static const char *runtime_ws_id(void) {
    return "";
}

static int server_fd = -1;

static ssize_t read_full(int fd, void *buf, size_t n) {
    size_t got = 0;
    while (got < n) {
        ssize_t r = read(fd, (char *)buf + got, n - got);
        if (r == 0) return (ssize_t)got; // EOF
        if (r < 0) {
            if (errno == EINTR) continue;
            return -1;
        }
        got += (size_t)r;
    }
    return (ssize_t)got;
}

static bool is_compliance_relevant_effect(const Packet *pkt) {
    if (!pkt) return false;
    switch (pkt->type) {
        case CMD_AGENT_SPAWN:
        case CMD_STORAGE_SYNC:
        case CMD_SYS_HALT:
            return true;
        case CMD_PING:
        default:
            return false;
    }
}

static bool payload_contains_token(const char *payload, const char *token) {
    return payload && token && strstr(payload, token) != NULL;
}

static bool validate_compliance_context_schema(const char *payload) {
    if (!payload || payload[0] == '\0') return false;

    return payload_contains_token(payload, "pack_ref")
        && payload_contains_token(payload, "purpose_id")
        && payload_contains_token(payload, "data_class")
        && payload_contains_token(payload, "retention_policy_id")
        && payload_contains_token(payload, "legal_basis")
        && payload_contains_token(payload, "subject_scope")
        && payload_contains_token(payload, "processor_role")
        && payload_contains_token(payload, "audit_required");
}

static void log_denied(const char *reason, int cmd_type) {
    char msg[192];
    snprintf(msg, sizeof(msg),
             "EVENT_DENIED reason=%s actor=kernel cmd=%d",
             safe_cstr(reason), cmd_type);

    // Phase-1: ws_id/trace_id not wired here -> stable placeholders
    yai_log_static(EV_TRANSITION_REJECTED, runtime_ws_id(), YAI_TRACE_PLACEHOLDER, YAI_LEVEL_WARN, msg, "null");
}

static int enforce_compliance_context(const Packet *pkt) {
    if (!pkt) return -1;
    if (!is_compliance_relevant_effect(pkt)) return 0;

    if (pkt->payload_size == 0) {
        log_denied("compliance_missing", (int)pkt->type);
        fprintf(stderr, "[RUNTIME] EVENT_DENIED reason=compliance_missing actor=kernel cmd=%d\n", (int)pkt->type);
        return -1;
    }

    // Copy payload as text (defensive, NUL terminate)
    char payload_text[1025];
    size_t copy_len = pkt->payload_size;
    if (copy_len > sizeof(pkt->payload)) copy_len = sizeof(pkt->payload);
    if (copy_len > 1024) copy_len = 1024;

    memcpy(payload_text, pkt->payload, copy_len);
    payload_text[copy_len] = '\0';

    if (!validate_compliance_context_schema(payload_text)) {
        log_denied("compliance_invalid", (int)pkt->type);
        fprintf(stderr, "[RUNTIME] EVENT_DENIED reason=compliance_invalid actor=kernel cmd=%d\n", (int)pkt->type);
        return -1;
    }

    return 0;
}

static void handle_client_command(int client_fd) {
    Packet pkt;
    memset(&pkt, 0, sizeof(pkt));

    ssize_t r = read_full(client_fd, &pkt, sizeof(Packet));
    if (r <= 0) return;

    // Hard cap payload_size
    if (pkt.payload_size > sizeof(pkt.payload)) {
        fprintf(stderr, "[RUNTIME] DROP reason=payload_size_overflow size=%u\n", pkt.payload_size);
        return;
    }

    if (enforce_compliance_context(&pkt) != 0) {
        return;
    }

    fprintf(stderr, "[RUNTIME] Received Command: %d (payload_size=%u)\n", (int)pkt.type, pkt.payload_size);

    switch (pkt.type) {
        case CMD_PING:
            fprintf(stderr, "[RUNTIME] PING received. Runtime is ALIVE.\n");
            break;
        case CMD_AGENT_SPAWN:
            fprintf(stderr, "[RUNTIME] Spawning agent (bytes=%u)\n", pkt.payload_size);
            break;
        case CMD_SYS_HALT:
            fprintf(stderr, "[RUNTIME] System Halt requested.\n");
            break;
        default:
            fprintf(stderr, "[RUNTIME] Unknown command type: %d\n", (int)pkt.type);
            break;
    }
}

int yai_transport_init(void) {
    struct sockaddr_un addr;

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -1;

    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;

    const char *sock_path = yai_socket_path();
    if (strlen(sock_path) >= sizeof(addr.sun_path)) {
        close(fd);
        return -1;
    }
    strncpy(addr.sun_path, sock_path, sizeof(addr.sun_path) - 1);

    (void)unlink(sock_path);

    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -2;
    }

    // keep runtime socket private too
    (void)chmod(sock_path, 0600);

    if (listen(fd, YAI_RUNTIME_BACKLOG) < 0) {
        close(fd);
        return -3;
    }

    server_fd = fd;
    fprintf(stderr, "[TRANSPORT] UDS Socket listening at %s\n", sock_path);
    return 0;
}

void yai_transport_serve_once(void) {
    if (server_fd < 0) return;

    int client_fd = accept(server_fd, NULL, NULL);
    if (client_fd < 0) return;

    handle_client_command(client_fd);
    close(client_fd);
}