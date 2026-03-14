#pragma once

#ifndef INCLUDE_TRANSPORT_FRAME_H
#define INCLUDE_TRANSPORT_FRAME_H

#include <stdint.h>

struct yai_transport_frame {
    uint32_t kind;
    uint32_t size;
    const void *payload;
};

#endif
