#ifndef YAI_PROTOCOL_H
#define YAI_PROTOCOL_H

#include <stdint.h>

// Definizione del pacchetto che l'Engine manda al Runtime
typedef enum {
    CMD_EXECUTE_AGENT = 0x01,
    CMD_KILL_AGENT    = 0x02,
    CMD_INGEST_FILE   = 0x03,
    CMD_STORAGE_SYNC  = 0x04
} Command;

typedef struct {
    uint8_t  version;
    uint8_t  command;
    uint16_t payload_len;
    uint32_t request_id;
    uint8_t  data[]; // Flessibile
} Packet;

#endif