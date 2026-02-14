#include "storage_gate.h"
#include "engine_bridge.h"

#include <string.h>

static void yai_set_last_error(Vault* v, const char* msg) {
    if (!v || !msg) return;
    strncpy(v->last_error, msg, sizeof(v->last_error) - 1);
    v->last_error[sizeof(v->last_error) - 1] = '\0';
}

bool yai_storage_verify_access(const char* db_path, StorageAccessLevel level) {
    (void)db_path;

    Vault* v = yai_get_vault();
    if (!v) return false;

    if (v->authority_lock) {
        yai_set_last_error(v, "storage denied: authority_lock");
        return false;
    }

    if (level == STORAGE_WRITE) {
        if (v->energy_consumed > v->energy_quota) {
            yai_set_last_error(v, "storage denied: energy accounting");
            return false;
        }

        uint32_t left = v->energy_quota - v->energy_consumed;
        if (left < 10) {
            yai_set_last_error(v, "storage denied: low energy");
            return false;
        }

        if (!yai_consume_energy(5)) {
            yai_set_last_error(v, "storage denied: energy consume failed");
            return false;
        }
    }

    return true;
}

void yai_storage_audit_io(const char* op, size_t bytes) {
    (void)op;
    (void)bytes;
    // TODO: hook observability / event log (TLA+, trace, etc.)
}
