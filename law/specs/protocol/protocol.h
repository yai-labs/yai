#ifndef YAI_PROTOCOL_H
#define YAI_PROTOCOL_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#include "yai_protocol_ids.h"
#include "transport.h"

/* ============================================================
   PROTOCOL ROOT DEFINITIONS
   ============================================================ */

#define YAI_PROTOCOL_MAGIC   0x59414950u /* 'Y''A''I''P' */
#define YAI_PROTOCOL_VERSION 1u

/* ============================================================
   SESSION STATE MACHINE (L3 CONTRACT)
   ============================================================ */

typedef enum {
    YAI_PROTO_STATE_IDLE       = 0,
    YAI_PROTO_STATE_HANDSHAKE  = 1,
    YAI_PROTO_STATE_READY      = 2,
    YAI_PROTO_STATE_ARMED      = 3,
    YAI_PROTO_STATE_ERROR      = 4
} yai_proto_state_t;

/* ============================================================
   CAPABILITIES (NEGOTIABLE FLAGS)
   ============================================================ */

#define YAI_CAP_STORAGE     (1u << 0)
#define YAI_CAP_INFERENCE   (1u << 1)
#define YAI_CAP_VECTOR      (1u << 2)
#define YAI_CAP_ENCRYPTION  (1u << 3)

/* ============================================================
   HANDSHAKE V1 (STRICT BINARY CONTRACT)
   ============================================================ */

#pragma pack(push, 1)

/* Client → Kernel */
typedef struct yai_handshake_req {
    uint32_t client_version;
    uint32_t capabilities_requested;
    char     client_name[32];
} yai_handshake_req_t;

/* Kernel → Client */
typedef struct yai_handshake_ack {
    uint32_t server_version;
    uint32_t capabilities_granted;
    uint16_t session_id;
    uint8_t  status;              /* yai_proto_state_t */
    uint8_t  _pad;                /* alignment */
} yai_handshake_ack_t;

#pragma pack(pop)

/* Static size validation */
_Static_assert(sizeof(yai_handshake_req_t) == 40,
               "yai_handshake_req_t must be 40 bytes");

_Static_assert(sizeof(yai_handshake_ack_t) == 12,
               "yai_handshake_ack_t must be 12 bytes");

#endif /* YAI_PROTOCOL_H */
