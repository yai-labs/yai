#ifndef ICE_VAULT_H
#define ICE_VAULT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

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

// Comandi Engine (orchestrator -> engine)
typedef enum {
    ICE_CMD_NONE = 0,
    ICE_CMD_PING = 1,
    ICE_CMD_NOOP = 2
} ice_command_t;

// Vault condiviso con Engine (layout compatibile)
typedef struct {
    uint32_t status;                 // Stato corrente (usa ice_state_t)
    uint32_t energy_quota;           // I-005: Budget energetico
    uint32_t energy_consumed;        // Consumo energetico
    char workspace_id[MAX_WS_ID];    // ID del contesto operativo
    char trace_id[MAX_TRACE_ID];     // I-001: Tracciabilità
    bool authority_lock;             // Se TRUE, l'Engine è bloccato (governance lock)
    uint8_t _pad0[3];                // padding per allineamento a 4 byte
    uint32_t last_command_id;        // Comando pendente
    uint32_t command_seq;            // Sequenza comando (incrementale)
    uint32_t last_processed_seq;     // Ultima sequenza processata
    uint32_t last_result;            // Risultato ultimo comando (0/1)
    char response_buffer[1024];      // Risposta Engine -> Kernel
    char last_error[MAX_ERR_MSG];    // Ultimo errore
    uint64_t logical_clock;          // I-002: Determinismo temporale (Kernel)
} ice_vault_t;

// ABI checks (offset + size)
_Static_assert(offsetof(ice_vault_t, status) == 0, "ice_vault_t.status offset mismatch");
_Static_assert(offsetof(ice_vault_t, last_command_id) == 144, "ice_vault_t.last_command_id offset mismatch");
_Static_assert(offsetof(ice_vault_t, command_seq) == 148, "ice_vault_t.command_seq offset mismatch");
_Static_assert(offsetof(ice_vault_t, last_processed_seq) == 152, "ice_vault_t.last_processed_seq offset mismatch");
_Static_assert(offsetof(ice_vault_t, response_buffer) == 160, "ice_vault_t.response_buffer offset mismatch");
_Static_assert(offsetof(ice_vault_t, logical_clock) == 1440, "ice_vault_t.logical_clock offset mismatch");
_Static_assert(sizeof(ice_vault_t) == 1448, "ice_vault_t size mismatch");

#endif
