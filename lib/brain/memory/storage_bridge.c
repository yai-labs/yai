/* SPDX-License-Identifier: Apache-2.0 */

#include <yai/brain/memory.h>

/* Baseline storage-bridge hook; durable backend integration is intentionally deferred. */
int yai_mind_storage_bridge_query(const yai_mind_memory_query_t *query,
                                  yai_mind_memory_result_t *result)
{
  if (!query || !result) return YAI_MIND_ERR_INVALID_ARG;

  result->match_count = 0;
  result->summary[0] = '\0';
  return YAI_MIND_OK;
}
