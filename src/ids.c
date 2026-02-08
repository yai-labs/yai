#include "ice_vault.h"
#include <stdio.h>

void ice_generate_runtime_id(ice_vault_t *vault, char *buffer) {
    // Generiamo un ID deterministico basato sul vault
    sprintf(buffer, "ice-rt-%08x", (unsigned int)vault->logical_clock);
}