#![allow(dead_code)]
use crate::memory::contracts::MemoryResult;
use crate::memory::legacy::store::MemoryCore;

pub fn put_fact(core: &MemoryCore, key: &str, value: &str, tags: &[String]) -> MemoryResult<()> {
    core.put_fact(key, value, tags)
}
