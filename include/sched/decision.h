#pragma once

#ifndef INCLUDE_SCHED_DECISION_H
#define INCLUDE_SCHED_DECISION_H

#include <task/types.h>

struct yai_sched_decision {
    yai_task_id_t selected_task;
    unsigned int cpu_hint;
};

#endif
