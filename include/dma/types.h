#pragma once

#ifndef INCLUDE_DMA_TYPES_H
#define INCLUDE_DMA_TYPES_H

#include <stdint.h>

typedef uint64_t yai_dma_channel_id_t;
typedef uint64_t yai_dma_request_id_t;

enum yai_dma_direction {
    YAI_DMA_NONE = 0,
    YAI_DMA_TO_DEVICE,
    YAI_DMA_FROM_DEVICE,
    YAI_DMA_BIDIRECTIONAL
};

#endif
