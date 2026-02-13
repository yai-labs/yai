#ifndef YAI_VAULT_H
#define YAI_VAULT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <string.h>

// Vault ABI (authoritative offsets from Law)
#include "yai_vault_abi.h"

// Protocol authority (source of truth)
#include "../../law/specs/protocol/commands.h"

#ifdef __cplusplus
extern "C" {
#endif

// -----------------------------------------------------------------------------
// Phase-1: Vault is the shared L0 contract between Kernel and Engine.
// This header is intentionally "stable-first": we can add helpers/macros freely,
// but we do NOT drift offsets/sizes without updating law/specs/vault ABI.
// -----------------------------------------------------------------------------

// System constants for SHM access
#define SHM_VAULT_PREFIX "/yai_vault_"
#define MAX_WS_ID        64
#define MAX_TRACE_ID     64
#define MAX_ERR_MSG      256

// Optional, stable-ish meta (does NOT affect yai_vault_t layout)
#define YAI_VAULT_MAGIC   0x59414956u /* 'YAIV' */
#define YAI_VAULT_VERSION 1u

// Kernel states (derived from Kernel TLA+)
typedef enum {
    YAI_STATE_HALT = 0,
    YAI_STATE_PREBOOT,
    YAI_STATE_READY,
    YAI_STATE_HANDOFF_COMPLETE,
    YAI_STATE_RUNNING,
    YAI_STATE_SUSPENDED,
    YAI_STATE_ERROR
} yai_state_t;

// Engine commands (control -> engine), from protocol authority
typedef yai_command_id_t yai_command_t;

// Canonical vault header (L0 ABI) — provided by law/specs/vault.
// NOTE: This header is an ABI overlay and may be used by other components.
// In Phase-1 the kernel uses yai_vault_t directly; keep both definitions stable.
#define YAI_VAULT_HEADER_CORE_SIZE 28
struct yai_vault_header {
    uint32_t magic;
    uint16_t version;
    uint16_t flags;
    uint32_t state_id;
    uint32_t command_id;
    uint32_t trace_id;
    uint32_t energy_quota;
    uint32_t energy_used;
    uint8_t  _reserved[YAI_VAULT_HEADER_SIZE - YAI_VAULT_HEADER_CORE_SIZE];
};
typedef struct yai_vault_header yai_vault_header_t;

// Vault shared region (layout must stay compatible)
// IMPORTANT: do not reorder / resize fields without updating ABI asserts below.
typedef struct {
    uint32_t status;                 // current state (yai_state_t)
    uint32_t energy_quota;           // I-005 budget
    uint32_t energy_consumed;        // consumed so far
    char     workspace_id[MAX_WS_ID];// workspace scope
    char     trace_id[MAX_TRACE_ID]; // I-001 traceability
    bool     authority_lock;         // governance lock (true => deny effectful ops)
    uint8_t  _pad0[3];               // alignment
    uint32_t last_command_id;        // last/pending command id
    uint32_t command_seq;            // monotonic command sequence
    uint32_t last_processed_seq;     // last processed seq
    uint32_t last_result;            // last result code (0=ok, !=0 error)
    char     response_buffer[1024];  // engine->kernel response (Phase-1: freeform)
    char     last_error[MAX_ERR_MSG];// last error summary (stable, short)
    uint64_t logical_clock;          // I-002 deterministic clock (kernel-owned)
} yai_vault_t;

// -----------------------------------------------------------------------------
// Result codes (stable-ish, Phase-1 friendly)
// -----------------------------------------------------------------------------
enum {
    YAI_VAULT_OK  = 0u,
    YAI_VAULT_ERR = 1u
};

// -----------------------------------------------------------------------------
// Helpers (do not change ABI)
// -----------------------------------------------------------------------------
static inline void yai_vault_clear_error(yai_vault_t *v) {
    if (!v) return;
    v->last_error[0] = '\0';
    v->last_result = YAI_VAULT_OK;
}

static inline void yai_vault_set_error(yai_vault_t *v, const char *msg) {
    if (!v) return;
    v->last_result = YAI_VAULT_ERR;
    if (!msg) {
        v->last_error[0] = '\0';
        return;
    }
    // Always NUL-terminate
    strncpy(v->last_error, msg, MAX_ERR_MSG - 1);
    v->last_error[MAX_ERR_MSG - 1] = '\0';
}

static inline void yai_vault_set_ws(yai_vault_t *v, const char *ws) {
    if (!v || !ws) return;
    strncpy(v->workspace_id, ws, MAX_WS_ID - 1);
    v->workspace_id[MAX_WS_ID - 1] = '\0';
}

static inline void yai_vault_set_trace(yai_vault_t *v, const char *trace) {
    if (!v || !trace) return;
    strncpy(v->trace_id, trace, MAX_TRACE_ID - 1);
    v->trace_id[MAX_TRACE_ID - 1] = '\0';
}

static inline uint32_t yai_vault_next_seq(yai_vault_t *v) {
    if (!v) return 0;
    v->command_seq += 1u;
    return v->command_seq;
}

static inline bool yai_vault_is_authority_locked(const yai_vault_t *v) {
    return v && v->authority_lock;
}

// External-effect check helper (Phase-1 semantics)
// If a command is "external" and authority_lock is true -> deny.
static inline bool yai_vault_allows_command(const yai_vault_t *v, yai_command_id_t cmd) {
    if (!v) return false;
    uint32_t cls = yai_command_class_for(cmd);
    if ((cls & YAI_CMD_CLASS_EXTERNAL) && v->authority_lock) return false;
    return true;
}

// Phase-1 default bootstrap for a newly mapped vault.
// Does NOT touch offsets; just fills safe defaults.
static inline void yai_vault_bootstrap_defaults(yai_vault_t *v, const char *ws_id_opt) {
    if (!v) return;

    if (ws_id_opt && v->workspace_id[0] == '\0') {
        yai_vault_set_ws(v, ws_id_opt);
    }
    if (v->energy_quota == 0) v->energy_quota = 1000;
    // energy_consumed starts at 0 (leave if already set)
    // status set by kernel FSM (leave)
    // trace_id left empty unless provided by control plane
    // logical_clock is kernel-owned; leave if already non-zero
}

// -----------------------------------------------------------------------------
// ABI checks (header overlay)
// -----------------------------------------------------------------------------
_Static_assert(offsetof(struct yai_vault_header, magic)        == YAI_VAULT_OFF_MAGIC,        "vault ABI drift: magic");
_Static_assert(offsetof(struct yai_vault_header, state_id)     == YAI_VAULT_OFF_STATE_ID,     "vault ABI drift: state_id");
_Static_assert(offsetof(struct yai_vault_header, command_id)   == YAI_VAULT_OFF_COMMAND_ID,   "vault ABI drift: command_id");
_Static_assert(offsetof(struct yai_vault_header, trace_id)     == YAI_VAULT_OFF_TRACE_ID,     "vault ABI drift: trace_id");
_Static_assert(offsetof(struct yai_vault_header, energy_quota) == YAI_VAULT_OFF_ENERGY_QUOTA, "vault ABI drift: energy_quota");
_Static_assert(offsetof(struct yai_vault_header, energy_used)  == YAI_VAULT_OFF_ENERGY_USED,  "vault ABI drift: energy_used");
_Static_assert(sizeof(struct yai_vault_header)                == YAI_VAULT_HEADER_SIZE,      "vault ABI drift: header size");

// -----------------------------------------------------------------------------
// ABI checks (yai_vault_t layout)
// -----------------------------------------------------------------------------
_Static_assert(offsetof(yai_vault_t, status)             == 0,    "yai_vault_t.status offset mismatch");
_Static_assert(offsetof(yai_vault_t, last_command_id)    == 144,  "yai_vault_t.last_command_id offset mismatch");
_Static_assert(offsetof(yai_vault_t, command_seq)        == 148,  "yai_vault_t.command_seq offset mismatch");
_Static_assert(offsetof(yai_vault_t, last_processed_seq) == 152,  "yai_vault_t.last_processed_seq offset mismatch");
_Static_assert(offsetof(yai_vault_t, response_buffer)    == 160,  "yai_vault_t.response_buffer offset mismatch");
_Static_assert(offsetof(yai_vault_t, logical_clock)      == 1440, "yai_vault_t.logical_clock offset mismatch");
_Static_assert(sizeof(yai_vault_t)                       == 1448, "yai_vault_t size mismatch");

#ifdef __cplusplus
}
#endif

#endif
