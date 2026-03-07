/* SPDX-License-Identifier: Apache-2.0 */

#include "mind.h"
#include "mind_cognition.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

static void run_role(yai_mind_agent_role_t role)
{
  yai_mind_cognition_request_t req = {0};
  yai_mind_cognition_response_t resp = {0};
  int rc;

  snprintf(req.session.session_id, sizeof(req.session.session_id), "%s", "agents-test");
  snprintf(req.session.workspace_id, sizeof(req.session.workspace_id), "%s", "ws_agents");
  snprintf(req.task.task_id, sizeof(req.task.task_id), "%s", "task-agents");
  snprintf(req.task.prompt, sizeof(req.task.prompt), "%s", "agent execution check");
  snprintf(req.user_input, sizeof(req.user_input), "%s", "validate code and history with knowledge");
  req.preferred_role = role;

  rc = yai_mind_cognition_execute(&req, &resp);
  assert(rc == YAI_MIND_OK);
  assert(resp.selected_role == role);
  assert(resp.output[0] != 0);
}

int main(void)
{
  yai_mind_config_t cfg = {.runtime_name = "test", .enable_mock_provider = 1};

  assert(yai_mind_init(&cfg) == YAI_MIND_OK);
  run_role(YAI_MIND_AGENT_ROLE_SYSTEM);
  run_role(YAI_MIND_AGENT_ROLE_CODE);
  run_role(YAI_MIND_AGENT_ROLE_KNOWLEDGE);
  run_role(YAI_MIND_AGENT_ROLE_HISTORIAN);
  run_role(YAI_MIND_AGENT_ROLE_VALIDATOR);
  assert(yai_mind_shutdown() == YAI_MIND_OK);

  puts("test_cognition_agents: ok");
  return 0;
}
