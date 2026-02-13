// kernel/src/core/control_transport.c
#include "control_transport.h"

#include <sys/stat.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <sys/time.h>

#include <unistd.h>
#include <string.h>
#include <stdio.h>
#include <errno.h>
#include <fcntl.h>

static int g_control_fd = -1;

static void best_effort_cloexec(int fd) {
#ifdef FD_CLOEXEC
    if (fd < 0) return;
    int flags = fcntl(fd, F_GETFD);
    if (flags >= 0) (void)fcntl(fd, F_SETFD, flags | FD_CLOEXEC);
#else
    (void)fd;
#endif
}

int yai_control_set_timeouts_ms(int fd, int recv_ms, int send_ms) {
    if (fd < 0) return YAI_CTL_ERR_ARG;

    if (recv_ms >= 0) {
        struct timeval tv;
        tv.tv_sec = recv_ms / 1000;
        tv.tv_usec = (recv_ms % 1000) * 1000;
        if (setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv)) < 0) {
            return YAI_CTL_ERR_SOCKET;
        }
    }

    if (send_ms >= 0) {
        struct timeval tv;
        tv.tv_sec = send_ms / 1000;
        tv.tv_usec = (send_ms % 1000) * 1000;
        if (setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv)) < 0) {
            return YAI_CTL_ERR_SOCKET;
        }
    }

    return YAI_CTL_OK;
}

int yai_control_listen(const char *path) {
    if (!path || path[0] == '\0') return YAI_CTL_ERR_ARG;

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;

    // sun_path must be NUL-terminated
    size_t plen = strlen(path);
    if (plen == 0 || plen >= sizeof(addr.sun_path)) return YAI_CTL_ERR_ARG;
    strncpy(addr.sun_path, path, sizeof(addr.sun_path) - 1);

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return YAI_CTL_ERR_SOCKET;

    // Replace previous listener if any
    if (g_control_fd >= 0) {
        close(g_control_fd);
        g_control_fd = -1;
    }

    (void)unlink(path);

    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return YAI_CTL_ERR_BIND;
    }

    // privileged control plane
    (void)chmod(path, 0600);

    if (listen(fd, YAI_CONTROL_BACKLOG) < 0) {
        close(fd);
        return YAI_CTL_ERR_LISTEN;
    }

    g_control_fd = fd;
    best_effort_cloexec(g_control_fd);

    fprintf(stderr, "[CONTROL] Listening at %s\n", path);
    return YAI_CTL_OK;
}

int yai_control_accept(void) {
    if (g_control_fd < 0) return YAI_CTL_ERR_ACCEPT;

    for (;;) {
        int cfd = accept(g_control_fd, NULL, NULL);
        if (cfd >= 0) {
            best_effort_cloexec(cfd);
            return cfd;
        }

        if (errno == EINTR) continue;
        return YAI_CTL_ERR_ACCEPT;
    }
}

static int drain_until_newline(int fd) {
    char tmp[256];
    for (;;) {
        ssize_t n = read(fd, tmp, sizeof(tmp));
        if (n == 0) return 0; // EOF
        if (n < 0) {
            if (errno == EINTR) continue;
            if (errno == EAGAIN || errno == EWOULDBLOCK) return YAI_CTL_ERR_TIMEOUT;
            return YAI_CTL_ERR_READ;
        }

        for (ssize_t i = 0; i < n; i++) {
            if (tmp[i] == '\n') return 0;
        }
    }
}

ssize_t yai_control_read_line(int fd, char *buf, size_t cap) {
    if (fd < 0 || !buf || cap < 2) return YAI_CTL_ERR_ARG;

    // enforce hard cap
    if (cap > (YAI_CONTROL_MAX_FRAME + 1)) cap = (YAI_CONTROL_MAX_FRAME + 1);

    size_t i = 0;

    for (;;) {
        char c = 0;
        ssize_t n = read(fd, &c, 1);

        if (n == 0) { // EOF
            buf[i] = '\0';
            return (ssize_t)i;
        }

        if (n < 0) {
            if (errno == EINTR) continue;
            if (errno == EAGAIN || errno == EWOULDBLOCK) return YAI_CTL_ERR_TIMEOUT;
            return YAI_CTL_ERR_READ;
        }

        if (c == '\n') {
            buf[i] = '\0';
            return (ssize_t)i;
        }

        // defensive: reject NUL inside frame
        if (c == '\0') {
            (void)drain_until_newline(fd);
            buf[0] = '\0';
            return YAI_CTL_ERR_READ;
        }

        if (i >= cap - 1) {
            (void)drain_until_newline(fd);
            buf[0] = '\0';
            return YAI_CTL_ERR_OVERFLOW;
        }

        buf[i++] = c;
    }
}

static int write_all(int fd, const char *p, size_t n) {
    while (n > 0) {
        ssize_t w = write(fd, p, n);
        if (w < 0) {
            if (errno == EINTR) continue;
            if (errno == EAGAIN || errno == EWOULDBLOCK) return YAI_CTL_ERR_TIMEOUT;
            return YAI_CTL_ERR_WRITE;
        }
        p += (size_t)w;
        n -= (size_t)w;
    }
    return YAI_CTL_OK;
}

int yai_control_write_line(int fd, const char *line) {
    if (fd < 0 || !line) return YAI_CTL_ERR_ARG;

    size_t len = strlen(line);
    if (len > YAI_CONTROL_MAX_FRAME) return YAI_CTL_ERR_OVERFLOW;

    int rc = write_all(fd, line, len);
    if (rc != YAI_CTL_OK) return rc;

    return write_all(fd, "\n", 1);
}

void yai_control_close_fd(int fd) {
    if (fd >= 0) (void)close(fd);
}

void yai_control_close(void) {
    if (g_control_fd >= 0) {
        close(g_control_fd);
        g_control_fd = -1;
    }
}
