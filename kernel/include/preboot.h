#ifndef PREBOOT_H
#define PREBOOT_H

#include "yai_vault.h"

// Verifica l'integrità dell'ambiente fisico e logico
int yai_run_preboot_checks();

// Mappa l'ambiente e inizializza i dati nel Vault
void yai_discover_environment(yai_vault_t *vault);

#endif
