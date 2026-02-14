#ifndef YAI_PROVIDER_GATE_H
#define YAI_PROVIDER_GATE_H

#include "../../law/specs/protocol/transport.h"
#include <stdbool.h>

typedef struct {
    char id[32];
    char host[128];
    int port;
    char endpoint[128];
    char api_key[128];
} yai_provider_config_t;

// Inizializza il gate con i dati del provider (chiamata dal config_loader)
void yai_provider_gate_init(const yai_provider_config_t* config);

// Punto di ingresso principale
char* yai_provider_gate_dispatch(const yai_rpc_envelope_t* env, const char* json_payload);

#endif