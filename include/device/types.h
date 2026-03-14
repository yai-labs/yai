#pragma once

#ifndef INCLUDE_DEVICE_TYPES_H
#define INCLUDE_DEVICE_TYPES_H

#include <stdint.h>

typedef uint64_t yai_device_id_t;
typedef uint64_t yai_driver_id_t;

enum yai_device_kind {
    YAI_DEVICE_UNKNOWN = 0,
    YAI_DEVICE_BLOCK,
    YAI_DEVICE_CHAR,
    YAI_DEVICE_NET,
    YAI_DEVICE_INPUT,
    YAI_DEVICE_VIRT,
    YAI_DEVICE_PLATFORM
};

#endif
