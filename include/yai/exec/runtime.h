#pragma once

#include <stdbool.h>
#include <stdint.h>

typedef enum yai_exec_runtime_state {
  YAI_EXEC_OFFLINE = 0,
  YAI_EXEC_READY = 1,
  YAI_EXEC_BUSY = 2,
  YAI_EXEC_ERROR = 3
} yai_exec_runtime_state_t;

#define YAI_EXEC_RPC_BUFFER_MAX 4096

typedef struct yai_exec_config {
  char storage_backend[32];
  uint16_t max_parallel_agents;
  bool enforce_tla_safety;
} yai_exec_config_t;

const char *yai_exec_runtime_state_name(yai_exec_runtime_state_t state);
int yai_exec_runtime_probe(void);
int yai_exec_config_load_initial(const char *config_path, yai_exec_config_t *out_cfg);
bool yai_exec_config_enforce_limits(yai_exec_config_t *cfg);
