#ifndef BOOTSTRAP_H
#define BOOTSTRAP_H

#include "yai_vault.h"

/* Vault bootstrap */
void yai_vault_populate(
    yai_vault_t *vault,
    const char *ws_id,
    uint32_t quota
);

/* System SHM initialization (Machine Plane) */
int yai_init_system_shm(void);

/* Authority transfer */
int yai_handoff_to_engine(yai_vault_t *vault);

#endif
