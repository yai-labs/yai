#pragma once

#ifndef INCLUDE_TASK_FLAGS_H
#define INCLUDE_TASK_FLAGS_H

enum yai_task_flags {
    YAI_TASK_FLAG_PRIVILEGED = 1u << 0,
    YAI_TASK_FLAG_BOUNDARY   = 1u << 1,
    YAI_TASK_FLAG_RESTART    = 1u << 2,
    YAI_TASK_FLAG_TRACED     = 1u << 3
};

#endif
