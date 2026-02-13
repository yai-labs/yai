// kernel/include/control_transport.h
#pragma once
#include <stddef.h>
#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

// Phase-1 transport policy: JSON Lines (one JSON object per '\n').
// Parsing/handshake/ws_id enforcement lives above transport; transport must be safe.

#ifndef YAI_CONTROL_BACKLOG
#define YAI_CONTROL_BACKLOG 16
#endif

#ifndef YAI_CONTROL_MAX_FRAME
#define YAI_CONTROL_MAX_FRAME (1024 * 1024) // 1 MiB hard cap
#endif

// Return codes (negative = error)
#define YAI_CTL_OK                 0
#define YAI_CTL_ERR_ARG           -1
#define YAI_CTL_ERR_SOCKET        -2
#define YAI_CTL_ERR_BIND          -3
#define YAI_CTL_ERR_LISTEN        -4
#define YAI_CTL_ERR_ACCEPT        -5
#define YAI_CTL_ERR_READ          -6
#define YAI_CTL_ERR_WRITE         -7
#define YAI_CTL_ERR_OVERFLOW      -8
#define YAI_CTL_ERR_TIMEOUT       -9

int yai_control_listen(const char *control_sock_path);
int yai_control_set_timeouts_ms(int fd, int recv_ms, int send_ms);
int yai_control_accept(void);

ssize_t yai_control_read_line(int fd, char *buf, size_t cap);
int yai_control_write_line(int fd, const char *line);

void yai_control_close_fd(int fd);
void yai_control_close(void);

#ifdef __cplusplus
}
#endif
