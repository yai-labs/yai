#pragma once

#ifndef INCLUDE_FIRMWARE_TYPES_H
#define INCLUDE_FIRMWARE_TYPES_H

#include <stdint.h>

typedef uint64_t yai_firmware_id_t;

enum yai_firmware_kind {
    YAI_FIRMWARE_UNKNOWN = 0,
    YAI_FIRMWARE_BOOT,
    YAI_FIRMWARE_DEVICE,
    YAI_FIRMWARE_RUNTIME,
    YAI_FIRMWARE_POLICY
};

#endif
