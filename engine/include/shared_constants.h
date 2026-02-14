#ifndef YAI_SHARED_CONSTANTS_H
#define YAI_SHARED_CONSTANTS_H

#include "../../law/specs/protocol/yai_protocol_ids.h"

// Allineamento rigoroso con transport.h
#define YAI_WS_ID_LEN    36
#define YAI_TRACE_ID_LEN 36

// Limiti di buffer basati sul protocollo
#define YAI_RPC_BUFFER_MAX 4096 
#define MAX_ERR_MSG        256

// Engine Lifecycle (Manteniamo questi per il Cortex)
typedef enum {
    ENGINE_OFFLINE = 0,
    ENGINE_READY   = 1,
    ENGINE_BUSY    = 2,
    ENGINE_ERROR   = 3
} engine_status_t;

#endif