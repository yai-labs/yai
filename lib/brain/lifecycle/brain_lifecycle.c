/* SPDX-License-Identifier: Apache-2.0 */

#include <yai/brain/brain.h>
#include <yai/brain/cognition.h>
#include <yai/brain/memory.h>
#include <yai/brain/providers.h>
#include <yai/brain/transport.h>

#include <string.h>

static yai_mind_runtime_t g_runtime = {0};

static void clear_runtime(void)
{
  memset(&g_runtime, 0, sizeof(g_runtime));
}

int yai_mind_init(const yai_mind_config_t *config)
{
  int rc;

  if (g_runtime.initialized) return YAI_MIND_OK;

  clear_runtime();
  if (config) g_runtime.config = *config;

  rc = yai_mind_transport_init();
  if (rc != YAI_MIND_OK) return rc;
  g_runtime.transport_ready = 1;

  rc = yai_mind_providers_init();
  if (rc != YAI_MIND_OK) {
    (void)yai_mind_transport_shutdown();
    clear_runtime();
    return rc;
  }
  g_runtime.providers_ready = 1;

  rc = yai_mind_memory_init();
  if (rc != YAI_MIND_OK) {
    (void)yai_mind_providers_shutdown();
    (void)yai_mind_transport_shutdown();
    clear_runtime();
    return rc;
  }
  g_runtime.memory_ready = 1;

  rc = yai_mind_cognition_init();
  if (rc != YAI_MIND_OK) {
    (void)yai_mind_memory_shutdown();
    (void)yai_mind_providers_shutdown();
    (void)yai_mind_transport_shutdown();
    clear_runtime();
    return rc;
  }
  g_runtime.cognition_ready = 1;

  g_runtime.initialized = 1;
  return YAI_MIND_OK;
}

int yai_mind_shutdown(void)
{
  if (!g_runtime.initialized) return YAI_MIND_OK;

  (void)yai_mind_cognition_shutdown();
  g_runtime.cognition_ready = 0;

  (void)yai_mind_memory_shutdown();
  g_runtime.memory_ready = 0;

  (void)yai_mind_providers_shutdown();
  g_runtime.providers_ready = 0;

  (void)yai_mind_transport_shutdown();
  g_runtime.transport_ready = 0;

  g_runtime.initialized = 0;
  memset(&g_runtime.config, 0, sizeof(g_runtime.config));
  return YAI_MIND_OK;
}

int yai_mind_is_initialized(void)
{
  return g_runtime.initialized;
}

const yai_mind_runtime_t *yai_mind_runtime_state(void)
{
  return &g_runtime;
}
