#pragma once

#ifndef INCLUDE_POWER_DOMAIN_STATE_H
#define INCLUDE_POWER_DOMAIN_STATE_H

#include <power/types.h>

struct yai_power_domain_state {
    yai_power_domain_id_t domain_id;
    enum yai_power_state_kind state;
};

#endif
