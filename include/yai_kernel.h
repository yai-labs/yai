#ifndef YAI_RUNTIME_H
#define YAI_RUNTIME_H

#include "yai_vault.h"
#include <time.h>

// Stati Canonici RFC-YAI-005 (Tradotti da Python)
typedef enum {
    YAI_STATE_CREATED = 0,
    YAI_STATE_PROVISIONED,
    YAI_STATE_CONTEXT_READY,
    YAI_STATE_EXECUTING,
    YAI_STATE_VALIDATING,
    YAI_STATE_COMMITTED,
    YAI_STATE_ABORTED,
    YAI_STATE_TERMINATED_BY_RUNTIME,
    YAI_STATE_TERMINATED
} yai_run_state_t;

// Capability Grant (Tradotto da enforcement.py)
typedef struct {
    uint32_t capability_id;
    uint32_t run_id;
    time_t expires_at;
    bool revoked;
    uint32_t scope_mask; // Usiamo i bit per gli scope, non le stringhe!
} yai_grant_t;

// RFC-YAI-003: Project Tree Scanner
void yai_scan_workspace(const char *path, int depth);

// FSM transition
int yai_runtime_transition(yai_vault_t *vault, yai_run_state_t to_state);

#endif
