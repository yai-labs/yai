#pragma once

#ifndef INCLUDE_SUPERVISOR_SUPERVISOR_RECORD_H
#define INCLUDE_SUPERVISOR_SUPERVISOR_RECORD_H

struct yai_supervisor_record {
    const char *source;
    const char *action;
    int applied;
};

#endif
