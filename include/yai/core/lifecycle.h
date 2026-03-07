#pragma once

/* Transitional public lifecycle boundary sourced from boot preboot/bootstrap headers. */
int yai_run_preboot_checks(void);
int yai_ensure_runtime_layout(const char *ws_id);
int yai_init_system_shm(void);
int yai_spawn_planes(int *root_pid, int *kernel_pid, const char *argv0);
