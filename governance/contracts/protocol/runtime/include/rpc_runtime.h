/**
 * @file rpc_runtime.h
 * @brief Runtime RPC helpers for envelope validation and response preparation.
 *
 * Shared protocol-runtime helper surface for core/exec/brain modules.
 */
#pragma once

#include <stdbool.h>
#include <transport.h>
#include <protocol.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Validate an incoming envelope against the expected workspace ID. */
bool yai_envelope_validate(
    const yai_rpc_envelope_t* env,
    const char* expected_ws_id);

/** Prepare a response envelope derived from an incoming request. */
void yai_envelope_prepare_response(
    yai_rpc_envelope_t* out,
    const yai_rpc_envelope_t* request,
    uint32_t command_id,
    uint32_t payload_len);

#ifdef __cplusplus
}
#endif
