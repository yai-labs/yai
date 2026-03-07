/* SPDX-License-Identifier: Apache-2.0 */

#include "mind.h"
#include "mind_memory.h"
#include "mind_transport.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
  yai_mind_config_t cfg = {.runtime_name = "reinit-test", .enable_mock_provider = 1};
  yai_mind_episodic_record_t episodic = {0};
  yai_mind_node_id_t node = YAI_MIND_NODE_ID_INVALID;
  char out[256] = {0};

  assert(yai_mind_transport_handle_raw("PING\n", out, sizeof(out)) == YAI_MIND_ERR_STATE);

  assert(yai_mind_init(&cfg) == YAI_MIND_OK);
  assert(yai_mind_graph_node_create("episodic", "ep1", "first", &node) == YAI_MIND_OK);
  assert(yai_mind_domain_episodic_append("ep-1", node, "first episode") == YAI_MIND_OK);
  assert(yai_mind_domain_episodic_latest(&episodic) == YAI_MIND_OK);
  assert(strcmp(episodic.episode_id, "ep-1") == 0);
  assert(yai_mind_shutdown() == YAI_MIND_OK);

  assert(yai_mind_init(&cfg) == YAI_MIND_OK);
  assert(yai_mind_domain_episodic_latest(&episodic) == YAI_MIND_ERR_NOT_FOUND);
  assert(yai_mind_shutdown() == YAI_MIND_OK);

  puts("test_lifecycle_reinit: ok");
  return 0;
}
