#include "ice_vault.h"
#include "ice_kernel.h"
#include <stdio.h>

// Grafo aggiornato secondo ice_vault.h
static const uint32_t TRANSITION_GRAPH[] = {
    [ICE_STATE_HALT]      = (1 << ICE_STATE_PREBOOT),
    [ICE_STATE_PREBOOT]   = (1 << ICE_STATE_READY),
    [ICE_STATE_READY]     = (1 << ICE_STATE_RUNNING) | (1 << ICE_STATE_HANDOFF_COMPLETE),
    [ICE_STATE_RUNNING]   = (1 << ICE_STATE_SUSPENDED) | (1 << ICE_STATE_ERROR) | (1 << ICE_STATE_HALT),
    [ICE_STATE_SUSPENDED] = (1 << ICE_STATE_RUNNING) | (1 << ICE_STATE_HALT),
    [ICE_STATE_ERROR]     = (1 << ICE_STATE_HALT)
};

int ice_kernel_transition(ice_vault_t *vault, ice_state_t to_state) {
    // Check limiti array
    if (vault->status >= (sizeof(TRANSITION_GRAPH)/sizeof(uint32_t))) return -1;

    uint32_t allowed_mask = TRANSITION_GRAPH[vault->status];

    if (!(allowed_mask & (1 << to_state))) {
        // Ora il log sarà chiaro: "READY (2) -> RUNNING (4) non è permesso se non lo definiamo"
        printf("[FATAL] Invalid State Transition: %d -> %d\n", vault->status, to_state);
        return -1;
    }

    vault->status = to_state;
    vault->logical_clock++;
    return 0;
}
