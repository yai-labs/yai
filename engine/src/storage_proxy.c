#include "../include/storage_gate.h"
#include "../include/engine_bridge.h"
#include <stdio.h>

bool yai_storage_verify_access(const char* db_path, StorageAccessLevel level) {
    (void)db_path; // Silenzia warning
    Vault* v = yai_get_vault();
    
    if (!v || v->authority_lock) return false;

    // Logica basata su quota e consumo
    if (level == STORAGE_WRITE) {
        uint32_t current_left = v->energy_quota - v->energy_consumed;
        if (current_left < 10) return false; 
        
        // Usiamo la funzione del bridge per consumare in modo atomico
        yai_consume_energy(5); 
    }

    return true; 
}

void yai_storage_audit_io(const char* op, size_t bytes) {
    // Placeholder per logging TLA+
    (void)op;
    (void)bytes;
}