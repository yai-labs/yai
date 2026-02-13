// kernel/include/transport.h
#ifndef YAI_TRANSPORT_H
#define YAI_TRANSPORT_H

#include <stdint.h>

typedef enum {
    CMD_PING = 0,
    CMD_AGENT_SPAWN = 1,
    CMD_STORAGE_SYNC = 2,
    CMD_SYS_HALT = 255
} CmdType;

typedef struct {
    CmdType type;
    uint32_t payload_size;
    uint8_t payload[1024];
} Packet;

int yai_transport_init(void);
void yai_transport_serve_once(void);

#endif
