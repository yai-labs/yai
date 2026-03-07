#pragma once

#include <yai/api/status.h>

typedef enum yai_runtime_mode {
  YAI_RUNTIME_MODE_UNSPECIFIED = 0,
  YAI_RUNTIME_MODE_CORE,
  YAI_RUNTIME_MODE_LEGACY_ROOT,
  YAI_RUNTIME_MODE_LEGACY_KERNEL,
  YAI_RUNTIME_MODE_LEGACY_ENGINE,
  YAI_RUNTIME_MODE_LEGACY_BRAIN
} yai_runtime_mode_t;

#define YAI_BIN_CLI "yai"
#define YAI_BIN_CORE "yai-core"
