#pragma once

#ifndef INCLUDE_FS_ACCESS_H
#define INCLUDE_FS_ACCESS_H

enum yai_fs_access_mask {
    YAI_FS_ACCESS_READ    = 1u << 0,
    YAI_FS_ACCESS_WRITE   = 1u << 1,
    YAI_FS_ACCESS_EXECUTE = 1u << 2,
    YAI_FS_ACCESS_ADMIN   = 1u << 3
};

struct yai_fs_access_request {
    const char *path;
    unsigned int mask;
};

#endif
