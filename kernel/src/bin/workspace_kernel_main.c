#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <errno.h>
#include <limits.h>

#include "yai_kernel.h"
#include "control_transport.h"
#include "yai_session.h"

#include <protocol/transport.h>
#include <protocol/yai_protocol_ids.h>
#include <protocol/protocol.h>   /* 👈 UNICA DEFINIZIONE HANDSHAKE */

/* ============================================================
   Ensure runtime directory
============================================================ */

static int ensure_kernel_directory(void)
{
    const char *home = getenv("HOME");
    if (!home) return -1;

    char path[PATH_MAX];

    snprintf(path, sizeof(path),
             "%s/.yai/run/kernel", home);

    struct stat st;

    if (stat(path, &st) == 0)
        return S_ISDIR(st.st_mode) ? 0 : -1;

    return mkdir(path, 0755);
}

/* ============================================================
   HANDLE CLIENT
============================================================ */

static void handle_client(int listen_fd)
{
    int cfd = accept(listen_fd, NULL, NULL);
    if (cfd < 0)
        return;

    int handshake_done = 0;

    for (;;)
    {
        yai_rpc_envelope_t env;
        char payload[YAI_MAX_PAYLOAD];

        ssize_t plen =
            yai_control_read_frame(
                cfd,
                &env,
                payload,
                sizeof(payload));

        if (plen < 0)
            break;

        if (env.magic != YAI_FRAME_MAGIC ||
            env.version != YAI_PROTOCOL_IDS_VERSION)
            break;

        /* ---------------- HANDSHAKE ---------------- */

        if (env.command_id == YAI_CMD_HANDSHAKE)
        {
            yai_handshake_ack_t ack;
            memset(&ack, 0, sizeof(ack));

            ack.server_version       = YAI_PROTOCOL_IDS_VERSION;
            ack.capabilities_granted = 0;
            ack.session_id           = 1;
            ack.status               = YAI_PROTO_STATE_READY;

            yai_rpc_envelope_t resp;
            memset(&resp, 0, sizeof(resp));

            resp.magic       = YAI_FRAME_MAGIC;
            resp.version     = YAI_PROTOCOL_IDS_VERSION;
            resp.command_id  = YAI_CMD_HANDSHAKE;
            resp.payload_len = sizeof(ack);

            strncpy(resp.ws_id,
                    env.ws_id,
                    sizeof(resp.ws_id) - 1);

            strncpy(resp.trace_id,
                    env.trace_id,
                    sizeof(resp.trace_id) - 1);

            yai_control_write_frame(cfd, &resp, &ack);

            handshake_done = 1;
            continue;
        }

        if (!handshake_done)
            break;

        /* ---------------- NORMAL DISPATCH ---------------- */

        yai_session_dispatch(cfd, &env, payload);
    }

    close(cfd);
}

/* ============================================================
   MAIN
============================================================ */

int main(int argc, char **argv)
{
    int master_mode = 0;

    for (int i = 1; i < argc; i++)
        if (strcmp(argv[i], "--master") == 0)
            master_mode = 1;

    if (!master_mode)
    {
        fprintf(stderr,
                "[KERNEL] Must be started by boot (--master)\n");
        return 1;
    }

    if (ensure_kernel_directory() != 0)
    {
        fprintf(stderr,
                "[KERNEL] Failed to create runtime directory\n");
        return 1;
    }

    const char *home = getenv("HOME");
    if (!home) home = "/tmp";

    char kernel_path[PATH_MAX];

    snprintf(kernel_path,
             sizeof(kernel_path),
             "%s/.yai/run/kernel/control.sock",
             home);

    unlink(kernel_path);

    int listen_fd =
        yai_control_listen_at(kernel_path);

    if (listen_fd < 0)
    {
        fprintf(stderr,
                "[KERNEL] Failed to start kernel plane\n");
        return 1;
    }

    printf("[KERNEL] Kernel Plane online (%s)\n",
           kernel_path);

    for (;;)
        handle_client(listen_fd);

    return 0;
}
