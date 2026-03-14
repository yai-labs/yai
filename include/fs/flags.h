#pragma once

#ifndef INCLUDE_FS_FLAGS_H
#define INCLUDE_FS_FLAGS_H

enum yai_fs_open_flags {
    YAI_FS_OPEN_READ      = 1u << 0,
    YAI_FS_OPEN_WRITE     = 1u << 1,
    YAI_FS_OPEN_APPEND    = 1u << 2,
    YAI_FS_OPEN_CREATE    = 1u << 3,
    YAI_FS_OPEN_TRUNCATE  = 1u << 4,
    YAI_FS_OPEN_DIRECTORY = 1u << 5
};

enum yai_fs_walk_flags {
    YAI_FS_WALK_FOLLOW_SYMLINKS = 1u << 0,
    YAI_FS_WALK_REQUIRE_EXIST   = 1u << 1,
    YAI_FS_WALK_PARENT_ONLY     = 1u << 2
};

#endif
