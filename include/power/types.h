#pragma once

#ifndef INCLUDE_POWER_TYPES_H
#define INCLUDE_POWER_TYPES_H

#include <stdint.h>

typedef uint64_t yai_power_domain_id_t;

enum yai_power_state_kind {
    YAI_POWER_STATE_UNKNOWN = 0,
    YAI_POWER_STATE_OFF,
    YAI_POWER_STATE_IDLE,
    YAI_POWER_STATE_ACTIVE,
    YAI_POWER_STATE_SUSPENDED
};

#endif
