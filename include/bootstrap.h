#ifndef BOOTSTRAP_H
#define BOOTSTRAP_H

#include "ice_vault.h"

// Trasferimento irreversibile di autorità allo stato successivo (Engine)
int ice_handoff_to_engine(ice_vault_t *vault);

#endif
