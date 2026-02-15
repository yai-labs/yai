#define _POSIX_C_SOURCE 200809L

#include "bootstrap.h"

#include <string.h>
#include <stdio.h>
#include <stdbool.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

#define SYSTEM_WS "system"
#define SHM_VAULT_PREFIX "/yai_vault_"

void yai_vault_populate(
    yai_vault_t *vault,
    const char *ws_id,
    uint32_t quota)
{
    memset(vault, 0, sizeof(yai_vault_t));

    vault->status = YAI_STATE_PREBOOT;
    vault->authority_lock = false;
    vault->energy_quota = quota;
    vault->energy_consumed = 0;
    vault->trace_id[0] = '\0';
    vault->logical_clock = 0;

    strncpy(vault->workspace_id, ws_id, MAX_WS_ID - 1);

    printf("[BOOT] Vault populated for WS: %s\n", ws_id);
}

/* ============================================================
   SYSTEM SHM (Machine Plane Vault)
   ============================================================ */

int yai_init_system_shm(void)
{
    char shm_path[128];

    snprintf(shm_path, sizeof(shm_path),
             "%s%s", SHM_VAULT_PREFIX, SYSTEM_WS);

    shm_unlink(shm_path);

    int fd = shm_open(shm_path, O_CREAT | O_RDWR, 0666);
    if (fd == -1)
        return -1;

    if (ftruncate(fd, sizeof(yai_vault_t)) != 0) {
        close(fd);
        return -2;
    }

    yai_vault_t *v = mmap(NULL,
                          sizeof(yai_vault_t),
                          PROT_READ | PROT_WRITE,
                          MAP_SHARED,
                          fd,
                          0);

    if (v == MAP_FAILED) {
        close(fd);
        return -3;
    }

    memset(v, 0, sizeof(yai_vault_t));
    v->status = YAI_STATE_PREBOOT;
    strncpy(v->workspace_id, SYSTEM_WS, MAX_WS_ID - 1);

    munmap(v, sizeof(yai_vault_t));
    close(fd);

    printf("[BOOT] System SHM initialized (%s)\n", shm_path);

    return 0;
}

/* ============================================================
   HANDOFF (not used yet, reserved)
   ============================================================ */

int yai_handoff_to_engine(yai_vault_t *vault)
{
    if (!vault)
        return -1;

    if (vault->authority_lock)
        return -1;

    vault->authority_lock = true;
    vault->status = YAI_STATE_HANDOFF_COMPLETE;

    return 0;
}
