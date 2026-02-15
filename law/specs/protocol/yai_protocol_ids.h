#ifndef YAI_PROTOCOL_IDS_H
#define YAI_PROTOCOL_IDS_H

#define YAI_PROTOCOL_IDS_VERSION 1

// Classi di Comando (Bitmask per permessi rapidi)
#define YAI_CMD_CLASS_CONTROL     0x0100u // Operazioni Kernel/Root plane
#define YAI_CMD_CLASS_STORAGE     0x0200u // Memoria (L2)
#define YAI_CMD_CLASS_PROVIDER    0x0300u // LLM/Inference
#define YAI_CMD_CLASS_PRIVILEGED  0xF000u // Richiede arming=true

typedef enum {
    // Controllo (0x01xx)
    YAI_CMD_PING        = 0x0101u,
    YAI_CMD_HANDSHAKE   = 0x0102u,

    // CONTROL PLANE CANONICO (L1): status/ws.create/ws.list/ws.destroy/etc
    // Nota: non esisteva -> la CLI ora lo usa come request_type="control"
    YAI_CMD_CONTROL     = 0x0104u,

    // Privileged
    YAI_CMD_RECONFIGURE = (0x0103u | YAI_CMD_CLASS_PRIVILEGED),

    // Storage (0x02xx)
    YAI_CMD_STORAGE_RPC = 0x0201u,

    // Providers (0x03xx)
    YAI_CMD_PROVIDER_RPC  = 0x0301u,
    YAI_CMD_EMBEDDING_RPC = 0x0302u
} yai_command_id_t;

#endif
