/* SPDX-License-Identifier: Apache-2.0 */

#include "mind.h"
#include "mind_transport.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
  yai_mind_config_t cfg = {.runtime_name = "test", .enable_mock_provider = 1};
  char out[1024] = {0};

  assert(yai_mind_init(&cfg) == YAI_MIND_OK);
  assert(yai_mind_transport_handle_raw("COGNITION validate this architecture\n", out, sizeof(out)) == YAI_MIND_OK);
  assert(strstr(out, "STATUS 200") != NULL);
  assert(strstr(out, "role=") != NULL);

  assert(yai_mind_shutdown() == YAI_MIND_OK);
  puts("test_mind_flow: ok");
  return 0;
}
