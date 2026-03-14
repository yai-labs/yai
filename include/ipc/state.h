#pragma once

#ifndef INCLUDE_IPC_STATE_H
#define INCLUDE_IPC_STATE_H

enum yai_ipc_state {
    YAI_IPC_DOWN = 0,
    YAI_IPC_NEGOTIATING,
    YAI_IPC_READY,
    YAI_IPC_CLOSED,
    YAI_IPC_FAILED
};

#endif
