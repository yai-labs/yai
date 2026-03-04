#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <dirent.h>
#include <limits.h>
#include <time.h>
#include <errno.h>

#include "yai_kernel.h"
#include "control_transport.h"
#include "yai_session.h"
#include "ws_id.h"

#include <transport.h>
#include <yai_protocol_ids.h>
#include <protocol.h>
#include <errors.h>
#include <roles.h>

#define YAI_BINARY_PAYLOAD_MAX 65536

/* ============================================================
   Internal: send a binary frame (envelope + payload)
============================================================ */
static void send_frame(
    int fd,
    const yai_rpc_envelope_t *req,
    uint32_t command_id,
    const void *payload,
    uint32_t payload_len
) {
    yai_rpc_envelope_t resp;
    memset(&resp, 0, sizeof(resp));

    resp.magic       = YAI_FRAME_MAGIC;
    resp.version     = YAI_PROTOCOL_IDS_VERSION;
    resp.command_id  = command_id;
    resp.payload_len = payload_len;

    strncpy(resp.ws_id, req->ws_id, sizeof(resp.ws_id) - 1);
    strncpy(resp.trace_id, req->trace_id, sizeof(resp.trace_id) - 1);

    resp.role      = req->role;
    resp.arming    = req->arming;
    resp.checksum  = 0;

    yai_control_write_frame(fd, &resp, payload);
}

static void send_error(
    int fd,
    const yai_rpc_envelope_t *req,
    uint32_t code,
    const char *reason
) {
    char payload[256];
    int n = snprintf(payload,
                     sizeof(payload),
                     "{\"status\":\"error\",\"code\":%u,\"reason\":\"%s\"}",
                     code,
                     reason ? reason : "unknown");
    if (n <= 0 || (size_t)n >= sizeof(payload))
        return;

    yai_rpc_envelope_t safe_req;
    memset(&safe_req, 0, sizeof(safe_req));
    if (req)
        safe_req = *req;

    send_frame(fd,
               &safe_req,
               safe_req.command_id ? safe_req.command_id : YAI_CMD_CONTROL,
               payload,
               (uint32_t)n);
}

static int valid_role(uint16_t role)
{
    return role == YAI_ROLE_NONE ||
           role == YAI_ROLE_USER ||
           role == YAI_ROLE_OPERATOR ||
           role == YAI_ROLE_SYSTEM;
}

static int mkdir_if_missing(const char *path, mode_t mode)
{
    struct stat st;
    if (stat(path, &st) == 0) {
        return S_ISDIR(st.st_mode) ? 0 : -1;
    }
    return mkdir(path, mode);
}

static int remove_tree(const char *path)
{
    DIR *d = opendir(path);
    if (!d) {
        return (errno == ENOENT) ? 0 : -1;
    }

    struct dirent *ent;
    while ((ent = readdir(d)) != NULL) {
        if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0)
            continue;

        char child[1024];
        int n = snprintf(child, sizeof(child), "%s/%s", path, ent->d_name);
        if (n <= 0 || (size_t)n >= sizeof(child)) {
            closedir(d);
            return -1;
        }

        struct stat st;
        if (stat(child, &st) != 0) {
            if (errno == ENOENT)
                continue;
            closedir(d);
            return -1;
        }

        if (S_ISDIR(st.st_mode)) {
            if (remove_tree(child) != 0) {
                closedir(d);
                return -1;
            }
        } else {
            if (unlink(child) != 0) {
                closedir(d);
                return -1;
            }
        }
    }

    closedir(d);
    return rmdir(path);
}

static int extract_json_string(const char *json, const char *key, char *out, size_t out_cap)
{
    if (!json || !key || !out || out_cap == 0)
        return -1;

    char needle[64];
    int nn = snprintf(needle, sizeof(needle), "\"%s\"", key);
    if (nn <= 0 || (size_t)nn >= sizeof(needle))
        return -1;

    const char *p = strstr(json, needle);
    if (!p)
        return -1;
    p = strchr(p, ':');
    if (!p)
        return -1;
    p++;
    while (*p == ' ' || *p == '\t')
        p++;
    if (*p != '"')
        return -1;
    p++;
    const char *q = strchr(p, '"');
    if (!q)
        return -1;

    size_t len = (size_t)(q - p);
    if (len >= out_cap)
        len = out_cap - 1;
    memcpy(out, p, len);
    out[len] = '\0';
    return 0;
}

