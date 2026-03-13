#pragma once

#include <yai/ipc/rpc.h>
#include <yai/orch/transport.h>

typedef char *(*yai_rpc_handler_t)(const char *ws_id, const yai_rpc_envelope_t *env, const char *payload);
char *yai_rpc_router_dispatch(const char *ws_id, const yai_rpc_envelope_t *env, const char *payload);
