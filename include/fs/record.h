#pragma once

#ifndef INCLUDE_FS_RECORD_H
#define INCLUDE_FS_RECORD_H

#include <fs/types.h>

struct yai_fs_record {
    yai_inode_id_t inode_id;
    const char *path;
    enum yai_fs_node_kind kind;
};

#endif
