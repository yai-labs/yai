#ifndef YAI_TRANSPORT_CLIENT_H
#define YAI_TRANSPORT_CLIENT_H

#include "transport.h"

int yai_transport_init(void);
int yai_send_command(CmdType type, const void* data, size_t len);

#endif
