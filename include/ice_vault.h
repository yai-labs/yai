#ifndef ICE_VAULT_H
#define ICE_VAULT_H

#include <stdint.h>
#include <stdbool.h>

// Costanti di Sistema per l'accesso SHM
#define SHM_VAULT_PREFIX "/ice_vault_"
#define MAX_WS_ID 64
#define MAX_TRACE_ID 64
#define MAX_ERR_MSG 256

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

// Vault condiviso con Engine (layout compatibile)
typedef struct {
    uint32_t status;                 // Stato corrente (usa ice_state_t)
    uint32_t energy_quota;           // I-005: Budget energetico
    uint32_t energy_consumed;        // Consumo energetico
    char workspace_id[MAX_WS_ID];    // ID del contesto operativo
    char trace_id[MAX_TRACE_ID];     // I-001: Tracciabilità
    bool authority_lock;             // Se TRUE, Engine è bloccato
    uint32_t last_command_id;        // Comando pendente
    uint32_t last_result;            // Risultato ultimo comando (0/1)
    char response_buffer[1024];      // Risposta Engine -> Kernel
    char last_error[MAX_ERR_MSG];    // Ultimo errore
    uint64_t logical_clock;          // I-002: Determinismo temporale (Kernel)
} ice_vault_t;

#endif
