/* SPDX-License-Identifier: Apache-2.0 */

#include "mind_cognition.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
  yai_mind_cognition_request_t req = {0};
  yai_mind_agent_role_t role;
  float score;

  snprintf(req.user_input, sizeof(req.user_input), "%s", "validate this governance policy change");
  role = yai_mind_reasoning_select_role(&req);
  assert(role == YAI_MIND_AGENT_ROLE_VALIDATOR);

  score = yai_mind_reasoning_score(&req, role);
  assert(score > 0.40f);
  assert(score <= 1.0f);

  req.preferred_role = YAI_MIND_AGENT_ROLE_CODE;
  role = yai_mind_reasoning_select_role(&req);
  assert(role == YAI_MIND_AGENT_ROLE_CODE);

  puts("test_reasoning_scoring: ok");
  return 0;
}
