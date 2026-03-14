#pragma once

#ifndef INCLUDE_FS_NAMESPACE_VIEW_H
#define INCLUDE_FS_NAMESPACE_VIEW_H

#include <ns/types.h>

struct yai_fs_namespace_view {
    yai_ns_id_t ns_id;
    const char *root_path;
};

#endif
