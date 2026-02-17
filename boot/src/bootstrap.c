#define _POSIX_C_SOURCE 200809L
#include "bootstrap.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/wait.h>
#include <limits.h>
#include <libgen.h>
#include <errno.h>

#define SYSTEM_WS "system"
#define SHM_PREFIX "/yai_vault_"

#define ROOT_BIN "yai-root-server"
#define KERNEL_BIN "yai-kernel"

static int spawn(const char *bin)
{
    pid_t pid = fork();
    if (pid < 0)
        return -1;

    if (pid == 0) {
        execl(bin, bin, "--master", NULL);
        perror("[BOOT-FATAL] exec failed");
        _exit(1);
    }

    return pid;
}

int yai_init_system_shm(void)
{
    char name[64];
    snprintf(name, sizeof(name), "%s%s", SHM_PREFIX, SYSTEM_WS);

    shm_unlink(name);

    int fd = shm_open(name, O_CREAT | O_RDWR, 0666);
    if (fd < 0)
        return -1;

    if (ftruncate(fd, sizeof(yai_vault_t)) != 0)
        return -2;

    yai_vault_t *v = mmap(NULL,
                          sizeof(yai_vault_t),
                          PROT_READ | PROT_WRITE,
                          MAP_SHARED,
                          fd,
                          0);

    if (v == MAP_FAILED)
        return -3;

    memset(v, 0, sizeof(yai_vault_t));
    strncpy(v->workspace_id, SYSTEM_WS, MAX_WS_ID - 1);

    munmap(v, sizeof(yai_vault_t));
    close(fd);

    printf("[BOOT] System SHM initialized (%s)\n", name);

    return 0;
}

static int get_exe_dir(char *out, size_t out_sz, const char *argv0)
{
    if (!argv0 || !argv0[0])
        return -1;

    char resolved[PATH_MAX];
    if (!realpath(argv0, resolved)) {
        perror("[BOOT-FATAL] realpath(argv0) failed");
        return -2;
    }

    /* dirname() may modify the passed buffer */
    char tmp[PATH_MAX];
    strncpy(tmp, resolved, sizeof(tmp) - 1);
    tmp[sizeof(tmp) - 1] = '\0';

    char *dir = dirname(tmp);
    if (!dir)
        return -3;

    if (snprintf(out, out_sz, "%s", dir) >= (int)out_sz)
        return -4;

    return 0;
}

int yai_spawn_planes(int *root_pid, int *kernel_pid, const char *argv0)
{
    char exe_dir[PATH_MAX];
    if (get_exe_dir(exe_dir, sizeof(exe_dir), argv0) != 0)
        return -10;

    char root_path[PATH_MAX];
    char kernel_path[PATH_MAX];

    snprintf(root_path, sizeof(root_path), "%s/%s", exe_dir, ROOT_BIN);
    snprintf(kernel_path, sizeof(kernel_path), "%s/%s", exe_dir, KERNEL_BIN);

    if (access(root_path, X_OK) != 0) {
        fprintf(stderr, "[BOOT-FATAL] missing/invalid root bin: %s\n", root_path);
        perror("[BOOT-FATAL] access(root) failed");
        return -1;
    }

    if (access(kernel_path, X_OK) != 0) {
        fprintf(stderr, "[BOOT-FATAL] missing/invalid kernel bin: %s\n", kernel_path);
        perror("[BOOT-FATAL] access(kernel) failed");
        return -2;
    }

    *root_pid = spawn(root_path);
    if (*root_pid < 0)
        return -3;

    *kernel_pid = spawn(kernel_path);
    if (*kernel_pid < 0)
        return -4;

    return 0;
}
