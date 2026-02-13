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
#include <time.h>

// -------------------------
// Error response writer (rpc.v1)
// -------------------------
int yai_rpc_write_error_v1(
    int fd,
    const char *ws_id,
    const char *trace_id,
    const char *code,
    const char *message,
    const char *detail_json
) {
    if (fd < 0) return -1;

    const char *ws = (ws_id && ws_id[0] != '\0') ? ws_id : "";
    const char *tr = (trace_id && trace_id[0] != '\0') ? trace_id : "";
    const char *c  = (code && code[0] != '\0') ? code : "ERR_UNKNOWN";
    const char *m  = (message && message[0] != '\0') ? message : "unknown error";
    const char *d  = (detail_json && detail_json[0] != '\0') ? detail_json : "null";

    char resp[YAI_RPC_ERRBUF];
    // message is treated as plain string; keep it short and stable.
    // detail_json must already be valid JSON (object/string/null)
    snprintf(resp, sizeof(resp),
        "{\"v\":%d,\"type\":\"error\","
        "\"code\":\"%s\",\"message\":\"%s\","
        "\"detail\":%s,"
        "\"trace_id\":\"%s\",\"ws_id\":\"%s\"}",
        YAI_RPC_V1, c, m, d, tr, ws
    );

    return yai_control_write_line(fd, resp);
}

// -------------------------
// trace_id generator (Phase-1: simple + deterministic enough)
// -------------------------
static void make_trace_id(char out[MAX_TRACE_ID]) {
    // time + pid + counter
    static uint64_t ctr = 0;
    uint64_t t = (uint64_t)time(NULL);
    uint64_t p = (uint64_t)getpid();
    ctr++;
    // fits in 64 chars easily
    snprintf(out, MAX_TRACE_ID, "tr-%llx-%llx-%llx",
             (unsigned long long)t,
             (unsigned long long)p,
             (unsigned long long)ctr);
}

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
    // 3. Ensure Run Dir + Lock + PID
    // -------------------------
    if (!yai_session_ensure_run_dir(&ws)) {
        fprintf(stderr, "[KERNEL] Failed to create run directory\n");
        return 1;
    }
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

    vault->status = YAI_STATE_READY;
    vault->authority_lock = false;

    fprintf(stderr, "--- YAI KERNEL C-CORE (L1) ---\n");
    fprintf(stderr, "[KERNEL] Workspace: %s\n", ws_id);

    // -------------------------
    // 6. FSM Transition
    // -------------------------
    if (yai_kernel_transition(vault, YAI_STATE_RUNNING) != 0) {
        fprintf(stderr, "[KERNEL] State transition failed\n");
        return 1;
    }

    // -------------------------
    // 7. Control Transport Init
    // -------------------------
    if (yai_control_listen(ws.control_sock) != 0) {
        fprintf(stderr, "[KERNEL] Control transport init failed\n");
        return 1;
    }

    fprintf(stderr, "[KERNEL] Awaiting control requests...\n");

    for (;;) {

        int cfd = yai_control_accept();
        if (cfd < 0) continue;

        // Phase-1 strict: handshake must be first message on the socket
        int handshaked = 0;

        for (;;) {
            char line[4096];
            ssize_t n = yai_control_read_line(cfd, line, sizeof(line));
            if (n <= 0) break; // EOF / timeout / error -> close

            char trace_id[MAX_TRACE_ID] = {0};
            make_trace_id(trace_id);

            char req_type[64] = {0};
            int v = yai_validate_envelope_v1(
                line,
                ws.ws_id,
                req_type,
                sizeof(req_type)
            );

            if (v != 0) {
                const char *code = "ERR_ENVELOPE_INVALID";
                const char *msg  = "invalid rpc envelope";
                const char *detail = "null";

                if (v == YAI_E_BAD_VERSION) { code = "ERR_PROTOCOL_VERSION"; msg = "bad protocol version"; }
                else if (v == YAI_E_MISSING_WS) { code = "ERR_WS_REQUIRED"; msg = "ws_id is required"; }
                else if (v == YAI_E_WS_MISMATCH) { code = "ERR_WS_MISMATCH"; msg = "ws_id mismatch"; }
                else if (v == YAI_E_MISSING_TYPE) { code = "ERR_REQUEST_MISSING"; msg = "request type missing"; }
                else if (v == YAI_E_TYPE_NOT_ALLOWED) { code = "ERR_REQUEST_NOT_ALLOWED"; msg = "request not allowed in phase1"; }
                else if (v == YAI_E_ROLE_REQUIRED) { code = "ERR_ROLE_REQUIRED"; msg = "role=operator required when arming=true"; }

                // audit event (schema_id + event_version enforced)
                yai_log_static(EV_TRANSITION_REJECTED, ws.ws_id, trace_id, "warn",
                               "rpc_rejected", "{\"reason\":\"validator\"}");

                (void)yai_rpc_write_error_v1(cfd, ws.ws_id, trace_id, code, msg, detail);
                // Phase-1: close on protocol error
                break;
            }

            // Handshake required first
            if (!handshaked && strcmp(req_type, "protocol_handshake") != 0) {
                yai_log_static(EV_TRANSITION_REJECTED, ws.ws_id, trace_id, "warn",
                               "rpc_rejected", "{\"reason\":\"handshake_required\"}");
                (void)yai_rpc_write_error_v1(
                    cfd, ws.ws_id, trace_id,
                    "ERR_HANDSHAKE_REQUIRED",
                    "protocol_handshake required before other requests",
                    "{\"expected_first\":\"protocol_handshake\"}"
                );
                break;
            }

            // ---- Dispatch ----
            if (strcmp(req_type, "protocol_handshake") == 0) {
                handshaked = 1;
                yai_control_write_line(cfd,
                    "{\"v\":1,\"type\":\"protocol_handshake_ok\",\"protocol_version\":1,\"server_version\":\"0.1\"}"
                );
            }
            else if (strcmp(req_type, "ping") == 0) {
                yai_control_write_line(cfd, "{\"v\":1,\"type\":\"pong\"}");
            }
            else if (strcmp(req_type, "status") == 0) {
                char resp[256];
                snprintf(resp, sizeof(resp),
                        "{\"v\":1,\"type\":\"status\",\"state\":\"down\",\"ws_id\":\"%s\"}",
                        ws.ws_id);
                yai_control_write_line(cfd, resp);
            }
            else {
                // should not happen due to allowlist, but keep safe
                (void)yai_rpc_write_error_v1(
                    cfd, ws.ws_id, trace_id,
                    "ERR_REQUEST_UNKNOWN",
                    "unknown request type",
                    "{\"phase\":\"phase1\"}"
                );
                break;
            }

            // Phase-1: one request per connection is OK; but allow multi after handshake.
            // If you want strict 1req/conn, uncomment:
            // break;
        }

        close(cfd);
    }

    return 0;
}
