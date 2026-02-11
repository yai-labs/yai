pub const SHM_VAULT_PREFIX: &str = "/yai_vault_";
pub const MAX_WS_ID: usize = 64;
pub const MAX_TRACE_ID: usize = 64;
pub const MAX_ERR_MSG: usize = 256;

// yai_state_t (Kernel/include/yai_vault.h)
pub const YAI_STATE_HALT: u32 = 0;
pub const YAI_STATE_PREBOOT: u32 = 1;
pub const YAI_STATE_READY: u32 = 2;
pub const YAI_STATE_HANDOFF_COMPLETE: u32 = 3;
pub const YAI_STATE_RUNNING: u32 = 4;
pub const YAI_STATE_SUSPENDED: u32 = 5;
pub const YAI_STATE_ERROR: u32 = 6;

// yai_command_t
pub const YAI_CMD_NONE: u32 = 0;
pub const YAI_CMD_PING: u32 = 1;
pub const YAI_CMD_NOOP: u32 = 2;

pub const RESPONSE_BUFFER_LEN: usize = 1024;

// Default knowledge DB path (outside Consciousness repo)
pub const DEFAULT_KNOWLEDGE_DB_PATH: &str = "../Data/knowledge.db";
