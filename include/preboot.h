#ifndef PREBOOT_H
#define PREBOOT_H

#include "ice_vault.h"

// Verifica l'integrità dell'ambiente fisico e logico
int ice_run_preboot_checks();

// Mappa l'ambiente e inizializza i dati nel Vault
void ice_discover_environment(ice_vault_t *vault);

#endif
