// tools/cli/src/env.c
#include "../include/yai_env.h"
#include <stdlib.h>

const char *yai_env_get(const char *key, const char *fallback) {
    const char *v = getenv(key);
    if (v && v[0] != '\0') return v;
    return fallback;
}

const char *yai_env_home(void) {
    const char *h = getenv("HOME");
    return (h && h[0] != '\0') ? h : NULL;
}
