#include <string.h>
#include "../include/agent_contract.h"
#include "../include/engine_bridge.h"

int validate_agent_action(const char* agent_id, uint32_t estimated_cost) {
    (void)agent_id; // Sopprime il warning unused-parameter
    Vault* v = yai_get_vault();
    
    if (v->energy_consumed + estimated_cost > v->energy_quota) {
        strncpy(v->last_error, "Insufficient energy for agent action", 255);
        return -1;
    }
    return 0;
}