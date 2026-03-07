/* SPDX-License-Identifier: Apache-2.0 */

#include "mind.h"
#include "mind_cognition.h"
#include "mind_memory.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
  yai_mind_config_t cfg = {.runtime_name = "test", .enable_mock_provider = 1};
  yai_mind_node_id_t seed = YAI_MIND_NODE_ID_INVALID;
  yai_mind_cognition_response_t resp = {0};

  assert(yai_mind_init(&cfg) == YAI_MIND_OK);
  assert(yai_mind_graph_node_create("semantic", "pipeline", "rag baseline", &seed) == YAI_MIND_OK);

  assert(yai_mind_cognition_execute_text("what is pipeline context", "rag-session", "mock", &resp) == YAI_MIND_OK);
  assert(resp.status == 200);
  assert(resp.plan_steps >= 2);
  assert(resp.output[0] != 0);

  assert(yai_mind_shutdown() == YAI_MIND_OK);
  puts("test_rag_pipeline: ok");
  return 0;
}
