/* SPDX-License-Identifier: Apache-2.0 */

#include "graph_backend.h"

#include <stdio.h>
#include <string.h>

static char g_rpc_endpoint[256];

int yai_mind_graph_backend_select_rpc(const char *endpoint)
{
  if (!endpoint || !endpoint[0]) return YAI_MIND_ERR_INVALID_ARG;
  snprintf(g_rpc_endpoint, sizeof(g_rpc_endpoint), "%s", endpoint);
  return YAI_MIND_ERR_NOT_IMPLEMENTED;
}
