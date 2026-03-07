/* SPDX-License-Identifier: Apache-2.0 */

#include "mind.h"
#include "mind_memory.h"
#include "mind_transport.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
  yai_mind_config_t cfg = {.runtime_name = "test", .enable_mock_provider = 1};
  char out[1024] = {0};
  yai_mind_node_id_t node = YAI_MIND_NODE_ID_INVALID;

  assert(yai_mind_init(&cfg) == YAI_MIND_OK);
  assert(yai_mind_graph_node_create("semantic", "transport:test", "transport seeded", &node) == YAI_MIND_OK);

  assert(yai_mind_transport_handle_raw("PING\n", out, sizeof(out)) == YAI_MIND_OK);
  assert(strstr(out, "STATUS 200") != NULL);

  memset(out, 0, sizeof(out));
  assert(yai_mind_transport_handle_raw("COMPLETE hello\n", out, sizeof(out)) == YAI_MIND_OK);
  assert(strstr(out, "mock completion") != NULL);

  memset(out, 0, sizeof(out));
  assert(yai_mind_transport_handle_raw("QUERY transport\n", out, sizeof(out)) == YAI_MIND_OK);
  assert(strstr(out, "matches=") != NULL);

  assert(yai_mind_shutdown() == YAI_MIND_OK);
  puts("test_transport: ok");
  return 0;
}
