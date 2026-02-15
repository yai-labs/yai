#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// Root Plane (L1) hardened UDS server.
// The frame/envelope is defined by Law.
#include "transport.h"
#include <stdint.h>

// Initialize root-plane UDS listener at the given path.
// If path is NULL/empty, implementation may fall back to getenv("YAI_RUNTIME_SOCKET")
// or DEFAULT_RUNTIME_SOCKET_PATH.
int  yai_transport_init_at(const char *path);

// Serve exactly one client connection (accept -> read envelope+payload -> dispatch -> close).
void yai_transport_serve_once(void);

#ifdef __cplusplus
}
#endif
