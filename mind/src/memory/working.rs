#![allow(dead_code)]
use crate::memory::contracts::MemoryResult;
use crate::memory::legacy::store::MemoryCore;
use crate::memory::legacy::types::Event;

pub fn recent_events(core: &MemoryCore, ws: &str, limit: usize) -> MemoryResult<Vec<Event>> {
    core.recent_events(ws, limit)
}
