#pragma once

#ifndef INCLUDE_FS_WALK_H
#define INCLUDE_FS_WALK_H

#include <fs/types.h>

struct yai_fs_walk_result {
    yai_dentry_id_t dentry_id;
    yai_inode_id_t inode_id;
    enum yai_fs_node_kind kind;
};

#endif
