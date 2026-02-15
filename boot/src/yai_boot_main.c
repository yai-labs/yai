#define _POSIX_C_SOURCE 200809L

#include "preboot.h"
#include "bootstrap.h"
#include "yai_vault.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <limits.h>
#include <sys/stat.h>
#include <errno.h>
#include <sys/wait.h>

#define KERNEL_BINARY "yai-kernel"
#define SYSTEM_WS "system"

// -----------------------------
// Verifica ABI
// -----------------------------
static int verify_vault_abi(void) {
    struct stat st;
    if (stat("law/specs/vault/yai_vault_abi.h", &st) != 0) {
        fprintf(stderr, "[BOOT-FATAL] Missing vault ABI header.\n");
        return -1;
    }
    return 0;
}

// -----------------------------
// Risoluzione path Kernel
// -----------------------------
static const char *resolve_kernel_path(void) {
    const char *env = getenv("YAI_KERNEL_BIN");
    if (env && env[0] != '\0')
        return env;
    return KERNEL_BINARY;
}

// -----------------------------
// Risoluzione path Root Server
// -----------------------------
static void resolve_root_path(char *out, size_t cap) {
    const char *home = getenv("HOME");
    if (!home) home = "/tmp";
    snprintf(out, cap, "%s/.yai/artifacts/yai-core/bin/yai-root-server", home);
}

// -----------------------------
// MAIN
// -----------------------------
int main(int argc, char **argv) {
    (void)argc;
    (void)argv;

    printf("\n\033[1;34m=== YAI MACHINE ENTRYPOINT (L0) ===\033[0m\n");

    yai_vault_t boot_ctx;
    memset(&boot_ctx, 0, sizeof(boot_ctx));
    strncpy(boot_ctx.workspace_id, SYSTEM_WS, MAX_WS_ID - 1);

    if (yai_run_preboot_checks() != 0) {
        fprintf(stderr, "[BOOT-FATAL] Preboot checks failed.\n");
        return EXIT_FAILURE;
    }

    if (verify_vault_abi() != 0) {
        return EXIT_FAILURE;
    }

    yai_discover_environment(&boot_ctx);

    if (yai_init_system_shm() != 0) {
        fprintf(stderr, "[BOOT-FATAL] Failed to initialize system SHM.\n");
        return EXIT_FAILURE;
    }

    printf("[BOOT] Environment verified. Launching Root Control Plane...\n");

    char root_server_bin[PATH_MAX];
    resolve_root_path(root_server_bin, sizeof(root_server_bin));

    char *root_args[] = {root_server_bin, "--master", NULL};

    pid_t root_pid = fork();
    if (root_pid == 0) {
        // Child: Root Control Plane persistente
        execv(root_server_bin, root_args);
        perror("[BOOT-FATAL] Root server exec failed");
        exit(EXIT_FAILURE);
    } else if (root_pid < 0) {
        perror("[BOOT-FATAL] fork() for Root server failed");
        return EXIT_FAILURE;
    }

    // Parent: continua con Kernel
    char kernel_bin_path[PATH_MAX];
    const char *home = getenv("HOME");
    snprintf(kernel_bin_path, sizeof(kernel_bin_path), "%s/.yai/artifacts/yai-core/bin/yai-kernel", home);

    char *kernel_args[] = { kernel_bin_path, "--master", NULL };

    pid_t kernel_pid = fork();
    if (kernel_pid == 0) {
        execv(kernel_bin_path, kernel_args);
        perror("[BOOT-FATAL] Kernel exec failed");
        exit(EXIT_FAILURE);
    }


    printf("[BOOT] Root (%d) and Kernel (%d) launched successfully\n", root_pid, kernel_pid);
    printf("[BOOT] Boot completed. Machine runtime is UP.\n");

    // -----------------------------
    // Loop di monitoraggio opzionale
    // -----------------------------
    while (1) {
        int status;
        pid_t p = waitpid(-1, &status, WNOHANG);
        if (p > 0) {
            if (p == root_pid) {
                fprintf(stderr, "[BOOT] Warning: Root server exited unexpectedly\n");
            } else if (p == kernel_pid) {
                fprintf(stderr, "[BOOT] Warning: Kernel exited unexpectedly\n");
            }
        }
        sleep(1);
    }

    return EXIT_SUCCESS;
}
