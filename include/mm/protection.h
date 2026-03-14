#pragma once

#ifndef INCLUDE_MM_PROTECTION_H
#define INCLUDE_MM_PROTECTION_H

enum yai_mm_protection {
    YAI_MM_PROT_NONE  = 0,
    YAI_MM_PROT_READ  = 1u << 0,
    YAI_MM_PROT_WRITE = 1u << 1,
    YAI_MM_PROT_EXEC  = 1u << 2
};

#endif
