#pragma once

#ifndef INCLUDE_DMA_REQUEST_H
#define INCLUDE_DMA_REQUEST_H

#include <dma/types.h>

struct yai_dma_request {
    yai_dma_request_id_t request_id;
    yai_dma_channel_id_t channel_id;
    enum yai_dma_direction direction;
    unsigned long size;
};

#endif
