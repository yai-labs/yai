#ifndef YAI_VAULT_H
#define YAI_VAULT_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <string.h>

#include "yai_vault_abi.h"
#include "../../law/specs/protocol/yai_protocol_ids.h" // <--- Aggiornato: Sorgente Unica

#ifdef __cplusplus
extern "C" {
#endif

/* --- COSTANTI DI SISTEMA --- */
#define SHM_VAULT_PREFIX "/yai_vault_"
#define MAX_WS_ID        64
#define MAX_TRACE_ID     64
#define MAX_ERR_MSG      256
#define YAI_VAULT_MAGIC   0x59414956u 
#define YAI_VAULT_VERSION 1u

/* --- STATI KERNEL (TLA+ Derived) --- */
typedef enum {
    YAI_STATE_HALT = 0,
    YAI_STATE_PREBOOT,
    YAI_STATE_READY,
    YAI_STATE_HANDOFF_COMPLETE,
    YAI_STATE_RUNNING,
    YAI_STATE_SUSPENDED,
    YAI_STATE_ERROR
} yai_state_t;

/* --- LAYOUT ABI (L0) --- */
#pragma pack(push, 1) // Forza l'allineamento ABI rigoroso per SHM
typedef struct {
    uint32_t status;                 
    uint32_t energy_quota;           
    uint32_t energy_consumed;        
    char     workspace_id[MAX_WS_ID];
    char     trace_id[MAX_TRACE_ID]; 
    bool     authority_lock;         
    uint8_t  _pad0[3];               
    uint32_t last_command_id;        
    uint32_t command_seq;            
    uint32_t last_processed_seq;     
    uint32_t last_result;            
    char     response_buffer[1024];  
    char     last_error[MAX_ERR_MSG];
    uint64_t logical_clock;          
} yai_vault_t;
#pragma pack(pop)

/* --- AUTHORITY ENFORCEMENT --- */

/**
 * Risolve la classe del comando usando i bitmask definiti nel protocollo.
 * Questo permette al Vault di sapere se un comando richiede accesso esterno (LLM).
 */
static inline uint32_t yai_resolve_command_class(uint32_t cmd) {
    // Estraiamo la classe (High Byte)
    uint32_t cls_prefix = cmd & 0xFF00u;
    
    if (cls_prefix == YAI_CMD_CLASS_PROVIDER || cls_prefix == 0x0400u) {
        return 0x02; // Identificato internamente come EXTERNAL/SOVEREIGN
    }
    return 0x01; // INTERNAL
}

/**
 * Invariante I-006: Controllo Accessi
 */
static inline bool yai_vault_allows_command(const yai_vault_t *v, uint32_t cmd) {
    if (!v) return false;
    
    uint32_t cls = yai_resolve_command_class(cmd);
    
    // Se l'Authority Lock è attivo, blocca comandi con effetti esterni (es. Inference)
    if (v->authority_lock && (cls == 0x02)) {
        return false; 
    }
    return true;
}

/* --- HELPERS DI BOOTSTRAP --- */

static inline void yai_vault_bootstrap_defaults(yai_vault_t *v, const char *ws_id_opt) {
    if (!v) return;
    memset(v, 0, sizeof(yai_vault_t));
    
    v->status = YAI_STATE_PREBOOT;
    v->energy_quota = 1000;
    v->authority_lock = true; // Sicurezza: locked di default fino a handshake
    
    if (ws_id_opt) {
        strncpy(v->workspace_id, ws_id_opt, MAX_WS_ID - 1);
    }
}

static inline void yai_vault_set_error(yai_vault_t *v, const char *msg) {
    if (!v) return;
    v->last_result = 1; 
    strncpy(v->last_error, msg ? msg : "unknown_err", MAX_ERR_MSG - 1);
}

#ifdef __cplusplus
}
#endif
#endif