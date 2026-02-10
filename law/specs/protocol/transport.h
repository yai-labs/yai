#ifndef YAI_PROTOCOL_TRANSPORT_H
#define YAI_PROTOCOL_TRANSPORT_H

#include <stdint.h>
#include <stddef.h>

typedef enum yai_msg_type {
    YAI_MSG_NONE = 0,
    YAI_MSG_HANDSHAKE = 1,
    YAI_MSG_HEARTBEAT = 2,
    YAI_MSG_COMMAND = 3,
    YAI_MSG_RESPONSE = 4,
    YAI_MSG_LOG = 5
} yai_msg_type_t;

#pragma pack(push, 1)
typedef struct yai_frame_header {
    uint32_t magic;
    uint32_t version;
    uint16_t msg_type;
    uint16_t flags;
    uint32_t length;
    uint32_t seq;
    uint32_t checksum;
} yai_frame_header_t;
#pragma pack(pop)

_Static_assert(sizeof(yai_frame_header_t) == 24, "yai_frame_header_t size mismatch");

#endif /* YAI_PROTOCOL_TRANSPORT_H */
