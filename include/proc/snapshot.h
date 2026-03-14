#pragma once

#ifndef INCLUDE_PROC_SNAPSHOT_H
#define INCLUDE_PROC_SNAPSHOT_H

#include <proc/pid.h>
#include <proc/status.h>

struct yai_proc_snapshot {
    yai_pid_t pid;
    enum yai_proc_status status;
    const char *name;
};

#endif
