#pragma once

#ifndef INCLUDE_TRANSPORT_MESSAGE_H
#define INCLUDE_TRANSPORT_MESSAGE_H

#include <stdint.h>

typedef uint64_t yai_transport_message_id_t;

struct yai_transport_message {
    yai_transport_message_id_t id;
    const char *topic;
};

#endif
