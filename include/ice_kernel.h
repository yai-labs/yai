#ifndef ICE_RUNTIME_H
#define ICE_RUNTIME_H

#include "ice_vault.h"
#include <time.h>

// Stati Canonici RFC-ICE-005 (Tradotti da Python)
typedef enum {
    ICE_STATE_CREATED = 0,
    ICE_STATE_PROVISIONED,
    ICE_STATE_CONTEXT_READY,
    ICE_STATE_EXECUTING,
    ICE_STATE_VALIDATING,
    ICE_STATE_COMMITTED,
    ICE_STATE_ABORTED,
    ICE_STATE_TERMINATED_BY_RUNTIME,
    ICE_STATE_TERMINATED
} ice_run_state_t;

// Capability Grant (Tradotto da enforcement.py)
typedef struct {
    uint32_t capability_id;
    uint32_t run_id;
    time_t expires_at;
    bool revoked;
    uint32_t scope_mask; // Usiamo i bit per gli scope, non le stringhe!
} ice_grant_t;

// RFC-ICE-003: Project Tree Scanner
void ice_scan_workspace(const char *path, int depth);

// FSM transition
int ice_runtime_transition(ice_vault_t *vault, ice_run_state_t to_state);

#endif
