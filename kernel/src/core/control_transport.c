#include "control_transport.h"
#include <protocol/transport.h>

#include <sys/socket.h>
#include <sys/un.h>
#include <sys/stat.h>
#include <unistd.h>
#include <string.h>
#include <stdio.h>
#include <errno.h>

static int g_control_fd = -1;

static int read_all(int fd, void *buf, size_t n)
{
    char *p = buf;
    size_t off = 0;

    while (off < n)
    {
        ssize_t r = read(fd, p + off, n - off);
        if (r == 0)
        {
            fprintf(stderr, "[CONTROL] Peer closed connection\n");
            return YAI_CTL_ERR_READ;
        }
        if (r < 0)
        {
            if (errno == EINTR) continue;
            perror("[CONTROL] Read error");
            return YAI_CTL_ERR_READ;
        }
        off += (size_t)r;
    }
    return YAI_CTL_OK;
}

static int write_all(int fd, const void *buf, size_t n)
{
    const char *p = buf;
    size_t off = 0;

    while (off < n)
    {
        ssize_t w = write(fd, p + off, n - off);
        if (w < 0)
        {
            if (errno == EINTR) continue;
            perror("[CONTROL] Write error");
            return YAI_CTL_ERR_WRITE;
        }
        off += (size_t)w;
    }
    return YAI_CTL_OK;
}

int yai_control_listen(const char *path)
{
    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));

    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, path, sizeof(addr.sun_path) - 1);

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0)
        return YAI_CTL_ERR_SOCKET;

    unlink(path);

    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0)
    {
        perror("[CONTROL] Bind failed");
        close(fd);
        return YAI_CTL_ERR_BIND;
    }

    chmod(path, 0600);

    if (listen(fd, YAI_CONTROL_BACKLOG) < 0)
    {
        perror("[CONTROL] Listen failed");
        close(fd);
        return YAI_CTL_ERR_LISTEN;
    }

    g_control_fd = fd;
    fprintf(stderr, "[CONTROL] Listening at %s\n", path);
    return YAI_CTL_OK;
}

int yai_control_accept(void)
{
    int cfd = accept(g_control_fd, NULL, NULL);
    if (cfd >= 0)
        fprintf(stderr, "[CONTROL] Accepted fd=%d\n", cfd);
    return cfd;
}

ssize_t yai_control_read_frame(
    int fd,
    yai_rpc_envelope_t *env,
    void *payload_buf,
    size_t payload_cap)
{
    if (read_all(fd, env, sizeof(*env)) != YAI_CTL_OK)
        return YAI_CTL_ERR_READ;

    if (env->magic != YAI_FRAME_MAGIC)
    {
        fprintf(stderr, "[CONTROL] Invalid magic\n");
        return YAI_CTL_ERR_READ;
    }

    if (env->payload_len > YAI_MAX_PAYLOAD ||
        env->payload_len > payload_cap)
    {
        fprintf(stderr, "[CONTROL] Payload overflow %u\n",
                env->payload_len);
        return YAI_CTL_ERR_OVERFLOW;
    }

    if (env->payload_len > 0)
    {
        if (read_all(fd, payload_buf, env->payload_len) != YAI_CTL_OK)
            return YAI_CTL_ERR_READ;
    }

    return env->payload_len;
}

int yai_control_write_frame(
    int fd,
    const yai_rpc_envelope_t *env,
    const void *payload)
{
    if (write_all(fd, env, sizeof(*env)) != YAI_CTL_OK)
        return YAI_CTL_ERR_WRITE;

    if (env->payload_len > 0 && payload)
    {
        if (write_all(fd, payload, env->payload_len) != YAI_CTL_OK)
            return YAI_CTL_ERR_WRITE;
    }

    return YAI_CTL_OK;
}
