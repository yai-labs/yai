/* law/specs/protocol/transport.h (INVARIATO: lo hai già giusto) */
#ifndef YAI_PROTOCOL_TRANSPORT_H
#define YAI_PROTOCOL_TRANSPORT_H

#include <stdint.h>

#define YAI_FRAME_MAGIC    0x59414950  // "YAIP" (YAI Protocol)
#define YAI_MAX_PAYLOAD    65536       // 64KB Hard Limit per sicurezza

#pragma pack(push, 1)

/**
 * L'Envelope Sovrano (L2 <-> L3)
 * Totale: 96 byte (Multiplo di 8, allineamento perfetto per SHM)
 */
typedef struct yai_rpc_envelope {
    uint32_t magic;         // Deve essere YAI_FRAME_MAGIC
    uint32_t version;       // Protocol version (es. 1)
    char     ws_id[36];     // UUID del Workspace (stringa, non NULL-terminated garantito)
    char     trace_id[36];  // Trace ID (stringa, non NULL-terminated garantito)

    uint32_t command_id;    // Da yai_protocol_ids.h
    uint16_t role;          // 0: Guest, 1: Operator, 2: Sovereign
    uint8_t  arming;        // 1 = true, 0 = false
    uint8_t  _pad;          // Padding per allineamento a 8 byte

    uint32_t payload_len;   // Lunghezza del JSON che segue (Max YAI_MAX_PAYLOAD)
    uint32_t checksum;      // Integrità dell'envelope + payload (0 per ora)
} yai_rpc_envelope_t;

#pragma pack(pop)

_Static_assert(sizeof(yai_rpc_envelope_t) == 96, "yai_rpc_envelope_t size must be 96 bytes");

#endif
