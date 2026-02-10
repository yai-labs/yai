#include "yai_vault.h"
#include "yai_kernel.h"
#include "kernel.h"
#include <stdio.h>
#include <string.h>

// Grafo aggiornato secondo yai_vault.h
static const uint32_t TRANSITION_GRAPH[] = {
    [YAI_STATE_HALT]      = (1 << YAI_STATE_PREBOOT),
    [YAI_STATE_PREBOOT]   = (1 << YAI_STATE_READY),
    [YAI_STATE_READY]     = (1 << YAI_STATE_RUNNING) | (1 << YAI_STATE_HANDOFF_COMPLETE),
    [YAI_STATE_RUNNING]   = (1 << YAI_STATE_SUSPENDED) | (1 << YAI_STATE_ERROR) | (1 << YAI_STATE_HALT),
    [YAI_STATE_HANDOFF_COMPLETE] = (1 << YAI_STATE_RUNNING),
    [YAI_STATE_SUSPENDED] = (1 << YAI_STATE_RUNNING) | (1 << YAI_STATE_HALT),
    [YAI_STATE_ERROR]     = (1 << YAI_STATE_HALT)
};

static void yai_trace_transition(yai_event_type_t type, yai_state_t from_state, yai_state_t to_state, const char *reason) {
    char msg[160];
    snprintf(msg, sizeof(msg), "state %d -> %d (%s)", from_state, to_state, reason ? reason : "ok");
    yai_log_static(type, msg);
}

static int yai_guard_energy(const yai_vault_t *vault) {
    if (vault->energy_consumed > vault->energy_quota) {
        return -1;
    }
    return 0;
}

static int yai_guard_authority(const yai_vault_t *vault, yai_state_t to_state) {
    // Mapping: authority_lock == true implies no authority / cognitive invalidation
    if (to_state == YAI_STATE_RUNNING && vault->authority_lock) {
        return -1;
    }
    return 0;
}

static int yai_guard_external_effect(const yai_vault_t *vault, yai_state_t to_state) {
    if (to_state != YAI_STATE_RUNNING) return 0;
    uint32_t cmd_class = yai_command_class_for((yai_command_id_t)vault->last_command_id);
    if ((cmd_class & YAI_CMD_CLASS_EXTERNAL) && vault->authority_lock) {
        return -1;
    }
    return 0;
}

int yai_kernel_transition(yai_vault_t *vault, yai_state_t to_state) {
    // Check limiti array
    if (vault->status >= (sizeof(TRANSITION_GRAPH)/sizeof(uint32_t))) return -1;

    yai_state_t from_state = vault->status;

    if (yai_guard_energy(vault) != 0) {
        yai_trace_transition(EV_TRANSITION_REJECTED, from_state, to_state, "energy_guard_failed");
        return -1;
    }

    if (yai_guard_authority(vault, to_state) != 0) {
        yai_trace_transition(EV_TRANSITION_REJECTED, from_state, to_state, "authority_guard_failed");
        return -1;
    }

    if (yai_guard_external_effect(vault, to_state) != 0) {
        yai_trace_transition(EV_TRANSITION_REJECTED, from_state, to_state, "external_effect_guard_failed");
        return -1;
    }

    uint32_t allowed_mask = TRANSITION_GRAPH[vault->status];

    if (!(allowed_mask & (1 << to_state))) {
        // Ora il log sarà chiaro: "READY (2) -> RUNNING (4) non è permesso se non lo definiamo"
        printf("[FATAL] Invalid State Transition: %d -> %d\n", vault->status, to_state);
        yai_trace_transition(EV_TRANSITION_REJECTED, from_state, to_state, "transition_not_allowed");
        return -1;
    }

    vault->status = to_state;
    vault->logical_clock++;
    yai_trace_transition(EV_STATE_TRANSITION, from_state, to_state, "accepted");
    return 0;
}
