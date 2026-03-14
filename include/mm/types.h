#pragma once

#ifndef INCLUDE_MM_TYPES_H
#define INCLUDE_MM_TYPES_H

#include <stdint.h>

typedef uint64_t yai_phys_addr_t;
typedef uint64_t yai_virt_addr_t;
typedef uint64_t yai_page_id_t;

enum yai_mm_region_kind {
    YAI_MM_REGION_UNKNOWN = 0,
    YAI_MM_REGION_HEAP,
    YAI_MM_REGION_STACK,
    YAI_MM_REGION_FILE,
    YAI_MM_REGION_DEVICE,
    YAI_MM_REGION_SHARED
};

#endif
