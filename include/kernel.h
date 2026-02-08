#ifndef KERNEL_H
#define KERNEL_H

#include <stdint.h>
#include "ice_vault.h"   // <--- AGGIUNGI QUESTO: Contiene ice_vault_t e ice_state_t
#include "ice_kernel.h"

// Event Categories (RFC-ICE-007)
typedef enum {
    CAT_RUNTIME = 0, CAT_COGNITIVE, CAT_DOMAIN, CAT_MEMORY, CAT_CAPABILITY
} ice_event_category_t;

// Event Types - La Tassonomia Chiusa
typedef enum {
    // Runtime
    EV_RUN_PROVISIONED = 100, EV_CONTEXT_RESOLVED, EV_VALIDATION_PASSED, EV_RUN_TERMINATED,
    // Cognitive
    EV_INFERENCE_STEP = 200, EV_DECISION_PROPOSED,
    // Memory
    EV_MEMORY_PROMOTED = 300, EV_MEMORY_EXPIRED, EV_MEMORY_INVALIDATED,
    // Capability
    EV_CAP_REQUESTED = 400, EV_CAP_GRANTED, EV_CAP_REVOKED
} ice_event_type_t;

// Memory Status (Da lifecycle.py)
typedef enum {
    MEM_ACTIVE = 0,
    MEM_EXPIRED,
    MEM_DEPRECATED,
    MEM_SUPERSEDED,
    MEM_INVALIDATED
} ice_mem_status_t;

typedef struct {
    uint32_t memory_id;
    ice_mem_status_t status;
    uint32_t replaced_by_id; // Per SUPERSEDED
} ice_memory_state_t;

int ice_memory_transition(ice_memory_state_t *mem, ice_mem_status_t new_status);

// Profile Configuration (Interfacce collegate al Kernel)
typedef enum {
    PROFILE_MINIMAL = 0, // Solo Kernel + CLI
    PROFILE_STUDIO,      // IDE Native + Agents
    PROFILE_EDGE,        // Embedded / Remote
    PROFILE_FULL         // Engine + Consciousness + Observability
} ice_profile_t;

typedef struct {
    ice_profile_t type;
    uint32_t max_sessions;    // es. 1 per Edge, 64 per Studio
    uint8_t observability_lv; // 0: None, 1: Audit, 2: Full Trace
    uint8_t allow_io_stream;  // Abilita/Disabilita streaming del disco
} ice_profile_config_t;

void ice_log_static(ice_event_type_t type, const char *msg);

// La struttura dell'evento atomico (A-003)
typedef struct {
    uint64_t timestamp;
    ice_event_type_t type;
    uint32_t run_id;
    char payload_summary[64]; // Sgrassato: solo l'essenziale per il kernel
} ice_event_t;

// --- State Machine Logic (Ex state_machine.py) ---

// Transizione dello stato globale del Kernel nel Vault
// Ritorna 0 se la transizione è valida secondo le regole ICE
int ice_kernel_transition(ice_vault_t *vault, ice_state_t new_state);

// Scansione del workspace per integrità (Ex project_tree.py)
void ice_scan_workspace(const char *path, int depth);



#endif

