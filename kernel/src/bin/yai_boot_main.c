#define _POSIX_C_SOURCE 200809L
#include "yai_vault.h"
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
#include <limits.h>
#include <libgen.h>
#ifdef __APPLE__
#include <sys/utsname.h>
#endif

// Percorso canonico del binario del Kernel (repo unico)
#define KERNEL_BINARY "yai-kernel"

static const char *resolve_kernel_path(char **argv) {
    const char *env = getenv("YAI_KERNEL_BIN");
    if (env && env[0] != '\0') {
        return env;
    }

    static char buf[PATH_MAX];
    if (argv && argv[0] && strchr(argv[0], '/')) {
        char tmp[PATH_MAX];
        strncpy(tmp, argv[0], sizeof(tmp) - 1);
        tmp[sizeof(tmp) - 1] = '\0';
        char *dir = dirname(tmp);
        snprintf(buf, sizeof(buf), "%s/%s", dir, KERNEL_BINARY);
        return buf;
    }

    return KERNEL_BINARY;
}

static int yai_create_shm_vault(const char *ws_id, uint32_t quota, const char *channel) {
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

    // Nota: Usiamo la struct yai_vault_t definita in yai_vault.h
    if (ftruncate(fd, sizeof(yai_vault_t)) != 0) {
        fprintf(stderr, "[BOOTSTRAP] ftruncate failed: %s\n", strerror(errno));
        close(fd);
        return -2;
    }

    yai_vault_t *v = mmap(NULL, sizeof(yai_vault_t), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (v == MAP_FAILED) {
        fprintf(stderr, "[BOOTSTRAP] mmap failed: %s\n", strerror(errno));
        close(fd);
        return -3;
    }

    // Inizializzazione pulita della memoria condivisa
    memset(v, 0, sizeof(yai_vault_t));
    v->status = YAI_STATE_HALT;
    v->energy_quota = quota;
    v->energy_consumed = 0;
    v->authority_lock = false;
    strncpy(v->workspace_id, ws_id, MAX_WS_ID - 1);
    
    // Log di sistema
    printf("[BOOTSTRAP] Vault created at %s (Size: %lu bytes)\n", shm_path, sizeof(yai_vault_t));

    munmap(v, sizeof(yai_vault_t));
    close(fd);
    return 0;
}

int main(int argc, char **argv) {
    printf("--- YAI BOOTSTRAP V1 (LAYER 0) ---\n");
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
    yai_vault_t local_context = {
        .status = YAI_STATE_PREBOOT,
        .energy_quota = 1000,
        .authority_lock = false
    };

    if (yai_run_preboot_checks() != 0) {
        printf("[FATAL] Preboot invariants violated. System halt.\n");
        return EXIT_FAILURE;
    }

    // 2. Discovery & Environment setup
    yai_discover_environment(&local_context);

    // 3. Creazione del Vault fisico (SHM)
    if (yai_create_shm_vault(ws_id, local_context.energy_quota, NULL) != 0) {
        printf("[FATAL] Failed to allocate SHM Vault for WS: %s\n", ws_id);
        return EXIT_FAILURE;
    }
    if (raid) {
#ifdef __APPLE__
        printf("[BOOTSTRAP] RAID channels disabled on macOS (shm name length constraints). Using core vault only.\n");
#else
        const char *channels[] = {"stream", "brain", "audit", "cache", "control"};
        for (size_t i = 0; i < sizeof(channels) / sizeof(channels[0]); i++) {
            if (yai_create_shm_vault(ws_id, local_context.energy_quota, channels[i]) != 0) {
                printf("[FATAL] Failed to allocate SHM Vault %s for WS: %s\n", channels[i], ws_id);
                return EXIT_FAILURE;
            }
        }
#endif
    }

    printf("YAI_BOOT_OK ws=%s\n", ws_id);
    fflush(stdout);

    const char *no_exec = getenv("YAI_BOOT_NO_EXEC");
    if (no_exec && strcmp(no_exec, "1") == 0) {
        printf("[BOOTSTRAP] Kernel exec disabled by env. Exiting bootstrap.\n");
        return EXIT_SUCCESS;
    }

    // 4. Authority Handoff: Esecuzione del Kernel
    // Invece di chiamare bridge engine, usiamo execvp per trasformare 
    // il processo Bootstrap nel processo Kernel.
    printf("[BOOTSTRAP] Handing off authority to KERNEL...\n");

    const char *kernel_bin = resolve_kernel_path(argv);
    char *kernel_args[] = { (char *)kernel_bin, "--ws", (char *)ws_id, NULL };

    if (strchr(kernel_bin, '/')) {
        if (execv(kernel_bin, kernel_args) == -1) {
            perror("[FATAL] Failed to execute YAI-Kernel");
            return EXIT_FAILURE;
        }
    } else {
        if (execvp(kernel_bin, kernel_args) == -1) {
            perror("[FATAL] Failed to execute YAI-Kernel");
            return EXIT_FAILURE;
        }
    }

    // Se arriviamo qui, execvp è fallita
    return EXIT_FAILURE;
}
