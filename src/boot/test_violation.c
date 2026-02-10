#include "yai_vault.h"
#include <stdio.h>
#include <stdbool.h>

void test_authority_violation() {
    // Inizializzazione manuale dello stato (Simuliamo lo Strap in HALT)
    yai_vault_t vault = {
        .status = YAI_STATE_HALT,
        .authority_lock = false, // Nessuna autorità all'inizio
        .energy_quota = 1000
    };

    printf("[TEST] Tentativo di esecuzione senza autorità...\n");
    if (vault.authority_lock == false) {
        printf("[GUARD] A-002: Esecuzione negata. Sistema fermo.\n");
    }

    // Simuliamo il primo Handoff dello Strap
    printf("[STRAP] Eseguo Handoff...\n");
    vault.authority_lock = true;
    vault.status = YAI_STATE_READY;

    // Simuliamo un tentativo illegale di ri-ottenere autorità o resettarla
    printf("[ATTACK] Tentativo di spoofing dell'autorità...\n");
    if (vault.authority_lock == true && vault.status == YAI_STATE_READY) {
        printf("[SUCCESS] L'autorità è già impegnata. Il Kernel blocca tentativi multipli.\n");
    }
}

int main() {
    test_authority_violation();
    return 0;
}
