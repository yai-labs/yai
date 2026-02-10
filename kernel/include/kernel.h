#ifndef KERNEL_H
#define KERNEL_H

#include <stdint.h>
#include "yai_vault.h"   // <--- AGGIUNGI QUESTO: Contiene yai_vault_t e yai_state_t
#include "yai_kernel.h"

// Event Categories (RFC-YAI-007)
typedef enum {
    CAT_RUNTIME = 0, CAT_COGNITIVE, CAT_DOMAIN, CAT_MEMORY, CAT_CAPABILITY
} yai_event_category_t;

// Event Types - La Tassonomia Chiusa
typedef enum {
    // Runtime
    EV_RUN_PROVISIONED = 100, EV_CONTEXT_RESOLVED, EV_VALIDATION_PASSED, EV_RUN_TERMINATED,
    EV_STATE_TRANSITION = 110, EV_TRANSITION_REJECTED = 111,
    // Cognitive
    EV_INFERENCE_STEP = 200, EV_DECISION_PROPOSED,
    // Memory
    EV_MEMORY_PROMOTED = 300, EV_MEMORY_EXPIRED, EV_MEMORY_INVALIDATED,
    // Capability
    EV_CAP_REQUESTED = 400, EV_CAP_GRANTED, EV_CAP_REVOKED
} yai_event_type_t;

// Memory Status (Da lifecycle.py)
typedef enum {
    MEM_ACTIVE = 0,
    MEM_EXPIRED,
    MEM_DEPRECATED,
    MEM_SUPERSEDED,
    MEM_INVALIDATED
} yai_mem_status_t;

typedef struct {
    uint32_t memory_id;
    yai_mem_status_t status;
    uint32_t replaced_by_id; // Per SUPERSEDED
} yai_memory_state_t;

int yai_memory_transition(yai_memory_state_t *mem, yai_mem_status_t new_status);

// Profile Configuration (Interfacce collegate al Kernel)
typedef enum {
    PROFILE_MINIMAL = 0, // Solo Kernel + CLI
    PROFILE_STUDIO,      // IDE Native + Agents
    PROFILE_EDGE,        // Embedded / Remote
    PROFILE_FULL         // Engine + Consciousness + Observability
} yai_profile_t;

typedef struct {
    yai_profile_t type;
    uint32_t max_sessions;    // es. 1 per Edge, 64 per Studio
    uint8_t observability_lv; // 0: None, 1: Audit, 2: Full Trace
    uint8_t allow_io_stream;  // Abilita/Disabilita streaming del disco
} yai_profile_config_t;

void yai_log_static(yai_event_type_t type, const char *msg);

// La struttura dell'evento atomico (A-003)
typedef struct {
    uint64_t timestamp;
    yai_event_type_t type;
    uint32_t run_id;
    char payload_summary[64]; // Sgrassato: solo l'essenziale per il kernel
} yai_event_t;

// --- State Machine Logic (Ex state_machine.py) ---

// Transizione dello stato globale del Kernel nel Vault
// Ritorna 0 se la transizione è valida secondo le regole YAI
int yai_kernel_transition(yai_vault_t *vault, yai_state_t new_state);

// Scansione del workspace per integrità (Ex project_tree.py)
void yai_scan_workspace(const char *path, int depth);



#endif
