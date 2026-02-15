#ifndef YAI_PROTOCOL_TRANSPORT_H
#define YAI_PROTOCOL_TRANSPORT_H

#include <stdint.h>

#define YAI_FRAME_MAGIC 0x59414950u   /* "YAIP" */
#define YAI_MAX_PAYLOAD 65536u        /* 64KB hard limit */

#pragma pack(push, 1)

/*
 * Envelope Sovrano (L2 <-> L3)
 * Dimensione fissa: 96 byte
 */
typedef struct yai_rpc_envelope {
    uint32_t magic;          /* Must be YAI_FRAME_MAGIC */
    uint32_t version;        /* YAI_PROTOCOL_IDS_VERSION */

    char     ws_id[36];      /* Workspace UUID */
    char     trace_id[36];   /* Trace UUID */

    uint32_t command_id;     /* From yai_protocol_ids.h */
    uint16_t role;           /* 0=Guest,1=Operator,2=Sovereign */
    uint8_t  arming;         /* 1=true */
    uint8_t  _pad;

    uint32_t payload_len;    /* Following payload size */
    uint32_t checksum;       /* Reserved (0 for now) */

} yai_rpc_envelope_t;

#pragma pack(pop)

_Static_assert(sizeof(yai_rpc_envelope_t) == 96,
               "yai_rpc_envelope_t must be 96 bytes");

#endif /* YAI_PROTOCOL_TRANSPORT_H */
