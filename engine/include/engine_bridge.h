#ifndef YAI_ENGINE_BRIDGE_H
#define YAI_ENGINE_BRIDGE_H

#include "yai_vault.h"

// Vault header of record è in Kernel/include/yai_vault.h.
// Alias locale per compatibilità Engine.
typedef yai_vault_t Vault;

// Prototipi
int yai_bridge_init(const char* ws_id);
Vault* yai_bridge_attach(const char* ws_id, const char* channel);
void yai_bridge_detach(void);
Vault* yai_get_vault(void);
bool yai_consume_energy(uint32_t amount);

// Audit log for TLA+ trace comparison
void yai_audit_log_transition(const char* action, uint32_t prev_state, uint32_t new_state);

#endif
