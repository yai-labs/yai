/* SPDX-License-Identifier: Apache-2.0 */

#include <yai/brain/transport.h>

static int g_transport_initialized = 0;

int yai_mind_transport_init(void)
{
  if (g_transport_initialized) return YAI_MIND_OK;
  g_transport_initialized = 1;
  return YAI_MIND_OK;
}

int yai_mind_transport_shutdown(void)
{
  if (!g_transport_initialized) return YAI_MIND_OK;
  g_transport_initialized = 0;
  return YAI_MIND_OK;
}

int yai_mind_transport_is_initialized(void)
{
  return g_transport_initialized;
}
