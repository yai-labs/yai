#pragma once

#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#include <protocol/runtime/rpc_runtime.h>



#define YAI_CONTROL_BACKLOG 16

/* Return codes */
#define YAI_CTL_OK            0
#define YAI_CTL_ERR_SOCKET   -1
#define YAI_CTL_ERR_BIND     -2
#define YAI_CTL_ERR_LISTEN   -3
#define YAI_CTL_ERR_READ     -4
#define YAI_CTL_ERR_WRITE    -5
#define YAI_CTL_ERR_OVERFLOW -6

/* Listen for control connections at a UNIX socket path */
int yai_control_listen_at(const char *path);

/* Read a single frame (envelope + payload) */
ssize_t yai_control_read_frame(
    int fd,
    yai_rpc_envelope_t *env,
    void *payload_buf,
    size_t payload_cap
);

/* Write a single frame (envelope + payload) */
int yai_control_write_frame(
    int fd,
    const yai_rpc_envelope_t *env,
    const void *payload
);
