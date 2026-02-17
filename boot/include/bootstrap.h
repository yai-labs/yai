#ifndef YAI_BOOTSTRAP_H
#define YAI_BOOTSTRAP_H

#include "yai_vault.h"
#include <stdint.h>

int yai_init_system_shm(void);

/* Spawns root + kernel.
   Returns 0 on success */
int yai_spawn_planes(int *root_pid, int *kernel_pid, const char *argv0);

#endif
