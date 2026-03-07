#pragma once

void yai_storage_init(void);
void yai_storage_shutdown(void);
char *yai_storage_handle_rpc(const char *ws_id, const char *method, const char *params_json);
