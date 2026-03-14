#pragma once

#ifndef INCLUDE_TRANSPORT_QUEUE_H
#define INCLUDE_TRANSPORT_QUEUE_H

#include <transport/message.h>

struct yai_transport_queue_entry {
    yai_transport_message_id_t message_id;
    unsigned int priority;
};

#endif
