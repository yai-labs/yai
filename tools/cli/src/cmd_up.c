#include "../include/yai_paths.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/stat.h>

#define YAI_OK 0
#define WAIT_RETRIES 20
#define WAIT_INTERVAL_US 200000  // 200ms

/* ------------------------------------------------------------
   Check root socket existence
------------------------------------------------------------ */

static int root_socket_exists(void) {
    char sock[512];
    if (yai_path_root_sock(sock, sizeof(sock)) != 0)
        return -1;

    struct stat st;
    return (stat(sock, &st) == 0) ? YAI_OK : -1;
}

/* ------------------------------------------------------------
   Wait for root socket ready
------------------------------------------------------------ */

static int wait_for_root_ready(void) {
    for (int i = 0; i < WAIT_RETRIES; i++) {
        if (root_socket_exists() == YAI_OK)
            return YAI_OK;

        usleep(WAIT_INTERVAL_US);
    }
    return -1;
}

/* ------------------------------------------------------------
   Resolve binary path inside ~/.yai/artifacts
------------------------------------------------------------ */

static int resolve_bin(char *out, size_t cap, const char *name) {
    const char *home = getenv("HOME");
    if (!home) return -1;

    int n = snprintf(
        out,
        cap,
        "%s/.yai/artifacts/yai-core/bin/%s",
        home,
        name
    );

    if (n <= 0 || (size_t)n >= cap)
        return -1;

    return 0;
}

/* ------------------------------------------------------------
   cmd_up
------------------------------------------------------------ */

void cmd_up(void) {
    printf("\033[1;33m[UP]\033[0m Orchestrating Sovereign Runtime...\n");

    char kernel_bin[512];
    char engine_bin[512];

    if (resolve_bin(kernel_bin, sizeof(kernel_bin), "yai-kernel") != 0 ||
        resolve_bin(engine_bin, sizeof(engine_bin), "yai-engine") != 0) {
        fprintf(stderr, "[ERROR] Could not resolve runtime binaries.\n");
        return;
    }

    /* --------------------------------------------------------
       L1: Kernel
    -------------------------------------------------------- */

    if (root_socket_exists() != YAI_OK) {
        printf("[UP] Root Plane offline. Launching kernel...\n");

        char cmd[600];
        snprintf(cmd, sizeof(cmd), "%s --master &", kernel_bin);

        if (system(cmd) != 0) {
            fprintf(stderr, "[ERROR] Failed to launch kernel.\n");
            return;
        }

        if (wait_for_root_ready() != YAI_OK) {
            fprintf(stderr, "[ERROR] Kernel did not become ready.\n");
            return;
        }
    } else {
        printf("[UP] Root Plane already active.\n");
    }

    /* --------------------------------------------------------
       L2: Engine
    -------------------------------------------------------- */

    printf("[UP] Launching Engine...\n");

    char cmd2[600];
    snprintf(cmd2, sizeof(cmd2), "%s &", engine_bin);

    if (system(cmd2) != 0) {
        fprintf(stderr, "[ERROR] Failed to launch Engine.\n");
        return;
    }

    printf("\n\033[1;32m✔ YAI Runtime is UP (Machine Context)\033[0m\n");
    printf("Next: choose a workspace with --ws and use 'yai mind ...'\n");
}
