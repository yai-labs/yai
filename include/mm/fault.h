#pragma once

#ifndef INCLUDE_MM_FAULT_H
#define INCLUDE_MM_FAULT_H

enum yai_mm_fault_reason {
    YAI_MM_FAULT_NONE = 0,
    YAI_MM_FAULT_NOT_PRESENT,
    YAI_MM_FAULT_PROTECTION,
    YAI_MM_FAULT_BAD_ADDRESS,
    YAI_MM_FAULT_POLICY_DENIED
};

struct yai_mm_fault {
    unsigned long address;
    enum yai_mm_fault_reason reason;
};

#endif
