// kernel/src/core/fsm.c
#include "yai_vault.h"
#include "kernel.h"
#include <stdio.h>
#include <string.h>

// Grafo delle transizioni (Kernel global state machine)
static const uint32_t TRANSITION_GRAPH[] = {
    [YAI_STATE_HALT]            = (1u << YAI_STATE_PREBOOT),
    [YAI_STATE_PREBOOT]         = (1u << YAI_STATE_READY),
    [YAI_STATE_READY]           = (1u << YAI_STATE_RUNNING) | (1u << YAI_STATE_HANDOFF_COMPLETE),
    [YAI_STATE_HANDOFF_COMPLETE]= (1u << YAI_STATE_RUNNING),
    [YAI_STATE_RUNNING]         = (1u << YAI_STATE_SUSPENDED) | (1u << YAI_STATE_ERROR) | (1u << YAI_STATE_HALT),
    [YAI_STATE_SUSPENDED]       = (1u << YAI_STATE_RUNNING) | (1u << YAI_STATE_HALT),
    [YAI_STATE_ERROR]           = (1u << YAI_STATE_HALT),
};

static const char *safe_cstr(const char *s) { return (s && s[0]) ? s : ""; }

static void trace_transition(
    yai_event_type_t ev,
    const yai_vault_t *vault,
    yai_state_t from_state,
    yai_state_t to_state,
    const char *reason
) {
    char msg[192];
    snprintf(msg, sizeof(msg), "kernel_state %u -> %u reason=%s",
             (unsigned)from_state, (unsigned)to_state, safe_cstr(reason));

    const char *ws = vault ? safe_cstr(vault->workspace_id) : "";
    const char *tr = vault ? safe_cstr(vault->trace_id) : "";

    char data[160];
    snprintf(data, sizeof(data), "{\"from\":%u,\"to\":%u,\"reason\":\"%s\"}",
             (unsigned)from_state, (unsigned)to_state, safe_cstr(reason));

    yai_log_static(ev, ws, tr, "info", msg, data);
}

// I-005: Controllo budget energetico
static int guard_energy(const yai_vault_t *vault) {
    if (!vault) return -1;
    if (vault->energy_consumed > vault->energy_quota) return -1;
    return 0;
}

// I-006: Controllo autorità su transizioni critiche
static int guard_authority(const yai_vault_t *vault, yai_state_t to_state) {
    if (!vault) return -1;
    // Se il lock è attivo, non si può entrare in RUNNING
    if (to_state == YAI_STATE_RUNNING && vault->authority_lock) return -1;
    return 0;
}

// Verifica effetti esterni (Network/AI)
static int guard_external_effect(const yai_vault_t *vault, yai_state_t to_state) {
    if (!vault || to_state != YAI_STATE_RUNNING) return 0;

    // Usiamo il resolver centralizzato in yai_vault.h
    uint32_t cmd_class = yai_resolve_command_class(vault->last_command_id);

    // Se il comando ha effetti esterni e l'autorità è bloccata -> REJECT
    if ((cmd_class == 0x02) && vault->authority_lock) return -1;
    return 0;
}

int yai_kernel_transition(yai_vault_t *vault, yai_state_t to_state) {
    if (!vault) return -1;

    const size_t n = sizeof(TRANSITION_GRAPH) / sizeof(TRANSITION_GRAPH[0]);
    if ((size_t)vault->status >= n || (size_t)to_state >= n) return -1;

    const yai_state_t from_state = (yai_state_t)vault->status;

    // --- ENFORCEMENT LAYER ---
    if (guard_energy(vault) != 0) {
        trace_transition(EV_TRANSITION_REJECTED, vault, from_state, to_state, "energy_limit");
        return -1;
    }
    if (guard_authority(vault, to_state) != 0) {
        trace_transition(EV_TRANSITION_REJECTED, vault, from_state, to_state, "authority_locked");
        return -1;
    }
    if (guard_external_effect(vault, to_state) != 0) {
        trace_transition(EV_TRANSITION_REJECTED, vault, from_state, to_state, "external_effect_denied");
        return -1;
    }

    // --- VALIDAZIONE GRAFO ---
    const uint32_t allowed_mask = TRANSITION_GRAPH[(size_t)from_state];
    if ((allowed_mask & (1u << to_state)) == 0) {
        trace_transition(EV_TRANSITION_REJECTED, vault, from_state, to_state, "invalid_transition");
        return -1;
    }

    // --- COMMIT STATE ---
    vault->status = (uint32_t)to_state;
    vault->logical_clock++;
    trace_transition(EV_STATE_TRANSITION, vault, from_state, to_state, "accepted");
    return 0;
}