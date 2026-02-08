#ifndef ICE_VAULT_H
#define ICE_VAULT_H

#include <stdint.h>
#include <stdbool.h>

// Costanti di Sistema per l'accesso SHM
#define SHM_VAULT_PREFIX "/ice_vault_"
#define MAX_WS_ID 64

// Stati del Kernel (Derivati dal Kernel TLA+)
typedef enum {
    ICE_STATE_HALT = 0,
    ICE_STATE_PREBOOT,
    ICE_STATE_READY,
    ICE_STATE_HANDOFF_COMPLETE,
    ICE_STATE_RUNNING,
    ICE_STATE_SUSPENDED,
    ICE_STATE_ERROR
} ice_state_t;

// La struttura che garantisce A-003 (State as a Derived Artifact)
typedef struct {
    ice_state_t state;          // Stato attuale della FSM
    uint8_t authority_bit;      // A-002: 1 = Autorità concessa, 0 = Negata
    bool cognitive_valid;       // A-004: Validità della configurazione
    uint32_t energy_budget;     // I-005: Abstract Cost (Accountability)
    uint64_t trace_id;          // I-001: Tracciabilità univoca
    uint64_t logical_clock;     // I-002: Determinismo temporale
    char workspace_id[MAX_WS_ID]; // ID del contesto operativo
} ice_vault_t;

#endif
