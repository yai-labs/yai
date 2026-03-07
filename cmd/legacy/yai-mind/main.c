/* SPDX-License-Identifier: Apache-2.0 */

#include <yai/brain/brain.h>
#include <yai/brain/memory.h>
#include <yai/brain/transport.h>

#include <stdio.h>
#include <string.h>

int main(int argc, char **argv)
{
  fprintf(stderr, "[DEPRECATED ENTRYPOINT] yai-mind is legacy. Use 'yai-core --run' or 'yai-core --brain-check'.\n");
  yai_mind_config_t config = {
    .runtime_name = "yai-mind",
    .enable_mock_provider = 1,
  };
  int rc;
  int check_only = 0;
  int serve_once = 0;
  const char *socket_path = NULL;

  for (int i = 1; i < argc; i++) {
    if (!argv || !argv[i]) continue;
    if (strcmp(argv[i], "--check") == 0) {
      check_only = 1;
      continue;
    }
    if (strcmp(argv[i], "--serve-once") == 0) {
      serve_once = 1;
      continue;
    }
    if (strcmp(argv[i], "--socket") == 0 && i + 1 < argc && argv[i + 1]) {
      socket_path = argv[++i];
      continue;
    }
  }

  rc = yai_mind_init(&config);
  if (rc != YAI_MIND_OK) {
    fprintf(stderr, "mind init failed: %d\n", rc);
    return 1;
  }

  {
    yai_mind_node_id_t seed = YAI_MIND_NODE_ID_INVALID;
    (void)yai_mind_graph_node_create("semantic", "mind:bootstrap", "mind daemon ready", &seed);
  }

  if (serve_once) {
    rc = yai_mind_uds_server_run_once(socket_path);
    if (rc != YAI_MIND_OK) {
      fprintf(stderr, "mind uds server failed: %d\n", rc);
      (void)yai_mind_shutdown();
      return 1;
    }
  }

  if (!check_only) {
    if (serve_once) {
      puts("yai-mind served one UDS request.");
    } else {
      puts("yai-mind bootstrap initialized.");
    }
  }

  rc = yai_mind_shutdown();
  if (rc != YAI_MIND_OK) {
    fprintf(stderr, "mind shutdown failed: %d\n", rc);
    return 1;
  }

  return 0;
}
