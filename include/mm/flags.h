#pragma once

#ifndef INCLUDE_MM_FLAGS_H
#define INCLUDE_MM_FLAGS_H

enum yai_mm_map_flags {
    YAI_MM_MAP_READ      = 1u << 0,
    YAI_MM_MAP_WRITE     = 1u << 1,
    YAI_MM_MAP_EXEC      = 1u << 2,
    YAI_MM_MAP_SHARED    = 1u << 3,
    YAI_MM_MAP_ANON      = 1u << 4,
    YAI_MM_MAP_LOCKED    = 1u << 5
};

#endif
