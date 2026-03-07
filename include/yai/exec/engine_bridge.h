#pragma once

#include <stdbool.h>
#include <stdint.h>

#include <yai/core/vault.h>

typedef yai_vault_t yai_exec_vault_t;

int yai_bridge_init(const char *ws_id);
yai_exec_vault_t *yai_bridge_attach(const char *ws_id, const char *channel);
void yai_bridge_detach(void);
yai_exec_vault_t *yai_get_vault(void);
bool yai_consume_energy(uint32_t amount);
void yai_audit_log_transition(const char *action, uint32_t prev_state, uint32_t new_state);
