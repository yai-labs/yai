#pragma once

#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#include <protocol/transport.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================
   CONTROL TRANSPORT (Binary Envelope v1)
   ============================================================ */

#ifndef YAI_CONTROL_BACKLOG
#define YAI_CONTROL_BACKLOG 16
#endif

/* Return codes */
#define YAI_CTL_OK                 0
#define YAI_CTL_ERR_ARG           -1
#define YAI_CTL_ERR_SOCKET        -2
#define YAI_CTL_ERR_BIND          -3
#define YAI_CTL_ERR_LISTEN        -4
#define YAI_CTL_ERR_ACCEPT        -5
#define YAI_CTL_ERR_READ          -6
#define YAI_CTL_ERR_WRITE         -7
#define YAI_CTL_ERR_OVERFLOW      -8

/* ============================================================
   Listener lifecycle
   ============================================================ */

int  yai_control_listen(const char *control_sock_path);
int  yai_control_accept(void);

/* ============================================================
   Frame I/O
   ============================================================ */

ssize_t yai_control_read_frame(
    int fd,
    yai_rpc_envelope_t *env,
    void *payload_buf,
    size_t payload_cap
);

int yai_control_write_frame(
    int fd,
    const yai_rpc_envelope_t *env,
    const void *payload
);

/* ============================================================
   Close helpers
   ============================================================ */

void yai_control_close_fd(int fd);
void yai_control_close(void);

#ifdef __cplusplus
}
#endif
