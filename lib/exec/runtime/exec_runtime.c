#include "internal.h"

const char *yai_exec_runtime_state_name(yai_exec_runtime_state_t state) {
  switch (state) {
    case YAI_EXEC_OFFLINE:
      return "offline";
    case YAI_EXEC_READY:
      return "ready";
    case YAI_EXEC_BUSY:
      return "busy";
    case YAI_EXEC_ERROR:
      return "error";
    default:
      return "unknown";
  }
}

int yai_exec_runtime_probe(void) {
  return (int)YAI_EXEC_READY;
}

