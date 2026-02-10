#include "yai_kernel.h"
#include "kernel.h"
#include <time.h> // Serve per time(NULL)

int yai_enforce_capability(const yai_grant_t *grant, uint32_t current_run_id, uint32_t req_scope) {
    // 1. Run ownership
    if (grant->run_id != current_run_id) return -1;

    // 2. Revoca
    if (grant->revoked) return -2;

    // 3. Scadenza (Sostituisce datetime.utcnow())
    if (grant->expires_at > 0 && time(NULL) >= grant->expires_at) return -3;

    // 4. Scope enforcement (Bitwise AND, velocità pura)
    if (!(grant->scope_mask & req_scope)) return -4;

    return 0; // SUCCESS
}

int yai_memory_transition(yai_memory_state_t *mem, yai_mem_status_t new_status) {
    // Invariante: Puoi cambiare stato solo se sei ACTIVE
    if (mem->status != MEM_ACTIVE) {
        return -1; // MemoryLifecycleError
    }

    mem->status = new_status;
    return 0;
}
