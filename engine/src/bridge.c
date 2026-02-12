#include "../include/engine_bridge.h"
#include <stdio.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <time.h>

static Vault* _vault = NULL;
static int _shm_fd = -1;

static Vault* attach_shm(const char* shm_path) {
    int fd = shm_open(shm_path, O_RDWR, 0666);
    if (fd == -1) return NULL;
    Vault* v = mmap(NULL, sizeof(Vault), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    close(fd);
    if (v == MAP_FAILED) return NULL;
    return v;
}

int yai_bridge_init(const char* ws_id) {
    char shm_path[128];
    snprintf(shm_path, sizeof(shm_path), "%s%s", SHM_VAULT_PREFIX, ws_id);

    _shm_fd = shm_open(shm_path, O_RDWR, 0666);
    if (_shm_fd == -1) return -1;

    _vault = mmap(NULL, sizeof(Vault), PROT_READ | PROT_WRITE, MAP_SHARED, _shm_fd, 0);
    if (_vault == MAP_FAILED) return -2;

    _vault->status = YAI_STATE_READY;
    return 0;
}

Vault* yai_bridge_attach(const char* ws_id, const char* channel) {
    char shm_path[128];
    char base_path[128];

    snprintf(base_path, sizeof(base_path), "%s%s", SHM_VAULT_PREFIX, ws_id);
    if (channel && channel[0] != '\0') {
        snprintf(shm_path, sizeof(shm_path), "%s%s_%s", SHM_VAULT_PREFIX, ws_id, channel);
    } else {
        snprintf(shm_path, sizeof(shm_path), "%s%s", SHM_VAULT_PREFIX, ws_id);
    }

    Vault* v = attach_shm(shm_path);
    if (!v && strcmp(shm_path, base_path) != 0) {
        v = attach_shm(base_path);
    }
    if (!v) return NULL;

    if (channel && strcmp(channel, "CORE") == 0) {
        _vault = v;
        _vault->status = YAI_STATE_READY;
    }
    return v;
}

Vault* yai_get_vault() { return _vault; }

bool yai_consume_energy(uint32_t amount) {
    if (!_vault || _vault->authority_lock) return false;
    if (_vault->energy_consumed + amount > _vault->energy_quota) return false;
    
    _vault->energy_consumed += amount;
    return true;
}

void yai_bridge_detach() {
    if (_vault) munmap(_vault, sizeof(Vault));
    if (_shm_fd != -1) close(_shm_fd);
}

void yai_audit_log_transition(const char* action, uint32_t prev_state, uint32_t new_state) {
    FILE* f = fopen("engine_runtime.trace", "a");
    if (!f) return;

    // timestamp, azione, stato_precedente, stato_nuovo, energia_attuale
    Vault* v = yai_get_vault();
    fprintf(f, "%ld,%s,%u,%u,%u\n",
            time(NULL),
            action,
            prev_state,
            new_state,
            v ? v->energy_consumed : 0);

    fclose(f);

    // Debug a video (opzionale)
    printf("[TLA-AUDIT] %s: %u -> %u\n", action, prev_state, new_state);
}
