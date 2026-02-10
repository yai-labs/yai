#include "bootstrap.h"
#include "yai_vault.h"
#include <string.h>
#include <stdio.h>

void yai_vault_populate(yai_vault_t *vault, const char *ws_id, uint32_t quota) {
    // Pulizia
    memset(vault, 0, sizeof(yai_vault_t));
    
    // Configurazione Iniziale
    vault->status = YAI_STATE_PREBOOT;
    vault->authority_lock = false;
    vault->energy_quota = quota;
    vault->energy_consumed = 0;
    vault->trace_id[0] = '\0';
    vault->logical_clock = 0;
    strncpy(vault->workspace_id, ws_id, MAX_WS_ID - 1);
    
    printf("[STRAP] Vault populated for WS: %s with Quota: %u\n", ws_id, quota);
}

// Implementazione che il linker sta cercando
int yai_handoff_to_engine(yai_vault_t *vault) {
    if (vault->status == YAI_STATE_ERROR) {
        printf("[STRAP-FATAL] Vault in ERROR state. Handoff aborted.\n");
        return -1;
    }

    if (vault->authority_lock) {
        printf("[ERROR] Authority already transferred!\n");
        return -1;
    }

    printf("[STRAP] Setting authority_bit for Engine takeover...\n");
    vault->authority_lock = true;
    vault->status = YAI_STATE_HANDOFF_COMPLETE;

    // In un'architettura reale qui faremmo execve()
    printf("[STRAP] Handoff successful. Transitioning to Engine.\n");
    return 0;
}
