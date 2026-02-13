// tools/cli/include/yai_env.h
#ifndef YAI_ENV_H
#define YAI_ENV_H

// returns getenv(key) if non-empty else fallback
const char *yai_env_get(const char *key, const char *fallback);

// HOME resolver (returns NULL if missing)
const char *yai_env_home(void);

#endif
