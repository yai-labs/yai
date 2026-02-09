#define _POSIX_C_SOURCE 200809L
#include "ice_vault.h"
#include "preboot.h"
#include "bootstrap.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#ifdef __APPLE__
#include <sys/utsname.h>
#endif

// Percorso canonico del binario del Kernel (repo unico)
#define KERNEL_BINARY "./bin/ice-kernel"

static int ice_create_shm_vault(const char *ws_id, uint32_t quota, const char *channel) {
    char shm_path[128];
    if (channel && channel[0] != '\0') {
        snprintf(shm_path, sizeof(shm_path), "%s%s_%s", SHM_VAULT_PREFIX, ws_id, channel);
    } else {
        snprintf(shm_path, sizeof(shm_path), "%s%s", SHM_VAULT_PREFIX, ws_id);
    }

    // Best-effort: remove stale SHM
    shm_unlink(shm_path);

    int fd = shm_open(shm_path, O_CREAT | O_RDWR, 0666);
    if (fd == -1) {
        fprintf(stderr, "[BOOTSTRAP] shm_open failed for %s: %s\n", shm_path, strerror(errno));
        return -1;
    }

    // Nota: Usiamo la struct ice_vault_t definita in ice_vault.h
    if (ftruncate(fd, sizeof(ice_vault_t)) != 0) {
        fprintf(stderr, "[BOOTSTRAP] ftruncate failed: %s\n", strerror(errno));
        close(fd);
        return -2;
    }

    ice_vault_t *v = mmap(NULL, sizeof(ice_vault_t), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (v == MAP_FAILED) {
        fprintf(stderr, "[BOOTSTRAP] mmap failed: %s\n", strerror(errno));
        close(fd);
        return -3;
    }

    // Inizializzazione pulita della memoria condivisa
    memset(v, 0, sizeof(ice_vault_t));
    v->status = ICE_STATE_HALT;
    v->energy_quota = quota;
    v->energy_consumed = 0;
    v->authority_lock = false;
    strncpy(v->workspace_id, ws_id, MAX_WS_ID - 1);
    
    // Log di sistema
    printf("[BOOTSTRAP] Vault created at %s (Size: %lu bytes)\n", shm_path, sizeof(ice_vault_t));

    munmap(v, sizeof(ice_vault_t));
    close(fd);
    return 0;
}

int main(int argc, char **argv) {
    printf("--- ICE BOOTSTRAP V1 (LAYER 0) ---\n");
    const char *ws_id = "default";
    bool raid = false;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--ws") == 0 && i + 1 < argc) {
            ws_id = argv[i + 1];
            i++;
        } else if (strcmp(argv[i], "--raid") == 0) {
            raid = true;
        }
    }

    // 1. Preboot: Inizializzazione stato locale
    ice_vault_t local_context = {
        .status = ICE_STATE_PREBOOT,
        .energy_quota = 1000,
        .authority_lock = false
    };

    if (ice_run_preboot_checks() != 0) {
        printf("[FATAL] Preboot invariants violated. System halt.\n");
        return EXIT_FAILURE;
    }

    // 2. Discovery & Environment setup
    ice_discover_environment(&local_context);

    // 3. Creazione del Vault fisico (SHM)
    if (ice_create_shm_vault(ws_id, local_context.energy_quota, NULL) != 0) {
        printf("[FATAL] Failed to allocate SHM Vault for WS: %s\n", ws_id);
        return EXIT_FAILURE;
    }
    if (raid) {
#ifdef __APPLE__
        printf("[BOOTSTRAP] RAID channels disabled on macOS (shm name length constraints). Using core vault only.\n");
#else
        const char *channels[] = {"stream", "brain", "audit", "cache", "control"};
        for (size_t i = 0; i < sizeof(channels) / sizeof(channels[0]); i++) {
            if (ice_create_shm_vault(ws_id, local_context.energy_quota, channels[i]) != 0) {
                printf("[FATAL] Failed to allocate SHM Vault %s for WS: %s\n", channels[i], ws_id);
                return EXIT_FAILURE;
            }
        }
#endif
    }

    // 4. Authority Handoff: Esecuzione del Kernel
    // Invece di chiamare bridge engine, usiamo execvp per trasformare 
    // il processo Bootstrap nel processo Kernel.
    printf("[BOOTSTRAP] Handing off authority to KERNEL...\n");
    
    char *kernel_args[] = { KERNEL_BINARY, "--ws", (char *)ws_id, NULL };
    
    if (execvp(KERNEL_BINARY, kernel_args) == -1) {
        perror("[FATAL] Failed to execute ICE-Kernel");
        return EXIT_FAILURE;
    }

    // Se arriviamo qui, execvp è fallita
    return EXIT_FAILURE;
}
