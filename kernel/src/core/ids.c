// kernel/src/core/ids.c  (fix: serve cap per non rischiare overflow)
#include "yai_vault.h"
#include <stdio.h>

int yai_generate_runtime_id(const yai_vault_t *vault, char *buffer, size_t cap) {
    if (!vault || !buffer || cap < 16) return -1;
    // Deterministico e stabile: dipende dal logical_clock
    // (Phase-1: ok; più avanti aggiungiamo machine_id + ws_id)
    int n = snprintf(buffer, cap, "yai-rt-%08x", (unsigned int)vault->logical_clock);
    return (n < 0 || (size_t)n >= cap) ? -1 : 0;
}
