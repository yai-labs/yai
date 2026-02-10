#ifndef YAI_PROTOCOL_H
#define YAI_PROTOCOL_H

#include <stdint.h>
#include <stddef.h>

#define YAI_PROTOCOL_MAGIC   0x59414950u /* 'Y''A''I''P' */
#define YAI_PROTOCOL_VERSION 1u

#include "protocol/commands.h"
#include "protocol/transport.h"
#include "protocol/auth.h"
#include "protocol/audit.h"

#endif /* YAI_PROTOCOL_H */
