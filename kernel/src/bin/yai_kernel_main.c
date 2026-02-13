#include "yai_vault.h"
#include "yai_kernel.h" 
#include "kernel.h"
#include "transport.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h> // <--- FIX 1: Per strcmp
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>

int main(int argc, char **argv) {
    const char *ws_id = "default";
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--ws") == 0 && i + 1 < argc) {
            ws_id = argv[i + 1];
        }
    }

    char shm_path[128];
    snprintf(shm_path, sizeof(shm_path), "%s%s", SHM_VAULT_PREFIX, ws_id);

    int fd = shm_open(shm_path, O_RDWR, 0666);
    if (fd == -1) {
        fprintf(stderr, "[KERNEL] shm_open failed for %s: %s\n", shm_path, strerror(errno));
        return 1;
    }

    yai_vault_t *vault = mmap(NULL, sizeof(yai_vault_t), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    close(fd);
    if (vault == MAP_FAILED) {
        fprintf(stderr, "[KERNEL] mmap failed: %s\n", strerror(errno));
        return 1;
    }

    if (vault->workspace_id[0] == '\0') {
        strncpy(vault->workspace_id, ws_id, MAX_WS_ID - 1);
    }
    if (vault->energy_quota == 0) {
        vault->energy_quota = 1000;
    }
    vault->status = YAI_STATE_READY;
    vault->authority_lock = false;
    if (vault->logical_clock == 0) {
        vault->logical_clock = 0;
    }

    printf("--- YAI KERNEL C-CORE (LAYER 1) ---\n");
    printf("[KERNEL] Operating in Workspace: %s\n", ws_id);

    // FIX 2: Assicurati che in fsm.c/kernel.h questa funzione esista.
    // Se si chiama yai_runtime_transition, rinominala in kernel.h/fsm.c
    if (yai_kernel_transition(vault, YAI_STATE_RUNNING) == 0) {

        if (yai_transport_init() != 0) {
            printf("[TRANSPORT] Failed to init socket.\n");
        }

        yai_scan_workspace(".", 0);

        printf("[KERNEL] Awaiting commands from Engine...\n");
        while (1) {
            yai_transport_serve_once();
        }

        if (vault->energy_quota < 1000) {
            printf("[KERNEL] Energy critical! Triggering memory mitigation.\n");
            // FIX 3: Cambiato il primo parametro per evitare il warning enum mismatch
            // Assicurati che EV_ERROR o simile sia definito in logger.h/kernel.h
            yai_log_static(0, "Energy budget below threshold."); 
        }
    }

    return 0;
}
