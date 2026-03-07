#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <yai/api/runtime.h>
#include <yai/api/version.h>

#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

static const char *resolve_core_binary(const char *argv0) {
  static char candidate[PATH_MAX];
  const char *slash = NULL;

  if (!argv0 || !argv0[0]) return YAI_BIN_CORE;
  slash = strrchr(argv0, '/');
  if (!slash) return YAI_BIN_CORE;
  if ((size_t)(slash - argv0) >= sizeof(candidate) - 16) return YAI_BIN_CORE;

  snprintf(candidate, sizeof(candidate), "%.*s/%s",
           (int)(slash - argv0), argv0, YAI_BIN_CORE);
  if (access(candidate, X_OK) == 0) return candidate;
  return YAI_BIN_CORE;
}

static int run_core(int argc, char **argv, int start_idx, const char *core_bin) {
  char *core_argv[64];
  int i = 0;

  core_argv[i++] = (char *)core_bin;
  for (int src = start_idx; src < argc && i < 63; ++src) {
    core_argv[i++] = argv[src];
  }
  core_argv[i] = NULL;

  execvp(core_argv[0], core_argv);
  fprintf(stderr, "yai: failed to execute %s: %s\n", YAI_BIN_CORE, strerror(errno));
  return 127;
}

static void print_help(void) {
  puts("yai - operator entrypoint");
  printf("version: %s\n", YAI_VERSION_STRING);
  puts("");
  puts("usage:");
  puts("  yai up               # start unified runtime baseline");
  puts("  yai status           # runtime composition probes");
  puts("  yai brain-check      # cognition/runtime smoke");
  puts("  yai core [args...]   # direct yai-core pass-through");
  puts("  yai --help           # show this help");
}

int main(int argc, char **argv) {
  const char *core_bin = resolve_core_binary((argc > 0) ? argv[0] : NULL);

  if (argc <= 1 || strcmp(argv[1], "--help") == 0 || strcmp(argv[1], "-h") == 0) {
    print_help();
    return 0;
  }

  if (strcmp(argv[1], "up") == 0) {
    char *up_argv[] = {(char *)core_bin, (char *)"--run", NULL};
    execvp(up_argv[0], up_argv);
    fprintf(stderr, "yai: failed to execute %s --run: %s\n", core_bin, strerror(errno));
    return 127;
  }
  if (strcmp(argv[1], "status") == 0) {
    char *status_argv[] = {(char *)core_bin, (char *)"--status", NULL};
    execvp(status_argv[0], status_argv);
    fprintf(stderr, "yai: failed to execute %s --status: %s\n", core_bin, strerror(errno));
    return 127;
  }
  if (strcmp(argv[1], "brain-check") == 0) {
    char *brain_argv[] = {(char *)core_bin, (char *)"--brain-check", NULL};
    execvp(brain_argv[0], brain_argv);
    fprintf(stderr, "yai: failed to execute %s --brain-check: %s\n", core_bin, strerror(errno));
    return 127;
  }
  if (strcmp(argv[1], "core") == 0) {
    return run_core(argc, argv, 2, core_bin);
  }

  /* Default multiplexer behavior: pass unknown subcommands to yai-core. */
  return run_core(argc, argv, 1, core_bin);
}
