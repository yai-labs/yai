#pragma once

#ifndef INCLUDE_SECURITY_CAPABILITY_H
#define INCLUDE_SECURITY_CAPABILITY_H

enum yai_security_capability {
    YAI_CAP_NONE    = 0,
    YAI_CAP_MOUNT   = 1u << 0,
    YAI_CAP_NETWORK = 1u << 1,
    YAI_CAP_POLICY  = 1u << 2,
    YAI_CAP_TRACE   = 1u << 3,
    YAI_CAP_SUPERVISOR = 1u << 4
};

#endif
