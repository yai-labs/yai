#pragma once

#ifndef INCLUDE_FS_TYPES_H
#define INCLUDE_FS_TYPES_H

#include <stdint.h>

typedef uint64_t yai_inode_id_t;
typedef uint64_t yai_super_id_t;
typedef uint64_t yai_mount_id_t;
typedef uint64_t yai_dentry_id_t;

enum yai_fs_node_kind {
    YAI_FS_NODE_UNKNOWN = 0,
    YAI_FS_NODE_FILE,
    YAI_FS_NODE_DIR,
    YAI_FS_NODE_SYMLINK,
    YAI_FS_NODE_DEVICE,
    YAI_FS_NODE_PIPE,
    YAI_FS_NODE_SOCKET
};

#endif
