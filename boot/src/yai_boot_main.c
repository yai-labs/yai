/* SPDX-License-Identifier: Apache-2.0 */
#define _POSIX_C_SOURCE 200809L
#include "preboot.h"
#include "bootstrap.h"
#include "yai_vault.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

#define SYSTEM_WS "system"

int main(void)
{
    printf("\n=== YAI MACHINE ENTRYPOINT (L0) ===\n");

    if (yai_run_preboot_checks() != 0) {
        fprintf(stderr, "[BOOT-FATAL] Preboot checks failed\n");
        return EXIT_FAILURE;
    }

    if (yai_ensure_runtime_layout(SYSTEM_WS) != 0) {
        fprintf(stderr, "[BOOT-FATAL] Runtime layout failed\n");
        return EXIT_FAILURE;
    }

    if (yai_init_system_shm() != 0) {
        fprintf(stderr, "[BOOT-FATAL] SHM init failed\n");
        return EXIT_FAILURE;
    }

    printf("[BOOT] Environment verified. Launching planes...\n");

    int root_pid = 0;
    int kernel_pid = 0;

    if (yai_spawn_planes(&root_pid, &kernel_pid) != 0) {
        fprintf(stderr, "[BOOT-FATAL] Failed to spawn planes\n");
        return EXIT_FAILURE;
    }

    printf("[BOOT] Root (%d) and Kernel (%d) launched\n",
           root_pid, kernel_pid);

    printf("[BOOT] Machine runtime is UP.\n");

    /* monitor loop */
    for (;;) {
        int status;
        pid_t p = waitpid(-1, &status, WNOHANG);

        if (p == root_pid)
            fprintf(stderr, "[BOOT] Root exited unexpectedly\n");

        if (p == kernel_pid)
            fprintf(stderr, "[BOOT] Kernel exited unexpectedly\n");

        sleep(1);
    }

    return EXIT_SUCCESS;
}
