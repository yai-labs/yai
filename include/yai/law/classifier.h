#pragma once

#include <stddef.h>

typedef struct yai_law_classification_ctx {
  char ws_id[64];
  char command[128];
  char action[64];
  char provider[64];
  char resource[64];
  char protocol[32];
  char workspace_mode[32];
  char declared_family[64];
  char declared_specialization[96];
  int has_params_hash;
  int black_box_mode;
  int has_authority_contract;
} yai_law_classification_ctx_t;

int yai_law_classify_event(const char *ws_id,
                           const char *payload,
                           yai_law_classification_ctx_t *out);
