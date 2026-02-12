#ifndef YAI_VAULT_H
#define YAI_VAULT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Vault ABI (authoritative offsets from Law)
#include "yai_vault_abi.h"

// Protocol authority (source of truth)
#include "../../law/specs/protocol/commands.h"

// Costanti di Sistema per l'accesso SHM
#define SHM_VAULT_PREFIX "/yai_vault_"
#define MAX_WS_ID 64
#define MAX_TRACE_ID 64
#define MAX_ERR_MSG 256

// Stati del Kernel (Derivati dal Kernel TLA+)
typedef enum {
    YAI_STATE_HALT = 0,
    YAI_STATE_PREBOOT,
    YAI_STATE_READY,
    YAI_STATE_HANDOFF_COMPLETE,
    YAI_STATE_RUNNING,
    YAI_STATE_SUSPENDED,
    YAI_STATE_ERROR
} yai_state_t;

// Comandi Engine (control -> engine) — from protocol authority
typedef yai_command_id_t yai_command_t;

// Canonical vault header (L0 ABI)
#define YAI_VAULT_HEADER_CORE_SIZE 28
struct yai_vault_header {
    uint32_t magic;
    uint16_t version;
    uint16_t flags;
    uint32_t state_id;
    uint32_t command_id;
    uint32_t trace_id;
    uint32_t energy_quota;
    uint32_t energy_used;
    uint8_t _reserved[YAI_VAULT_HEADER_SIZE - YAI_VAULT_HEADER_CORE_SIZE];
};
typedef struct yai_vault_header yai_vault_header_t;

// Vault condiviso con Engine (layout compatibile)
typedef struct {
    uint32_t status;                 // Stato corrente (usa yai_state_t)
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
} yai_vault_t;

// Vault ABI (header) checks
_Static_assert(offsetof(struct yai_vault_header, magic) == YAI_VAULT_OFF_MAGIC, "vault ABI drift: magic");
_Static_assert(offsetof(struct yai_vault_header, state_id) == YAI_VAULT_OFF_STATE_ID, "vault ABI drift: state_id");
_Static_assert(offsetof(struct yai_vault_header, command_id) == YAI_VAULT_OFF_COMMAND_ID, "vault ABI drift: command_id");
_Static_assert(offsetof(struct yai_vault_header, trace_id) == YAI_VAULT_OFF_TRACE_ID, "vault ABI drift: trace_id");
_Static_assert(offsetof(struct yai_vault_header, energy_quota) == YAI_VAULT_OFF_ENERGY_QUOTA, "vault ABI drift: energy_quota");
_Static_assert(offsetof(struct yai_vault_header, energy_used) == YAI_VAULT_OFF_ENERGY_USED, "vault ABI drift: energy_used");
_Static_assert(sizeof(struct yai_vault_header) == YAI_VAULT_HEADER_SIZE, "vault ABI drift: header size");

// ABI checks (offset + size)
_Static_assert(offsetof(yai_vault_t, status) == 0, "yai_vault_t.status offset mismatch");
_Static_assert(offsetof(yai_vault_t, last_command_id) == 144, "yai_vault_t.last_command_id offset mismatch");
_Static_assert(offsetof(yai_vault_t, command_seq) == 148, "yai_vault_t.command_seq offset mismatch");
_Static_assert(offsetof(yai_vault_t, last_processed_seq) == 152, "yai_vault_t.last_processed_seq offset mismatch");
_Static_assert(offsetof(yai_vault_t, response_buffer) == 160, "yai_vault_t.response_buffer offset mismatch");
_Static_assert(offsetof(yai_vault_t, logical_clock) == 1440, "yai_vault_t.logical_clock offset mismatch");
_Static_assert(sizeof(yai_vault_t) == 1448, "yai_vault_t size mismatch");

#endif
