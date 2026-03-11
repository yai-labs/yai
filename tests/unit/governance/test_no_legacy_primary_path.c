#include <stdio.h>
#include <string.h>

#include <yai/law/loader.h>

int main(void) {
  yai_law_runtime_t rt;
  char err[256] = {0};

  if (yai_law_load_runtime(&rt, err, sizeof(err)) != 0) {
    fprintf(stderr, "no_legacy_primary_path: runtime load failed: %s\n", err);
    return 1;
  }

  if (strcmp(rt.root, "deps/law") == 0) {
    fprintf(stderr, "no_legacy_primary_path: deps/law selected as primary path\n");
    return 1;
  }

  if (strcmp(rt.root, "embedded/law") != 0) {
    fprintf(stderr, "no_legacy_primary_path: unexpected primary root: %s\n", rt.root);
    return 1;
  }

  puts("no_legacy_primary_path: ok");
  return 0;
}
