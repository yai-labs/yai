/* SPDX-License-Identifier: Apache-2.0 */

#include "mind_providers.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
  yai_mind_provider_registry_t *registry;
  yai_mind_provider_response_t response = {0};
  float vec[4] = {0};

  assert(yai_mind_providers_init() == YAI_MIND_OK);
  registry = yai_mind_providers_registry();
  assert(registry != NULL);
  assert(yai_mind_provider_registry_default(registry) != NULL);
  assert(yai_mind_provider_registry_get(registry, "mock") != NULL);

  assert(yai_mind_client_completion("mock", "hello world", &response) == YAI_MIND_OK);
  assert(response.status == 200);
  assert(strstr(response.output, "mock completion") != NULL);

  assert(yai_mind_client_embedding("mock", "embed me", vec, 4) == YAI_MIND_OK);
  assert(vec[0] >= 0.0f);

  assert(yai_mind_providers_shutdown() == YAI_MIND_OK);
  puts("test_providers: ok");
  return 0;
}
