#pragma once

#ifndef INCLUDE_IPC_FRAME_H
#define INCLUDE_IPC_FRAME_H

#include <stdint.h>

struct yai_ipc_frame {
    uint32_t opcode;
    uint32_t length;
};

#endif
