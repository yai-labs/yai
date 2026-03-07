#include <errno.h>
#include <stdio.h>
#include <string.h>

#include <yai/api/runtime.h>
#include <yai/brain/brain.h>
#include <yai/brain/cognition.h>
#include <yai/core/lifecycle.h>
#include <yai/exec/runtime.h>

static void print_help(void) {
  puts("yai-core - unified runtime entrypoint");
  puts("");
  puts("usage:");
  puts("  yai-core --help");
  puts("  yai-core --run");
  puts("  yai-core --status");
  puts("  yai-core --brain-check");
  puts("  yai-core --preflight");
  puts("  yai-core --exec-probe");
  puts("");
  puts("runtime composition order:");
  puts("  core(preflight/layout) -> exec(probe) -> brain(init/check) -> shutdown");
}

static int run_preflight(void) {
  int rc = yai_run_preboot_checks();
  if (rc != 0) {
    fprintf(stderr, "yai-core: preboot checks failed (rc=%d)\n", rc);
    return 1;
  }
  rc = yai_ensure_runtime_layout("system");
  if (rc != 0) {
    fprintf(stderr, "yai-core: runtime layout failed (rc=%d)\n", rc);
    return 1;
  }
  puts("yai-core: preflight OK");
  return 0;
}

static int run_exec_probe(void) {
  int state = yai_exec_runtime_probe();
  printf("yai-core: exec runtime state=%s (%d)\n", yai_exec_runtime_state_name((yai_exec_runtime_state_t)state), state);
  return (state >= 0) ? 0 : 1;
}

static int run_brain_check(void) {
  yai_mind_config_t cfg = {.runtime_name = "yai-core", .enable_mock_provider = 1};
  yai_mind_cognition_response_t out = {0};
  int rc = yai_mind_init(&cfg);
  if (rc != YAI_MIND_OK) {
    fprintf(stderr, "yai-core: brain init failed (%d)\n", rc);
    return 1;
  }
  rc = yai_mind_cognition_execute_text("brain check", "yai-core-check", "mock", &out);
  if (rc != YAI_MIND_OK) {
    fprintf(stderr, "yai-core: brain cognition probe failed (%d)\n", rc);
    (void)yai_mind_shutdown();
    return 1;
  }
  printf("yai-core: brain check OK role=%s score=%.2f\n", yai_mind_agent_role_name(out.selected_role), out.score);
  return (yai_mind_shutdown() == YAI_MIND_OK) ? 0 : 1;
}

static int run_status(void) {
  int preboot = yai_run_preboot_checks();
  int layout = yai_ensure_runtime_layout("system");
  int exec_state = yai_exec_runtime_probe();
  printf("yai-core: status core.preboot=%d core.layout=%d exec.state=%s(%d)\n",
         preboot,
         layout,
         yai_exec_runtime_state_name((yai_exec_runtime_state_t)exec_state),
         exec_state);
  return (preboot == 0 && layout == 0 && exec_state >= 0) ? 0 : 1;
}

static int run_runtime(void) {
  yai_mind_config_t cfg = {.runtime_name = "yai-core-runtime", .enable_mock_provider = 1};
  int rc;

  puts("yai-core: init core/preflight...");
  rc = run_preflight();
  if (rc != 0) return rc;

  puts("yai-core: attach exec plane...");
  rc = run_exec_probe();
  if (rc != 0) return rc;

  puts("yai-core: attach brain plane...");
  rc = yai_mind_init(&cfg);
  if (rc != YAI_MIND_OK) {
    fprintf(stderr, "yai-core: brain init failed (%d)\n", rc);
    return 1;
  }

  puts("yai-core: runtime composition ready (core+exec+brain)");

  puts("yai-core: teardown brain plane...");
  if (yai_mind_shutdown() != YAI_MIND_OK) return 1;

  puts("yai-core: runtime shutdown complete");
  return 0;
}

int main(int argc, char **argv) {
  if (argc <= 1) {
    return run_runtime();
  }

  if (strcmp(argv[1], "--help") == 0 || strcmp(argv[1], "-h") == 0) {
    print_help();
    return 0;
  }

  if (strcmp(argv[1], "--run") == 0) {
    return run_runtime();
  }
  if (strcmp(argv[1], "--status") == 0) {
    return run_status();
  }
  if (strcmp(argv[1], "--brain-check") == 0) {
    return run_brain_check();
  }
  if (strcmp(argv[1], "--preflight") == 0) {
    return run_preflight();
  }
  if (strcmp(argv[1], "--exec-probe") == 0) {
    return run_exec_probe();
  }

  if (strncmp(argv[1], "--legacy-", 9) == 0) {
    fprintf(stderr, "yai-core: %s is no longer available after legacy removal\n", argv[1]);
    return 3;
  }

  fprintf(stderr, "yai-core: unknown mode '%s'\n", argv[1]);
  print_help();
  return 2;
}
