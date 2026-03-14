#pragma once

#ifndef INCLUDE_TASK_QUEUE_H
#define INCLUDE_TASK_QUEUE_H

#include <task/types.h>

struct yai_task_queue_entry {
    yai_task_id_t task_id;
    unsigned int priority;
};

#endif
