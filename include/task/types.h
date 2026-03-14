#pragma once

#ifndef INCLUDE_TASK_TYPES_H
#define INCLUDE_TASK_TYPES_H

#include <stdint.h>

typedef uint64_t yai_task_id_t;

enum yai_task_kind {
    YAI_TASK_UNKNOWN = 0,
    YAI_TASK_KERNEL,
    YAI_TASK_RUNTIME,
    YAI_TASK_WORKER,
    YAI_TASK_SUPERVISOR
};

#endif
