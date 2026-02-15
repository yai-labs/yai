#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>

#include "yai_kernel.h"
#include "control_transport.h"
#include "yai_session.h"

#include <protocol/transport.h>
#include <protocol/yai_protocol_ids.h>
#include <protocol/protocol.h>

/* ============================================================
   SOCKET PATH
   ============================================================ */

static void build_socket_path(char *out, size_t cap, const char *ws_id)
{
    const char *home = getenv("HOME");
    if (!home)
        home = "/tmp";

    snprintf(out, cap, "%s/.yai/run/%s/control.sock",
             home,
             ws_id ? ws_id : "default");
}

/* ============================================================
   SAFE ENVELOPE COPY
   ============================================================ */

static void copy_envelope_meta(
    yai_rpc_envelope_t *resp,
    const yai_rpc_envelope_t *req,
    uint32_t command_id)
{
    memset(resp, 0, sizeof(*resp));

    resp->magic = YAI_FRAME_MAGIC;
    resp->version = YAI_PROTOCOL_IDS_VERSION;
    resp->command_id = command_id;

    strncpy(resp->ws_id, req->ws_id, sizeof(resp->ws_id) - 1);
    strncpy(resp->trace_id, req->trace_id, sizeof(resp->trace_id) - 1);
}

/* ============================================================
   MAIN
   ============================================================ */

int main(int argc, char **argv)
{
    const char *ws_id = "default";

    printf("\n[YAI] --- SOVEREIGN KERNEL RUNTIME V1.0 ---\n");

    char socket_path[512];
    build_socket_path(socket_path, sizeof(socket_path), ws_id);

    if (yai_control_listen(socket_path) != YAI_CTL_OK)
    {
        fprintf(stderr, "[FATAL] Control Plane failed on %s\n", socket_path);
        return 1;
    }

    printf("[YAI] Root Plane online: %s\n", socket_path);
    printf("[YAI] Awaiting secure RPC envelopes...\n\n");

    for (;;)
    {
        int cfd = yai_control_accept();
        if (cfd < 0)
            continue;

        fprintf(stderr, "[KERNEL] Client connected fd=%d\n", cfd);

        int handshake_done = 0;

        for (;;)
        {
            yai_rpc_envelope_t env;
            char payload[YAI_MAX_PAYLOAD];

            ssize_t plen = yai_control_read_frame(
                cfd,
                &env,
                payload,
                sizeof(payload));

            if (plen <= 0)
            {
                fprintf(stderr, "[KERNEL] Client closed fd=%d\n", cfd);
                break;
            }

            fprintf(stderr,
                    "[KERNEL] RECV cmd=%u len=%u role=%u arming=%u\n",
                    env.command_id,
                    env.payload_len,
                    env.role,
                    env.arming);

            /* ---------------- PROTOCOL VALIDATION ---------------- */

            if (env.version != YAI_PROTOCOL_IDS_VERSION)
            {
                fprintf(stderr, "[KERNEL] Protocol version mismatch\n");
                break;
            }

            /* ---------------- HANDSHAKE ---------------- */

            if (env.command_id == YAI_CMD_HANDSHAKE)
            {
                if ((size_t)plen != sizeof(yai_handshake_req_t))
                {
                    fprintf(stderr, "[KERNEL] Invalid handshake size\n");
                    break;
                }

                yai_handshake_ack_t ack;
                memset(&ack, 0, sizeof(ack));

                ack.server_version = YAI_PROTOCOL_IDS_VERSION;
                ack.capabilities_granted = YAI_CAP_NONE;
                ack.session_id = 1;
                ack.status = YAI_PROTO_STATE_READY;

                yai_rpc_envelope_t resp;
                copy_envelope_meta(&resp, &env, YAI_CMD_HANDSHAKE);
                resp.payload_len = sizeof(ack);

                yai_control_write_frame(cfd, &resp, &ack);

                handshake_done = 1;
                fprintf(stderr, "[KERNEL] Handshake OK\n");
                continue;
            }

            if (!handshake_done)
            {
                fprintf(stderr, "[KERNEL] Command before handshake\n");
                break;
            }

            /* ---------------- AUTHORITY ENFORCEMENT ---------------- */

            if (env.arming == 1 && env.role == 0)
            {
                fprintf(stderr, "[KERNEL] Authority violation\n");
                break;
            }

            /* ---------------- SESSION DISPATCH ---------------- */

            yai_session_dispatch(
                cfd,
                &env,
                payload);

            fprintf(stderr, "[KERNEL] Dispatch complete\n");
        }

        close(cfd);
    }

    return 0;
}
