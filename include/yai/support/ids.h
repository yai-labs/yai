#pragma once

#include <stddef.h>

#include <yai/core/vault.h>
#include <yai/core/workspace.h>
#include <yai/protocol/message_types.h>

int yai_generate_runtime_id(const yai_vault_t *vault, char *buffer, size_t cap);
