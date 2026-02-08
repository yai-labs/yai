#ifndef ICE_TRANSPORT_H
#define ICE_TRANSPORT_H

#include <stdint.h>

typedef enum {
    CMD_PING = 0,
    CMD_AGENT_SPAWN = 1,
    CMD_STORAGE_SYNC = 2,
    CMD_SYS_HALT = 255
} IceCmdType;

typedef struct {
    IceCmdType type;
    uint32_t payload_size;
    uint8_t payload[1024];
} IcePacket;

int ice_transport_init(void);
void ice_transport_serve_once(void);

#endif
