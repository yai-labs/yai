#![allow(dead_code)]
use crate::core::scheduler::Scheduler;
use crate::memory::legacy::store::MemoryCore;

// Stub: awareness loop will be implemented with a non-busy async task later.
pub fn start_awareness(_core: MemoryCore, _scheduler: Scheduler, _ws: String, _trace: String) {}
