/* SPDX-License-Identifier: Apache-2.0 */

#include <yai/cognition/memory.h>

int yai_knowledge_memory_start(void)
{
  return 0;
}

int yai_knowledge_memory_stop(void)
{
  return 0;
}

int yai_domain_authority_grant(yai_node_id_t node_id,
                               const char *policy,
                               int level)
{
  (void)node_id;
  (void)policy;
  (void)level;
  return 0;
}

int yai_domain_episodic_append(const char *episode_id,
                               yai_node_id_t node_id,
                               const char *summary)
{
  (void)episode_id;
  (void)node_id;
  (void)summary;
  return 0;
}
