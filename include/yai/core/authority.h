#pragma once

#include <stdbool.h>
#include <stdint.h>

#include <yai/core/vault.h>

bool yai_vault_allows_command(const yai_vault_t *v, uint32_t cmd);
