#include "ice_vault.h"
#include "ice_kernel.h" 
#include "kernel.h"
#include "transport.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h> // <--- FIX 1: Per strcmp

int main(int argc, char **argv) {
    const char *ws_id = "default";
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--ws") == 0 && i + 1 < argc) {
            ws_id = argv[i + 1];
        }
    }

    ice_vault_t vault = {
        .state = ICE_STATE_READY,
        .energy_budget = 5000,
        .authority_bit = 1,
        .logical_clock = 0
    };

    printf("--- ICE KERNEL C-CORE (LAYER 1) ---\n");
    printf("[KERNEL] Operating in Workspace: %s\n", ws_id);

    // FIX 2: Assicurati che in fsm.c/kernel.h questa funzione esista.
    // Se si chiama ice_runtime_transition, rinominala in kernel.h/fsm.c
    if (ice_kernel_transition(&vault, ICE_STATE_RUNNING) == 0) {

        if (ice_transport_init() != 0) {
            printf("[TRANSPORT] Failed to init socket.\n");
        }

        ice_scan_workspace(".", 0);

        printf("[KERNEL] Awaiting commands from Engine...\n");
        ice_transport_serve_once();

        if (vault.energy_budget < 1000) {
            printf("[KERNEL] Energy critical! Triggering memory mitigation.\n");
            // FIX 3: Cambiato il primo parametro per evitare il warning enum mismatch
            // Assicurati che EV_ERROR o simile sia definito in logger.h/kernel.h
            ice_log_static(0, "Energy budget below threshold."); 
        }
    }

    return 0;
}
