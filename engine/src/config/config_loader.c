#include "../include/config_enforcer.h"
#include <stdio.h>
#include <string.h>

// Mock loader per ora (verrà sostituito con un parser JSON/YAML C)
int yai_config_load_initial(const char* config_path, HardenedConfig* out_cfg) {
    printf("[YAI-CONFIG] Loading core config from %s...\n", config_path);
    
    // Default safe values
    strncpy(out_cfg->storage_backend, "duckdb", 31);
    out_cfg->max_parallel_agents = 4;
    out_cfg->enforce_tla_safety = true;

    return yai_config_enforce_limits(out_cfg) ? 0 : -1;
}

bool yai_config_enforce_limits(HardenedConfig* cfg) {
    if (cfg->max_parallel_agents > 32) return false; // Limite fisico
    return true;
}
