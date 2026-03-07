#pragma once

#include <yai/protocol/rpc_runtime.h>

int yai_rpc_write_error_v1(int fd,
                           const char *ws_id,
                           const char *trace,
                           const char *code,
                           const char *msg,
                           const char *actor);
