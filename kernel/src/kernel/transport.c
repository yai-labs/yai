#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include "transport.h"
#include "kernel.h"

#define SOCKET_PATH "/tmp/yai_runtime.sock"

static const char *yai_socket_path(void) {
    const char *env_path = getenv("YAI_RUNTIME_SOCKET");
    if (env_path && env_path[0] != '\0') {
        return env_path;
    }
    return SOCKET_PATH;
}

static int server_fd = -1;

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
    if (!payload || payload[0] == '\0') {
        return false;
    }

    return payload_contains_token(payload, "pack_ref")
        && payload_contains_token(payload, "purpose_id")
        && payload_contains_token(payload, "data_class")
        && payload_contains_token(payload, "retention_policy_id")
        && payload_contains_token(payload, "legal_basis")
        && payload_contains_token(payload, "subject_scope")
        && payload_contains_token(payload, "processor_role")
        && payload_contains_token(payload, "audit_required");
}

static int enforce_compliance_context(const Packet *pkt) {
    size_t copy_len;
    char payload_text[1025];

    if (!pkt) return -1;
    if (!is_compliance_relevant_effect(pkt)) return 0;

    if (pkt->payload_size == 0) {
        yai_log_static(EV_TRANSITION_REJECTED, "EVENT_DENIED reason=compliance_missing actor=kernel");
        fprintf(stderr, "[RUNTIME] EVENT_DENIED reason=compliance_missing actor=kernel cmd=%d\n", pkt->type);
        return -1;
    }

    copy_len = pkt->payload_size;
    if (copy_len > sizeof(pkt->payload)) {
        copy_len = sizeof(pkt->payload);
    }
    memcpy(payload_text, pkt->payload, copy_len);
    payload_text[copy_len] = '\0';

    if (!validate_compliance_context_schema(payload_text)) {
        yai_log_static(EV_TRANSITION_REJECTED, "EVENT_DENIED reason=compliance_invalid actor=kernel");
        fprintf(stderr, "[RUNTIME] EVENT_DENIED reason=compliance_invalid actor=kernel cmd=%d\n", pkt->type);
        return -1;
    }

    return 0;
}

void handle_client_command(int client_fd) {
    Packet pkt;
    ssize_t bytes = read(client_fd, &pkt, sizeof(Packet));

    if (bytes > 0) {
        if (enforce_compliance_context(&pkt) != 0) {
            return;
        }

        printf("[RUNTIME] Received Command: %d (Size: %u)\n", pkt.type, pkt.payload_size);

        switch (pkt.type) {
            case CMD_PING:
                printf("[RUNTIME] PING received. Runtime is ALIVE.\n");
                break;
            case CMD_AGENT_SPAWN:
                printf("[RUNTIME] Spawning agent: %s\n", pkt.payload);
                break;
            case CMD_SYS_HALT:
                printf("[RUNTIME] System Halt requested.\n");
                break;
            default:
                printf("[RUNTIME] Unknown command type: %d\n", pkt.type);
        }
    }
}

int yai_transport_init(void) {
    struct sockaddr_un addr;

    server_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (server_fd < 0) return -1;

    memset(&addr, 0, sizeof(struct sockaddr_un));
    addr.sun_family = AF_UNIX;
    const char *sock_path = yai_socket_path();
    strncpy(addr.sun_path, sock_path, sizeof(addr.sun_path) - 1);

    unlink(sock_path);
    if (bind(server_fd, (struct sockaddr *)&addr, sizeof(struct sockaddr_un)) < 0) return -2;
    if (listen(server_fd, 5) < 0) return -3;

    printf("[TRANSPORT] UDS Socket listening at %s\n", sock_path);
    return 0;
}

void yai_transport_serve_once(void) {
    if (server_fd < 0) return;

    int client_fd = accept(server_fd, NULL, NULL);
    if (client_fd < 0) return;

    handle_client_command(client_fd);
    close(client_fd);
}
