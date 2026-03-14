#pragma once

#ifndef INCLUDE_NS_BINDING_H
#define INCLUDE_NS_BINDING_H

#include <ns/types.h>

struct yai_ns_binding {
    yai_ns_id_t parent_ns_id;
    yai_ns_id_t child_ns_id;
    const char *binding_kind;
};

#endif
