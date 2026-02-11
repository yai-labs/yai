#![allow(dead_code)]
use crate::memory::contracts::MemoryResult;
use crate::memory::legacy::store::MemoryCore;
use crate::memory::legacy::types::EventKind;

pub fn record_user_event(core: &MemoryCore, ws: &str, trace: &str, payload: &str) -> MemoryResult<()> {
    core.put_event(ws, trace, EventKind::User, payload)
}

pub fn record_agent_event(core: &MemoryCore, ws: &str, trace: &str, payload: &str) -> MemoryResult<()> {
    core.put_event(ws, trace, EventKind::Agent, payload)
}
