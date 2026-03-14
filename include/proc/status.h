#pragma once

#ifndef INCLUDE_PROC_STATUS_H
#define INCLUDE_PROC_STATUS_H

enum yai_proc_status {
    YAI_PROC_UNKNOWN = 0,
    YAI_PROC_STARTING,
    YAI_PROC_RUNNING,
    YAI_PROC_STOPPED,
    YAI_PROC_EXITED,
    YAI_PROC_FAILED
};

#endif
