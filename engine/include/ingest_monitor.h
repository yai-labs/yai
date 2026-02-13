#ifndef YAI_INGEST_MONITOR_H
#define YAI_INGEST_MONITOR_H

#include <stddef.h>
#include <stdint.h>

typedef struct {
    size_t current_buffer_usage;
    size_t peak_buffer_usage;
    uint32_t active_ingest_jobs;
} IngestMetrics;

int yai_ingest_track_start(size_t expected_size);
void yai_ingest_track_end(size_t actual_size);
IngestMetrics* yai_ingest_get_metrics(void);

#endif
