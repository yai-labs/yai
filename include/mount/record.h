#pragma once

#ifndef INCLUDE_MOUNT_RECORD_H
#define INCLUDE_MOUNT_RECORD_H

#include <mount/types.h>

struct yai_mount_record {
    yai_mount_graph_id_t graph_id;
    enum yai_mount_kind kind;
    const char *target;
};

#endif
