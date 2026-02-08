#include <stdint.h>

#define MAX_SESSIONS 16
#define MAX_PATH_LEN 256

typedef enum { WS_CREATED, WS_ACTIVE, WS_CLOSED, WS_ERROR } ice_ws_state_t;

typedef struct {
    uint32_t workspace_id;
    char name[32];
    char base_path[MAX_PATH_LEN];
    ice_ws_state_t state;
} ice_workspace_t;

typedef struct {
    uint32_t session_id;
    uint32_t run_id;
    ice_workspace_t workspace;
    uint32_t capability_mask; // Le tue stringhe "capabilities" diventano bit!
} ice_session_t;

// Il registro delle sessioni è un semplice array fisso (Sicurezza I-002)
static ice_session_t session_registry[MAX_SESSIONS];
