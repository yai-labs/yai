#ifndef YAI_ENGINE_RPC_ENVELOPE_H
#define YAI_ENGINE_RPC_ENVELOPE_H

#include "../../law/specs/protocol/transport.h" // La Bibbia dei 96 byte
#include <stdbool.h>

// Validatore universale per l'Engine
// Controlla: Magic, Checksum, e che il ws_id dell'envelope sia quello dell'Engine
bool yai_envelope_validate(const yai_rpc_envelope_t* env, const char* expected_ws_id);

// Helper per costruire una risposta veloce
void yai_envelope_prepare_ack(yai_rpc_envelope_t* out, const yai_rpc_envelope_t* request);

#endif