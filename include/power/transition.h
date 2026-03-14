#pragma once

#ifndef INCLUDE_POWER_TRANSITION_H
#define INCLUDE_POWER_TRANSITION_H

#include <power/types.h>

struct yai_power_transition {
    enum yai_power_state_kind from_state;
    enum yai_power_state_kind to_state;
};

#endif