static int extract_argv_first(const char *json, char *out, size_t out_cap)
{
    const char *p = strstr(json ? json : "", "\"argv\"");
    if (!p)
        return -1;
    p = strchr(p, '[');
    if (!p)
        return -1;
    p++;
    while (*p == ' ' || *p == '\t')
        p++;
    if (*p != '"')
        return -1;
    p++;
    const char *q = strchr(p, '"');
    if (!q)
        return -1;
    size_t len = (size_t)(q - p);
    if (len >= out_cap)
        len = out_cap - 1;
    memcpy(out, p, len);
    out[len] = '\0';
    return 0;
}

static int write_manifest(const char *ws_dir, const char *ws_id)
{
    char path[1024];
    int n = snprintf(path, sizeof(path), "%s/manifest.json", ws_dir);
    if (n <= 0 || (size_t)n >= sizeof(path))
        return -1;

    FILE *f = fopen(path, "w");
    if (!f)
        return -1;

    time_t now = time(NULL);
    fprintf(f,
            "{\n"
            "  \"ws_id\": \"%s\",\n"
            "  \"created_at\": %ld,\n"
            "  \"layout\": \"v2\"\n"
            "}\n",
            ws_id, (long)now);
    fclose(f);
    return 0;
}

static int handle_workspace_action(const char *home, const char *ws_id, const char *action)
{
    char yai_dir[1024];
    char run_dir[1024];
    char ws_dir[1024];
    char auth_dir[1024];
    char events_dir[1024];
    char engine_dir[1024];
    char logs_dir[1024];

    if (!home || !ws_id || !action)
        return -1;

    if (snprintf(yai_dir, sizeof(yai_dir), "%s/.yai", home) <= 0 ||
        snprintf(run_dir, sizeof(run_dir), "%s/.yai/run", home) <= 0 ||
        snprintf(ws_dir, sizeof(ws_dir), "%s/.yai/run/%s", home, ws_id) <= 0 ||
        snprintf(auth_dir, sizeof(auth_dir), "%s/authority", ws_dir) <= 0 ||
        snprintf(events_dir, sizeof(events_dir), "%s/events", ws_dir) <= 0 ||
        snprintf(engine_dir, sizeof(engine_dir), "%s/engine", ws_dir) <= 0 ||
        snprintf(logs_dir, sizeof(logs_dir), "%s/logs", ws_dir) <= 0) {
        return -1;
    }

    if (strcmp(action, "destroy") == 0) {
        return remove_tree(ws_dir);
    }

    if (strcmp(action, "reset") == 0) {
        (void)remove_tree(ws_dir);
        action = "create";
    }

    if (strcmp(action, "create") == 0) {
        if (mkdir_if_missing(yai_dir, 0755) != 0 ||
            mkdir_if_missing(run_dir, 0755) != 0 ||
            mkdir_if_missing(ws_dir, 0755) != 0 ||
            mkdir_if_missing(auth_dir, 0755) != 0 ||
            mkdir_if_missing(events_dir, 0755) != 0 ||
            mkdir_if_missing(engine_dir, 0755) != 0 ||
            mkdir_if_missing(logs_dir, 0755) != 0) {
            return -1;
        }
        return write_manifest(ws_dir, ws_id);
    }

    return -2;
}

