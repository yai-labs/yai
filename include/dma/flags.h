#pragma once

#ifndef INCLUDE_DMA_FLAGS_H
#define INCLUDE_DMA_FLAGS_H

enum yai_dma_flags {
    YAI_DMA_FENCE_REQUIRED = 1u << 0,
    YAI_DMA_ZERO_COPY      = 1u << 1,
    YAI_DMA_COHERENT       = 1u << 2
};

#endif
