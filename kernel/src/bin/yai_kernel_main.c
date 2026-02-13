#include "yai_vault.h"
#include "yai_kernel.h"
#include "kernel.h"
#include "transport.h"
#include "yai_session.h"
#include "control_transport.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>

static void print_workspace_paths(const yai_workspace_t* ws) {
    printf("WS ID:        %s\n", ws->ws_id);
    printf("RUN DIR:      %s\n", ws->run_dir);
    printf("CONTROL SOCK: %s\n", ws->control_sock);
    printf("LOCK FILE:    %s\n", ws->lock_file);
    printf("PID FILE:     %s\n", ws->pid_file);
}

int main(int argc, char **argv) {

    const char *ws_id = "default";
    int dry_run = 0;

    // -------------------------
    // 1. Arg Parsing
    // -------------------------
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--ws") == 0 && i + 1 < argc) {
            ws_id = argv[i + 1];
        }
        if (strcmp(argv[i], "--dry-run") == 0) {
            dry_run = 1;
        }
    }

    // -------------------------
    // 2. Workspace Path Resolution (L1 Canonical)
    // -------------------------
    yai_workspace_t ws;

    if (!yai_ws_build_paths(&ws, ws_id)) {
        fprintf(stderr, "[KERNEL] Invalid workspace id: %s\n", ws_id);
        return 1;
    }

    if (dry_run) {
        printf("--- YAI KERNEL DRY RUN ---\n");
        print_workspace_paths(&ws);
        return 0;
    }

    // -------------------------
    // 3. Ensure Run Dir
    // -------------------------
    if (!yai_session_ensure_run_dir(&ws)) {
        fprintf(stderr, "[KERNEL] Failed to create run directory\n");
        return 1;
    }
    // ---- Ownership check ----
    if (!yai_workspace_try_lock(&ws)) {
        fprintf(stderr, "[KERNEL] Workspace already running.\n");
        return 1;
    }

    if (!yai_workspace_write_pid(&ws)) {
        fprintf(stderr, "[KERNEL] Failed to write PID file.\n");
        return 1;
    }


    // -------------------------
    // 4. Vault SHM Init
    // -------------------------
    char shm_path[128];
    snprintf(shm_path, sizeof(shm_path), "%s%s", SHM_VAULT_PREFIX, ws_id);

    int fd = shm_open(shm_path, O_RDWR, 0666);
    if (fd == -1) {
        fprintf(stderr, "[KERNEL] shm_open failed for %s: %s\n",
                shm_path, strerror(errno));
        return 1;
    }

    yai_vault_t *vault = mmap(NULL,
                              sizeof(yai_vault_t),
                              PROT_READ | PROT_WRITE,
                              MAP_SHARED,
                              fd,
                              0);
    close(fd);

    if (vault == MAP_FAILED) {
        fprintf(stderr, "[KERNEL] mmap failed: %s\n", strerror(errno));
        return 1;
    }

    // -------------------------
    // 5. Vault Bootstrap
    // -------------------------
    if (vault->workspace_id[0] == '\0') {
        strncpy(vault->workspace_id, ws_id, MAX_WS_ID - 1);
    }

    if (vault->energy_quota == 0) {
        vault->energy_quota = 1000;
    }

    if (vault->logical_clock == 0) {
        vault->logical_clock = 0;
    }

    vault->status = YAI_STATE_READY;
    vault->authority_lock = false;

    printf("--- YAI KERNEL C-CORE (L1) ---\n");
    printf("[KERNEL] Workspace: %s\n", ws_id);

    // -------------------------
    // 6. FSM Transition
    // -------------------------
    if (yai_kernel_transition(vault, YAI_STATE_RUNNING) != 0) {
        fprintf(stderr, "[KERNEL] State transition failed\n");
        return 1;
    }



    // 7. Control Transport Init
    if (yai_control_listen(ws.control_sock) != 0) {
        fprintf(stderr, "[KERNEL] Control transport init failed\n");
        return 1;
    }

    printf("[KERNEL] Awaiting control requests...\n");

    for (;;) {

        int cfd = yai_control_accept();
        if (cfd < 0) continue;

        char line[4096];
        ssize_t n = yai_control_read_line(cfd, line, sizeof(line));

        if (n > 0) {

    char req_type[64] = {0};

    int v = yai_validate_envelope_v1(
        line,
        ws.ws_id,
        req_type,
        sizeof(req_type)
    );

    if (v != 0) {

        const char *err = "ERR_ENVELOPE_INVALID";

        if (v == -2) err = "ERR_PROTOCOL_VERSION";
        else if (v == -3) err = "ERR_WS_REQUIRED";
        else if (v == -5) err = "ERR_REQUEST_UNKNOWN";

        char resp[256];
        snprintf(resp, sizeof(resp),
                 "{\"v\":1,\"type\":\"error\",\"message\":\"%s\"}",
                 err);

        yai_control_write_line(cfd, resp);
        close(cfd);
        continue;
    }

    // ---- Dispatch ----

    if (strcmp(req_type, "ping") == 0) {
            yai_control_write_line(cfd,
                "{\"v\":1,\"type\":\"pong\"}");
        }

        else if (strcmp(req_type, "protocol_handshake") == 0) {
            yai_control_write_line(cfd,
                "{\"v\":1,\"type\":\"protocol_handshake\",\"protocol_version\":1,\"server_version\":\"0.1\"}");
        }

        else if (strcmp(req_type, "status") == 0) {
            char resp[256];
            snprintf(resp, sizeof(resp),
                    "{\"v\":1,\"type\":\"status\",\"state\":\"down\",\"ws_id\":\"%s\"}",
                    ws.ws_id);
            yai_control_write_line(cfd, resp);
        }

        else {
            yai_control_write_line(cfd,
                "{\"v\":1,\"type\":\"error\",\"message\":\"ERR_REQUEST_UNKNOWN\"}");
        }
    }


        close(cfd);
    }

    return 0;
}
