#ifndef YAI_RPC_ROUTER_H
#define YAI_RPC_ROUTER_H

#include "../../law/specs/protocol/transport.h"

/**
 * Firma universale per gli handler del router.
 * Ora include ws_id per permettere agli handler di accedere al DB corretto.
 */
typedef char* (*yai_rpc_handler_t)(const char* ws_id, const yai_rpc_envelope_t* env, const char* payload);

/**
 * Punto di ingresso: riceve ws_id, envelope e payload, trova l'handler, esegue.
 */
char* yai_rpc_router_dispatch(const char* ws_id, const yai_rpc_envelope_t* env, const char* payload);

// Handler specifico per l'Inference
char* handle_provider_inference(const char* ws_id, const yai_rpc_envelope_t* env, const char* payload);

#endif