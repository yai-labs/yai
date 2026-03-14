#pragma once

#ifndef INCLUDE_SECURITY_CONTEXT_H
#define INCLUDE_SECURITY_CONTEXT_H

#include <stdint.h>

typedef uint64_t yai_security_context_id_t;

struct yai_security_context {
    yai_security_context_id_t id;
    const char *label;
};

#endif
