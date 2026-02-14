#ifndef YAI_SESSION_H
#define YAI_SESSION_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#define MAX_SESSIONS    32
#define MAX_WS_ID_LEN   64
#define MAX_PATH_LEN    512

// --- Capabilities Bitmask ---
typedef uint32_t yai_cap_mask_t;
#define YAI_CAP_NONE            0u
#define YAI_CAP_RPC_PING        (1u << 0)
#define YAI_CAP_RPC_HANDSHAKE   (1u << 1)
#define YAI_CAP_RPC_STATUS      (1u << 2)

typedef enum {
    YAI_WS_CREATED = 0,
    YAI_WS_ACTIVE,
    YAI_WS_ERROR
} yai_ws_state_t;

typedef struct {
    char ws_id[MAX_WS_ID_LEN];
    char run_dir[MAX_PATH_LEN];
    char control_sock[MAX_PATH_LEN]; // Mantieni per compatibilità ABI
    char lock_file[MAX_PATH_LEN];
    char pid_file[MAX_PATH_LEN];      // <--- AGGIUNTO (Risolve errore 1)
    yai_ws_state_t state;
} yai_workspace_t;

typedef struct {
    uint32_t session_id;
    uint32_t run_id;
    yai_workspace_t ws;
    yai_cap_mask_t caps;
    uint32_t owner_pid;              // <--- AGGIUNTO (Risolve errore 2)
} yai_session_t;

extern yai_session_t g_session_registry[MAX_SESSIONS];

// --- API Prototypes ---
bool yai_ws_validate_id(const char* ws_id);
bool yai_ws_build_paths(yai_workspace_t* ws, const char* ws_id);
bool yai_session_ensure_run_dir(const yai_workspace_t* ws);
bool yai_workspace_try_lock(const yai_workspace_t* ws);
void yai_workspace_unlock(const yai_workspace_t* ws);
bool yai_workspace_write_pid(const yai_workspace_t* ws);
bool yai_session_acquire(yai_session_t** out, const char* ws_id);
void yai_session_release(yai_session_t* s);
void yai_session_dispatch(int client_fd, const char* ws_id, uint32_t cmd_id, const char* payload);
#endif