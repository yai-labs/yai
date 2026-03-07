/* SPDX-License-Identifier: Apache-2.0 */

#include "mind_memory.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
  yai_mind_node_id_t n1 = YAI_MIND_NODE_ID_INVALID;
  yai_mind_node_id_t n2 = YAI_MIND_NODE_ID_INVALID;
  yai_mind_edge_id_t e1 = YAI_MIND_EDGE_ID_INVALID;
  yai_mind_graph_node_t node = {0};
  yai_mind_graph_edge_t edge = {0};
  yai_mind_memory_query_t q = {0};
  yai_mind_memory_result_t r = {0};
  yai_mind_graph_stats_t s = {0};

  assert(yai_mind_memory_init() == YAI_MIND_OK);
  assert(strcmp(yai_mind_graph_backend_name(), "inmemory") == 0);

  assert(yai_mind_graph_node_create("semantic", "term:kernel", "kernel runtime layer", &n1) == YAI_MIND_OK);
  assert(yai_mind_graph_node_create("episodic", "event:start", "runtime started", &n2) == YAI_MIND_OK);
  assert(yai_mind_graph_edge_create(n1, n2, "references", 0.8f, &e1) == YAI_MIND_OK);

  assert(yai_mind_graph_node_get(n1, &node) == YAI_MIND_OK);
  assert(strcmp(node.domain, "semantic") == 0);
  assert(yai_mind_graph_edge_get(e1, &edge) == YAI_MIND_OK);
  assert(edge.from_node == n1 && edge.to_node == n2);

  snprintf(q.query, sizeof(q.query), "kernel");
  q.limit = 10;
  assert(yai_mind_memory_query_run(&q, &r) == YAI_MIND_OK);
  assert(r.match_count >= 1);

  assert(yai_mind_graph_stats_get(&s) == YAI_MIND_OK);
  assert(s.node_count >= 2);
  assert(s.edge_count >= 1);

  assert(yai_mind_memory_shutdown() == YAI_MIND_OK);
  puts("test_memory_graph: ok");
  return 0;
}
