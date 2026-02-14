#ifndef YAI_TRANSPORT_CLIENT_H
#define YAI_TRANSPORT_CLIENT_H
#include <stdbool.h> // <--- AGGIUNGI QUESTO PER RISOLVERE L'ERRORE
#include <stddef.h>
#include <stdint.h>
#include "../../law/specs/protocol/transport.h" // La Bibbia dei 96 byte

#ifdef __cplusplus
extern "C" {
#endif

#define YAI_RPC_LINE_MAX 8192 // Raddoppiato per i JSON pesanti degli LLM

typedef struct {
    int fd;
    char ws_id[36];      // Allineato alla LAW
    uint32_t session_id; // Ottenuto dopo l'handshake
    bool connected;
} yai_rpc_client_t;

/**
 * Connette al socket del Kernel per un workspace specifico.
 * Path tipico: ~/.yai/run/<ws_id>/control.sock
 */
int yai_rpc_connect(yai_rpc_client_t *c, const char *ws_id);

/**
 * Chiude la connessione e resetta lo stato del client.
 */
void yai_rpc_close(yai_rpc_client_t *c);

/**
 * Handshake V2: Invia l'envelope di handshake e valida la risposta.
 * Sincronizza le versioni tra Mind/Engine e Kernel.
 */
int yai_rpc_handshake(yai_rpc_client_t *c, uint32_t capabilities);

/**
 * La chiamata RPC sovrana.
 * Prende un envelope pre-compilato (con trace_id, command_id, etc)
 * e lo invia insieme al payload JSON.
 */
int yai_rpc_call(
    yai_rpc_client_t *c,
    const yai_rpc_envelope_t *env, 
    const char *json_payload,      
    char *out_response,           
    size_t out_cap
);

/**
 * Helper: genera un trace_id deterministico di 36 byte.
 */
void yai_make_trace_id(char out[36]);

#ifdef __cplusplus
}
#endif

#endif