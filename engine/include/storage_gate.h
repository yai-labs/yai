#ifndef STORAGE_GATE_H
#define STORAGE_GATE_H

/**
 * Inizializza il registro globale delle connessioni DB.
 * Chiamata una volta al boot dell'engine.
 */
void yai_storage_init(void);

/**
 * Chiude tutte le connessioni SQLite attive.
 */
void yai_storage_shutdown(void);

/**
 * Gestisce le richieste RPC per il grafo semantico.
 * @param ws_id Il workspace di destinazione (multi-tenancy)
 * @param method Il comando (put_node, get_node, etc)
 * @param params_json Parametri in formato JSON string
 */
char* yai_storage_handle_rpc(const char* ws_id, const char* method, const char* params_json);

#endif