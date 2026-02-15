#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "yai_kernel.h"
#include "control_transport.h"
#include <protocol/transport.h>
#include <protocol/protocol.h>
#include <protocol/yai_protocol_ids.h>

/*
 * Scrive un errore protocol-level usando Envelope Binario.
 * Il payload resta JSON (stringa).
 */
void yai_rpc_write_error_v1(
    int fd,
    const char* ws_id,
    const char* trace,
    const char* code,
    const char* msg,
    const char* actor
) {
    if (fd < 0)
        return;

    char payload[512];

    int n = snprintf(
        payload,
        sizeof(payload),
        "{\"type\":\"error\",\"error\":{\"code\":\"%s\",\"msg\":\"%s\",\"actor\":\"%s\"}}",
        code ? code : "UNKNOWN",
        msg ? msg : "unknown error",
        actor ? actor : "kernel"
    );

    if (n <= 0 || (size_t)n >= sizeof(payload))
        return;

    yai_rpc_envelope_t env;
    memset(&env, 0, sizeof(env));

    env.magic   = YAI_FRAME_MAGIC;
    env.version = YAI_PROTOCOL_VERSION;

    if (ws_id)
        strncpy(env.ws_id, ws_id, sizeof(env.ws_id) - 1);

    if (trace)
        strncpy(env.trace_id, trace, sizeof(env.trace_id) - 1);

    env.command_id  = YAI_CMD_PING;   // placeholder safe
    env.role        = 0;
    env.arming      = 0;
    env.payload_len = (uint32_t)strlen(payload);
    env.checksum    = 0;

    yai_control_write_frame(fd, &env, payload);
}
