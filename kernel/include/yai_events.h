#pragma once
#include <stdint.h>

// Event Types - Tassonomia chiusa (Phase-1: stabile)
typedef enum {
    // Runtime
    EV_RUN_PROVISIONED = 100,
    EV_CONTEXT_RESOLVED,
    EV_VALIDATION_PASSED,
    EV_RUN_TERMINATED,

    EV_STATE_TRANSITION = 110,
    EV_TRANSITION_REJECTED = 111,

    // Cognitive
    EV_INFERENCE_STEP = 200,
    EV_DECISION_PROPOSED,

    // Memory
    EV_MEMORY_PROMOTED = 300,
    EV_MEMORY_EXPIRED,
    EV_MEMORY_INVALIDATED,

    // Capability
    EV_CAP_REQUESTED = 400,
    EV_CAP_GRANTED,
    EV_CAP_REVOKED
} yai_event_type_t;
