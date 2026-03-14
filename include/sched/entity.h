#pragma once

#ifndef INCLUDE_SCHED_ENTITY_H
#define INCLUDE_SCHED_ENTITY_H

#include <task/types.h>

struct yai_sched_entity {
    yai_task_id_t task_id;
    unsigned int weight;
    unsigned int class_id;
};

#endif
