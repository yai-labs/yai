#include "yai_vault.h"
#include <stdio.h>

void yai_generate_runtime_id(yai_vault_t *vault, char *buffer) {
    // Generiamo un ID deterministico basato sul vault
    sprintf(buffer, "yai-rt-%08x", (unsigned int)vault->logical_clock);
}