#pragma once

#ifndef INCLUDE_FS_ERRORS_H
#define INCLUDE_FS_ERRORS_H

enum yai_fs_error {
    YAI_FS_OK = 0,
    YAI_FS_ERR_NOT_FOUND = -1,
    YAI_FS_ERR_NOT_DIR = -2,
    YAI_FS_ERR_ACCESS = -3,
    YAI_FS_ERR_EXISTS = -4,
    YAI_FS_ERR_INVALID = -5,
    YAI_FS_ERR_BUSY = -6
};

#endif
