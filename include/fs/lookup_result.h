#pragma once

#ifndef INCLUDE_FS_LOOKUP_RESULT_H
#define INCLUDE_FS_LOOKUP_RESULT_H

#include <fs/result.h>
#include <fs/types.h>

struct yai_fs_lookup_result {
    enum yai_fs_result result;
    yai_dentry_id_t dentry_id;
    yai_inode_id_t inode_id;
};

#endif