/* ============================================================
   Binary connection handler (one-shot, Root or WS)
============================================================ */
void yai_kernel_handle_binary_connection(int cfd)
{
    yai_rpc_envelope_t env;
    char payload[YAI_BINARY_PAYLOAD_MAX + 1];

    ssize_t r = yai_control_read_frame(
        cfd,
        &env,
        payload,
        sizeof(payload) - 1
    );

    if (r < 0) {
        if (r == YAI_CTL_ERR_OVERFLOW)
            send_error(cfd, &env, YAI_E_PAYLOAD_TOO_BIG, "payload_too_big");
        close(cfd);
        return;
    }

    printf("[KERNEL] RECV cmd=%u len=%u role=%u arming=%u\n",
           env.command_id,
           env.payload_len,
           env.role,
           env.arming);

    /* -------- Strict protocol validation -------- */
    if (env.magic != YAI_FRAME_MAGIC) {
        send_error(cfd, &env, YAI_E_BAD_MAGIC, "bad_magic");
        close(cfd);
        return;
    }

    if (env.version != YAI_PROTOCOL_IDS_VERSION) {
        send_error(cfd, &env, YAI_E_BAD_VERSION, "bad_version");
        close(cfd);
        return;
    }

    if (env.payload_len > YAI_MAX_PAYLOAD) {
        send_error(cfd, &env, YAI_E_PAYLOAD_TOO_BIG, "payload_too_big");
        close(cfd);
        return;
    }

    if (env.checksum != 0) {
        send_error(cfd, &env, YAI_E_BAD_CHECKSUM, "bad_checksum");
        close(cfd);
        return;
    }

    if (env.arming > 1) {
        send_error(cfd, &env, YAI_E_ARMING_REQUIRED, "arming_flag_invalid");
        close(cfd);
        return;
    }

    if (!valid_role(env.role)) {
        send_error(cfd, &env, YAI_E_ROLE_REQUIRED, "role_invalid");
        close(cfd);
        return;
    }

    if (!yai_ws_id_is_valid(env.ws_id)) {
        send_error(cfd, &env, YAI_E_BAD_WS_ID, "bad_ws_id");
        close(cfd);
        return;
    }

    /* -------- HANDSHAKE -------- */
    if (env.command_id == YAI_CMD_HANDSHAKE) {

        if ((size_t)r != sizeof(yai_handshake_req_t)) {
            send_error(cfd, &env, YAI_E_PAYLOAD_TOO_BIG, "bad_handshake_payload_size");
            close(cfd);
            return;
        }

        yai_handshake_req_t *req = (yai_handshake_req_t *)payload;
        yai_handshake_ack_t ack;
        memset(&ack, 0, sizeof(ack));

        ack.server_version       = YAI_PROTOCOL_IDS_VERSION;
        ack.capabilities_granted = req->capabilities_requested;
        ack.session_id           = 1;
        ack.status               = YAI_PROTO_STATE_READY;

        send_frame(cfd, &env, YAI_CMD_HANDSHAKE, &ack, sizeof(ack));
        printf("[KERNEL] Handshake OK\n");
        close(cfd);
        return;
    }

    /* -------- Require authority for non-handshake calls -------- */
    if (env.role != YAI_ROLE_OPERATOR) {
        send_error(cfd, &env, YAI_E_ROLE_REQUIRED, "role_required");
        close(cfd);
        return;
    }

    if (!env.arming) {
        send_error(cfd, &env, YAI_E_ARMING_REQUIRED, "arming_required");
        close(cfd);
        return;
    }

    /* -------- PING / Root Status -------- */
    if (env.command_id == YAI_CMD_PING) {

        const char *pong;
        if (strcmp(env.ws_id, "root") == 0) {
            pong = "{\"status\":\"pong\",\"plane\":\"root\"}";
        } else {
            pong = "{\"status\":\"pong\"}";
        }

        send_frame(cfd, &env, YAI_CMD_PING, pong, (uint32_t)strlen(pong));
        printf("[KERNEL] Pong sent\n");
        close(cfd);
        return;
    }

    if (env.command_id == YAI_CMD_CONTROL_CALL) {
        char command_id[128] = {0};
        char action[64] = {0};
        if (extract_json_string(payload, "command_id", command_id, sizeof(command_id)) != 0) {
            send_error(cfd, &env, YAI_E_INTERNAL_ERROR, "bad_command_id");
            close(cfd);
            return;
        }

        if (strcmp(command_id, "yai.kernel.ws") == 0) {
            if (extract_argv_first(payload, action, sizeof(action)) != 0) {
                send_error(cfd, &env, YAI_E_INTERNAL_ERROR, "bad_args");
                close(cfd);
                return;
            }

            const char *home = getenv("HOME");
            if (!home || !home[0])
                home = "/tmp";

            int rc = handle_workspace_action(home, env.ws_id, action);
            if (rc == 0) {
                char out[256];
                int n = snprintf(out,
                                 sizeof(out),
                                 "{\"status\":\"ok\",\"code\":\"OK\",\"reason\":\"workspace_%s\",\"command_id\":\"yai.kernel.ws\",\"target_plane\":\"kernel\",\"ws_id\":\"%s\"}",
                                 action,
                                 env.ws_id);
                if (n > 0 && (size_t)n < sizeof(out)) {
                    send_frame(cfd, &env, env.command_id, out, (uint32_t)n);
                } else {
                    send_error(cfd, &env, YAI_E_INTERNAL_ERROR, "response_encode_failed");
                }
                close(cfd);
                return;
            }

            if (rc == -2) {
                const char *err = "{\"status\":\"error\",\"code\":\"BAD_ARGS\",\"reason\":\"unsupported_workspace_action\",\"command_id\":\"yai.kernel.ws\",\"target_plane\":\"kernel\"}";
                send_frame(cfd, &env, env.command_id, err, (uint32_t)strlen(err));
                close(cfd);
                return;
            }

            send_error(cfd, &env, YAI_E_INTERNAL_ERROR, "workspace_action_failed");
            close(cfd);
            return;
        }
    }

    /* -------- DEFAULT RESPONSE -------- */
    const char *ok = "{\"status\":\"ok\"}";
    send_frame(cfd, &env, env.command_id, ok, (uint32_t)strlen(ok));
    close(cfd);
}
