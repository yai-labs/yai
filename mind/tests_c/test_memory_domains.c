/* SPDX-License-Identifier: Apache-2.0 */

#include "mind_memory.h"

#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
  yai_mind_node_id_t node = YAI_MIND_NODE_ID_INVALID;
  yai_mind_activation_record_t ar = {0};
  yai_mind_activation_trace_t at = {0};
  yai_mind_authority_record_t auth = {0};
  yai_mind_episodic_record_t ep = {0};
  yai_mind_semantic_record_t sem = {0};
  yai_mind_node_id_t nearest = YAI_MIND_NODE_ID_INVALID;
  float distance = -1.0f;
  float v1[4] = {0.1f, 0.2f, 0.3f, 0.4f};
  float v2[4] = {0.2f, 0.1f, 0.4f, 0.3f};
  float qv[4] = {0.11f, 0.19f, 0.29f, 0.41f};

  assert(yai_mind_memory_init() == YAI_MIND_OK);
  assert(yai_mind_graph_node_create("semantic", "term:authority", "authority definition", &node) == YAI_MIND_OK);

  assert(yai_mind_domain_activation_record(node, 0.92f, "phase3") == YAI_MIND_OK);
  assert(yai_mind_domain_activation_last(&ar, &at) == YAI_MIND_OK);
  assert(ar.node_id == node);
  assert(ar.score > 0.9f);
  assert(at.tick > 0);

  assert(yai_mind_domain_authority_grant(node, "policy.read", 3) == YAI_MIND_OK);
  assert(yai_mind_domain_authority_get(node, &auth) == YAI_MIND_OK);
  assert(strcmp(auth.policy, "policy.read") == 0);

  assert(yai_mind_domain_episodic_append("ep-1", node, "workspace initialized") == YAI_MIND_OK);
  assert(yai_mind_domain_episodic_latest(&ep) == YAI_MIND_OK);
  assert(strcmp(ep.episode_id, "ep-1") == 0);

  assert(yai_mind_domain_semantic_put("authority", "governance constraint", node) == YAI_MIND_OK);
  assert(yai_mind_domain_semantic_get("authority", &sem) == YAI_MIND_OK);
  assert(strcmp(sem.term, "authority") == 0);

  assert(yai_mind_domain_vector_upsert(node, v1, 4) == YAI_MIND_OK);
  assert(yai_mind_domain_vector_upsert(node + 1U, v2, 4) == YAI_MIND_OK);
  assert(yai_mind_domain_vector_nearest(qv, 4, &nearest, &distance) == YAI_MIND_OK);
  assert(yai_mind_node_id_is_valid(nearest));
  assert(distance >= 0.0f);

  assert(yai_mind_memory_shutdown() == YAI_MIND_OK);
  puts("test_memory_domains: ok");
  return 0;
}
