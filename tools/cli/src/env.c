// tools/cli/src/env.c
#include "../include/yai_env.h"
#include <stdlib.h>

const char *yai_env_get(const char *key, const char *fallback) {
    if (!key || key[0] == '\0') return fallback;
    const char *v = getenv(key);
    return (v && v[0] != '\0') ? v : fallback;
}

const char *yai_env_home(void) {
    // canonical: prefer HOME; fallback to getenv("USERPROFILE") for portability; else NULL
    const char *h = getenv("HOME");
    if (h && h[0] != '\0') return h;

    // optional fallback (harmless on mac/linux, useful on windows-like envs)
    h = getenv("USERPROFILE");
    if (h && h[0] != '\0') return h;

    return NULL;
}
