#pragma once

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

typedef struct yai_hardened_config {
  char storage_backend[32];
  uint16_t max_parallel_agents;
  bool enforce_tla_safety;
} yai_hardened_config_t;

/* Transitional rpc.v1 envelope validation errors. */
#define YAI_E_OK                0
#define YAI_E_BAD_ARG          -1
#define YAI_E_BAD_VERSION      -2
#define YAI_E_MISSING_WS       -3
#define YAI_E_WS_MISMATCH      -4
#define YAI_E_MISSING_TYPE     -5
#define YAI_E_TYPE_NOT_ALLOWED -6
#define YAI_E_PRIV_REQUIRED    -7
#define YAI_E_ROLE_REQUIRED    -8
#define YAI_E_HANDSHAKE_REQUIRED -9

bool yai_config_enforce_limits(yai_hardened_config_t *cfg);
int yai_validate_envelope_v1(const char *line, const char *expected_ws, char *out_request_type, size_t req_cap);
