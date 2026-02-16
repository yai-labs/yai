#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <errno.h>
#include <stdint.h>
#include <limits.h>
#include <time.h>

#include <protocol/transport.h>
#include <protocol/yai_protocol_ids.h>

#define YAI_STATE_READY 1
#define YAI_CAP_NONE    0

static FILE *root_log = NULL;

/* ============================================================
   LOGGING
   ============================================================ */

static void log_init(const char *home)
{
    char log_path[PATH_MAX];
    snprintf(log_path, sizeof(log_path),
             "%s/.yai/run/root/root.log", home);

    root_log = fopen(log_path, "a");

    if (!root_log) {
        fprintf(stderr, "[ROOT] Failed to open log file: %s\n", log_path);
        root_log = stderr;
    } else {
        setvbuf(root_log, NULL, _IOLBF, 0); /* line buffered */
    }

    time_t now = time(NULL);
    fprintf(root_log, "\n=== ROOT START %ld ===\n", now);
    fflush(root_log);
}

#define LOG(fmt, ...)                                   \
    do {                                                \
        fprintf(stdout, fmt "\n", ##__VA_ARGS__);       \
        if (root_log)                                   \
            fprintf(root_log, fmt "\n", ##__VA_ARGS__); \
    } while (0)

/* ============================================================
   IO HELPERS
   ============================================================ */

static int read_all(int fd, void *buf, size_t n)
{
    size_t off = 0;
    char *p = buf;

    while (off < n) {
        ssize_t r = read(fd, p + off, n - off);
        if (r <= 0) {
            if (r < 0 && errno == EINTR) continue;
            return -1;
        }
        off += (size_t)r;
    }
    return 0;
}

static int write_all(int fd, const void *buf, size_t n)
{
    size_t off = 0;
    const char *p = buf;

    while (off < n) {
        ssize_t w = write(fd, p + off, n - off);
        if (w <= 0) {
            if (w < 0 && errno == EINTR) continue;
            return -1;
        }
        off += (size_t)w;
    }
    return 0;
}

/* ============================================================
   SEND FRAME
   ============================================================ */

static void send_frame(int fd,
                       const yai_rpc_envelope_t *req,
                       uint32_t cmd,
                       const void *payload,
                       uint32_t payload_len)
{
    yai_rpc_envelope_t resp;
    memset(&resp, 0, sizeof(resp));

    resp.magic       = YAI_FRAME_MAGIC;
    resp.version     = YAI_PROTOCOL_IDS_VERSION;
    resp.command_id  = cmd;
    resp.payload_len = payload_len;

    strncpy(resp.ws_id, req->ws_id, sizeof(resp.ws_id) - 1);
    strncpy(resp.trace_id, req->trace_id, sizeof(resp.trace_id) - 1);

    write_all(fd, &resp, sizeof(resp));

    if (payload_len > 0)
        write_all(fd, payload, payload_len);
}

/* ============================================================
   HANDLE CLIENT
   ============================================================ */

static void handle_client(int cfd)
{
    LOG("[ROOT] Client connected");

    int handshake_done = 0;

    for (;;) {

        yai_rpc_envelope_t env;

        if (read_all(cfd, &env, sizeof(env)) != 0)
            break;

        LOG("[ROOT] RECV cmd=%u len=%u role=%u arming=%u ws='%s'",
            env.command_id,
            env.payload_len,
            env.role,
            env.arming,
            env.ws_id);

        if (env.magic != YAI_FRAME_MAGIC ||
            env.version != YAI_PROTOCOL_IDS_VERSION) {
            LOG("[ROOT] Invalid magic/version");
            break;
        }

        if (env.payload_len > YAI_MAX_PAYLOAD) {
            LOG("[ROOT] Payload overflow");
            break;
        }

        char payload[YAI_MAX_PAYLOAD];
        if (env.payload_len > 0) {
            if (read_all(cfd, payload, env.payload_len) != 0)
                break;
        }

        /* SANITY CHECK */
        if (strchr(env.ws_id, '/')) {
            LOG("[ROOT] Invalid ws_id detected: %s", env.ws_id);
            break;
        }

        /* HANDSHAKE */
        if (env.command_id == YAI_CMD_HANDSHAKE) {

            struct {
                uint32_t version;
                uint32_t capabilities;
                uint32_t session_id;
                uint32_t status;
            } ack;

            memset(&ack, 0, sizeof(ack));
            ack.version      = YAI_PROTOCOL_IDS_VERSION;
            ack.capabilities = YAI_CAP_NONE;
            ack.session_id   = 1;
            ack.status       = YAI_STATE_READY;

            send_frame(cfd,
                       &env,
                       YAI_CMD_HANDSHAKE,
                       &ack,
                       sizeof(ack));

            LOG("[ROOT] Handshake ACK sent");
            handshake_done = 1;
            continue;
        }

        if (!handshake_done) {
            LOG("[ROOT] Command before handshake");
            break;
        }

        /* PING */
        if (env.command_id == YAI_CMD_PING) {
            const char *pong = "{\"pong\":true}";
            send_frame(cfd,
                       &env,
                       YAI_CMD_PING,
                       pong,
                       (uint32_t)strlen(pong));
            LOG("[ROOT] Pong sent");
            continue;
        }

        /* DEFAULT */
        const char *ok = "{\"status\":\"ok\"}";
        send_frame(cfd,
                   &env,
                   env.command_id,
                   ok,
                   (uint32_t)strlen(ok));
    }

    close(cfd);
    LOG("[ROOT] Client disconnected");
}

/* ============================================================
   MAIN
   ============================================================ */

int main(void)
{
    const char *home = getenv("HOME");
    if (!home) home = "/tmp";

    log_init(home);

    char sock_path[PATH_MAX];
    snprintf(sock_path, sizeof(sock_path),
             "%s/.yai/run/root/control.sock", home);

    unlink(sock_path);

    int sfd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (sfd < 0) {
        LOG("[ROOT] socket() failed");
        exit(1);
    }

    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, sock_path,
            sizeof(addr.sun_path) - 1);

    if (bind(sfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        LOG("[ROOT] bind() failed");
        exit(1);
    }

    if (listen(sfd, 16) < 0) {
        LOG("[ROOT] listen() failed");
        exit(1);
    }

    LOG("[ROOT] Listening on %s", sock_path);

    for (;;) {
        int cfd = accept(sfd, NULL, NULL);
        if (cfd >= 0)
            handle_client(cfd);
    }

    return 0;
}
