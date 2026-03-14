#pragma once

#ifndef INCLUDE_SCHED_SLICE_H
#define INCLUDE_SCHED_SLICE_H

struct yai_sched_slice {
    unsigned int budget_ms;
    unsigned int latency_ms;
};

#endif
