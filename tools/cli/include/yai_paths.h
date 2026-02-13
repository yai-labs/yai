// tools/cli/include/yai_paths.h
#ifndef YAI_PATHS_H
#define YAI_PATHS_H

#include <stddef.h>

// ~/.yai/run/<ws_id>/control.sock
int yai_path_control_sock(const char *ws_id, char *out, size_t cap);

#endif
