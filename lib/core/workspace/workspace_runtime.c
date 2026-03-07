#include "internal.h"

const char *yai_workspace_runtime_state_name(int attached) {
  return attached ? "attached" : "detached";
}

