use crate::shared::constants::{
    YAI_CMD_NONE, YAI_CMD_NOOP, YAI_CMD_PING, YAI_STATE_ERROR, YAI_STATE_HALT,
    YAI_STATE_HANDOFF_COMPLETE, YAI_STATE_PREBOOT, YAI_STATE_READY, YAI_STATE_RUNNING,
    YAI_STATE_SUSPENDED,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EngineStatus {
    Halt = YAI_STATE_HALT,
    Preboot = YAI_STATE_PREBOOT,
    Ready = YAI_STATE_READY,
    HandoffComplete = YAI_STATE_HANDOFF_COMPLETE,
    Running = YAI_STATE_RUNNING,
    Suspended = YAI_STATE_SUSPENDED,
    Error = YAI_STATE_ERROR,
}

impl EngineStatus {
    pub fn from_raw(value: u32) -> EngineStatus {
        match value {
            YAI_STATE_PREBOOT => EngineStatus::Preboot,
            YAI_STATE_READY => EngineStatus::Ready,
            YAI_STATE_HANDOFF_COMPLETE => EngineStatus::HandoffComplete,
            YAI_STATE_RUNNING => EngineStatus::Running,
            YAI_STATE_SUSPENDED => EngineStatus::Suspended,
            YAI_STATE_ERROR => EngineStatus::Error,
            _ => EngineStatus::Halt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CommandId {
    None = YAI_CMD_NONE,
    Ping = YAI_CMD_PING,
    Noop = YAI_CMD_NOOP,
}

impl CommandId {
    pub fn from_raw(value: u32) -> CommandId {
        match value {
            YAI_CMD_PING => CommandId::Ping,
            YAI_CMD_NOOP => CommandId::Noop,
            _ => CommandId::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AgentId {
    System = 1,
    Code = 2,
    Historian = 3,
    Validator = 4,
    Knowledge = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct RoutingDecision {
    pub agent: AgentId,
    pub command: CommandId,
}
