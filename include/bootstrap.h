#ifndef BOOTSTRAP_H
#define BOOTSTRAP_H

#include "yai_vault.h"

// Trasferimento irreversibile di autorità allo stato successivo (Engine)
int yai_handoff_to_engine(yai_vault_t *vault);

#endif
