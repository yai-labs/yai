#pragma once

#ifndef INCLUDE_FS_NAMESPACE_BINDING_H
#define INCLUDE_FS_NAMESPACE_BINDING_H

#include <fs/types.h>
#include <ns/types.h>

struct yai_fs_namespace_binding {
    yai_inode_id_t inode_id;
    yai_ns_id_t ns_id;
};

#endif
