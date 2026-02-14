#include "../../include/transport_client.h"
#include "../../law/specs/protocol/transport.h"
#include "../../law/specs/protocol/yai_protocol_ids.h"
#include <stdbool.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

/**
 * Risoluzione path del socket basata sulla gerarchia Sovereign.
 * L'Engine cerca il socket di controllo nel workspace specifico.
 */
static int build_control_sock_path(const char *ws_id, char *out, size_t cap) {
    const char *home = getenv("HOME");
    if (!home) return -1;
    // Percorso standard definito in ADR-001
    snprintf(out, cap, "%s/.yai/run/%s/control.sock", home, ws_id);
    return 0;
}

/**
 * Generatore di Trace ID deterministico per il Blocco 4.
 * Permette l'audit end-to-end della richiesta.
 */
void yai_make_trace_id(char out[36]) {
    static uint32_t ctr = 0;
    // Format: tr-<timestamp>-<counter>
    snprintf(out, 36, "tr-%lx-%u", (unsigned long)time(NULL), ctr++);
}

/**
 * Stabilisce la connessione fisica verso il Root Plane.
 */
int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id) {
    if (!c || !ws_id) return -1;
    memset(c, 0, sizeof(*c));
    
    char sock_path[256];
    if (build_control_sock_path(ws_id, sock_path, sizeof(sock_path)) < 0) return -1;

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -2;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, sock_path, sizeof(addr.sun_path) - 1);

    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -3;
    }

    c->fd = fd;
    strncpy(c->ws_id, ws_id, 35);
    c->connected = true;
    return 0;
}

/**
 * Esegue la chiamata RPC atomica (Hardened).
 * Invia l'envelope da 96 byte seguito dal payload JSON.
 */
int yai_rpc_call(
    yai_rpc_client_t *c,
    const yai_rpc_envelope_t *env, 
    const char *json_payload,      
    char *out_response,           
    size_t out_cap
) {
    if (!c || !c->connected) return -1;

    // 1. Invio l'Envelope (96 byte fissi) - Deve superare il Magic check del Kernel
    if (write(c->fd, env, sizeof(yai_rpc_envelope_t)) != (ssize_t)sizeof(yai_rpc_envelope_t)) {
        fprintf(stderr, "[TRANSPORT_CLIENT] TX_FAILED: Envelope write error\n");
        return -2;
    }

    // 2. Invio il Payload (solo se autorizzato dall'envelope)
    if (env->payload_len > 0 && json_payload) {
        if (write(c->fd, json_payload, env->payload_len) != (ssize_t)env->payload_len) {
            fprintf(stderr, "[TRANSPORT_CLIENT] TX_FAILED: Payload write error\n");
            return -3;
        }
    }

    // 3. Lettura risposta (Audit log TX_SENT implícito se arriviamo qui)
    ssize_t r = read(c->fd, out_response, out_cap - 1);
    if (r < 0) {
        fprintf(stderr, "[TRANSPORT_CLIENT] RX_FAILED: No response from Kernel\n");
        return -4;
    }
    out_response[r] = '\0';

    return 0;
}

/**
 * Protocol Handshake per validare versioni e capacità.
 */
int yai_rpc_handshake(yai_rpc_client_t *c, uint32_t capabilities) {
    yai_rpc_envelope_t env;
    memset(&env, 0, sizeof(env));

    // Parametri critici per il superamento del filtro del Kernel
    env.magic = YAI_FRAME_MAGIC; 
    env.version = 1;
    env.command_id = 0x0102u; // YAI_CMD_HANDSHAKE da yai_protocol_ids.h
    
    strncpy(env.ws_id, c->ws_id, 35);
    yai_make_trace_id(env.trace_id);
    
    char payload[128];
    snprintf(payload, sizeof(payload), "{\"caps\":%u,\"client\":\"yai-engine-v1\"}", capabilities);
    env.payload_len = (uint32_t)strlen(payload);

    char resp[1024];
    int rc = yai_rpc_call(c, &env, payload, resp, sizeof(resp));
    
    if (rc == 0) {
        printf("[TRANSPORT_CLIENT] Handshake OK: %s\n", resp);
    }
    return rc;
}

void yai_rpc_close(yai_rpc_client_t *c) {
    if (c && c->connected) {
        close(c->fd);
        c->fd = -1;
        c->connected = false;
    }
}