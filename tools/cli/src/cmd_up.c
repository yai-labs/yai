#include "../include/yai_paths.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define YAI_OK 0

static int ping_root_socket(void) {
    char sock[512];
    if (yai_path_root_sock(sock, sizeof(sock)) != 0) return -1;
    return (access(sock, F_OK) == 0) ? YAI_OK : -1;
}

void cmd_up(void) {
    printf("\033[1;33m[UP]\033[0m Orchestrating Sovereign Runtime...\n");

    // 1) L0->L1: boot/kernel
    if (ping_root_socket() != YAI_OK) {
        printf("[UP] Root Plane offline. Launching yai-boot...\n");
        if (system("yai-boot &") != 0) {
            fprintf(stderr, "[ERROR] Failed to ignite bootstrap.\n");
            return;
        }
        sleep(2);
    }

    // 2) L1->L2: engine
    printf("[UP] Root Plane active. Igniting Engine...\n");
    if (system("yai-engine &") != 0) {
        fprintf(stderr, "[ERROR] Failed to launch Engine.\n");
        return;
    }

    printf("\n\033[1;32m✔ YAI Runtime is UP (Global Context)\033[0m\n");
    printf("Next: choose a workspace with --ws and use 'yai mind ...'\n");
}
