#include <stdio.h>
#include <signal.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include <time.h>
#include <sys/select.h>

// Header di Progetto
#include "../include/shared_constants.h"
#include "../include/engine_bridge.h"
#include "../include/engine_cortex.h"
#include "../include/transport_client.h"
#include "../include/storage_gate.h"
#include "../include/rpc_router.h"

// Header della LAW
#include "../../law/specs/protocol/transport.h"
#include "../../law/specs/protocol/yai_protocol_ids.h"

static volatile int keep_running = 1;

typedef struct {
    Vault* core;
    Vault* brain;
} VaultCluster;

static void handle_signal(int sig) {
    (void)sig;
    keep_running = 0;
}

static void handle_panic(int sig) {
    fprintf(stderr, "\n[PANIC] Signal %d: Locking Vault & Emergency Exit.\n", sig);
    Vault* v = yai_get_vault();
    if (v) {
        v->authority_lock = true;
        v->status = 4; // ERROR state
    }
    yai_storage_shutdown();
    _exit(1);
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "USAGE: ./yai-engine <workspace_id>\n");
        return 1;
    }

    const char *ws_id = argv[1];

    signal(SIGINT,  handle_signal);
    signal(SIGTERM, handle_signal);
    signal(SIGSEGV, handle_panic);

    // 1. Storage L2 - Inizializzazione Globale (Registro Connessioni)
    // Non passiamo più ws_id perché la connessione è aperta in lazy-loading
    yai_storage_init(); 

    // 2. Vault SHM
    VaultCluster cluster;
    cluster.core = yai_bridge_attach(ws_id, "CORE");
    if (!cluster.core) {
        fprintf(stderr, "FATAL: Vault attach failed for ws=%s\n", ws_id);
        return 1;
    }
    cluster.brain = yai_bridge_attach(ws_id, "brain");

    // 3. RPC Control Plane (Canale Sovereign verso il Kernel)
    yai_rpc_client_t rpc_client;
    bool rpc_active = false;
    
    // Inizializziamo la struttura client
    memset(&rpc_client, 0, sizeof(rpc_client));
    
    if (yai_rpc_connect(&rpc_client, ws_id) == 0) {
        fprintf(stderr, "[RPC] Control socket connected to Kernel.\n");
        // Handshake con ID dal nuovo yai_protocol_ids.h
        yai_rpc_handshake(&rpc_client, YAI_CMD_HANDSHAKE);
        rpc_active = true;
    } else {
        fprintf(stderr, "[RPC] Warning: Kernel connection unavailable. Standing by.\n");
    }

    fprintf(stderr, "[ENGINE] Sovereign L2 Online (ws=%s)\n", ws_id);

    uint32_t last_shm_seq = cluster.core->command_seq;

    // 4. Main Loop
    while (keep_running) {
        
        // CANALE A: State/Legacy (Vault via SHM)
        if (cluster.core->command_seq != last_shm_seq) {
            last_shm_seq = cluster.core->command_seq;
            fprintf(stderr, "[SHM] Command from SHM: 0x%x\n", cluster.core->last_command_id);
            cluster.core->last_processed_seq = last_shm_seq;
        }

        // CANALE B: Inference/Sovereign (Socket via RPC)
        if (rpc_active && rpc_client.fd >= 0) {
            fd_set read_fds;
            struct timeval tv = {0, 5000}; // 5ms sleep
            FD_ZERO(&read_fds);
            FD_SET(rpc_client.fd, &read_fds);

            if (select(rpc_client.fd + 1, &read_fds, NULL, NULL, &tv) > 0) {
                yai_rpc_envelope_t env;
                ssize_t n = read(rpc_client.fd, &env, sizeof(env));
                
                if (n == sizeof(env)) {
                    char payload[YAI_RPC_BUFFER_MAX] = {0};
                    if (env.payload_len > 0 && env.payload_len < YAI_RPC_BUFFER_MAX) {
                        read(rpc_client.fd, payload, env.payload_len);
                    }

                    // Il Router gestisce il dispatching usando il ws_id dell'engine
                    // Passiamo il ws_id corrente per assicurare il multi-tenancy dello storage
                    char* response = yai_rpc_router_dispatch(ws_id, &env, payload);
                    if (response) {
                        write(rpc_client.fd, response, strlen(response));
                        free(response);
                    }
                } else if (n == 0) {
                    fprintf(stderr, "[RPC] Connection closed by Kernel.\n");
                    rpc_active = false;
                }
            }
        }

        usleep(1000); // 1ms tick rate
    }

    // 5. Cleanup
    fprintf(stderr, "\n[ENGINE] Cleaning up resources...\n");
    if (rpc_active) yai_rpc_close(&rpc_client);
    yai_storage_shutdown();
    yai_bridge_detach();

    return 0;
}