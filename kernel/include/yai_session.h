// kernel/include/yai_session.h
#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifndef MAX_SESSIONS
#define MAX_SESSIONS 16
#endif

#ifndef MAX_PATH_LEN
#define MAX_PATH_LEN 256
#endif

#ifndef MAX_WS_ID_LEN
#define MAX_WS_ID_LEN 32   // es: "dev", "prod", "ws-42"
#endif

typedef enum {
    YAI_WS_CREATED = 0,
    YAI_WS_ACTIVE  = 1,
    YAI_WS_CLOSED  = 2,
    YAI_WS_ERROR   = 3
} yai_ws_state_t;

// Capabilities come bitmask (stabile, deterministico)
typedef uint32_t yai_cap_mask_t;

enum {
    YAI_CAP_NONE            = 0u,
    YAI_CAP_RPC_PING        = 1u << 0,
    YAI_CAP_RPC_HANDSHAKE   = 1u << 1,
    YAI_CAP_RPC_STATUS      = 1u << 2,
    // riservati per blocchi futuri:
    YAI_CAP_RPC_UP          = 1u << 8,
    YAI_CAP_RPC_DOWN        = 1u << 9,
    YAI_CAP_RPC_SHELL       = 1u << 10,
};

typedef struct {
    char ws_id[MAX_WS_ID_LEN];         // canonico: stringa
    char run_dir[MAX_PATH_LEN];        // ~/.yai/run/<ws>/
    char control_sock[MAX_PATH_LEN];   // ~/.yai/run/<ws>/control.sock
    char lock_file[MAX_PATH_LEN];      // ~/.yai/run/<ws>/lock
    char pid_file[MAX_PATH_LEN];       // ~/.yai/run/<ws>/daemon.pid
    yai_ws_state_t state;
} yai_workspace_t;

typedef struct {
    uint32_t session_id;   // indice/handle interno
    uint32_t run_id;       // monotonic run counter (per ws)
    yai_workspace_t ws;
    yai_cap_mask_t caps;
    uint32_t owner_pid;
} yai_session_t;

// Registry globale (definito in un .c, NON qui)
extern yai_session_t g_session_registry[MAX_SESSIONS];

// --- API (blocco 1) ---
bool yai_ws_validate_id(const char* ws_id);
bool yai_ws_build_paths(yai_workspace_t* ws, const char* ws_id);

bool yai_session_acquire(yai_session_t** out, const char* ws_id);
void yai_session_release(yai_session_t* s);

bool yai_workspace_try_lock(const yai_workspace_t* ws);
void yai_workspace_unlock(const yai_workspace_t* ws);
bool yai_workspace_write_pid(const yai_workspace_t* ws);

bool yai_session_ensure_run_dir(const yai_workspace_t* ws);
