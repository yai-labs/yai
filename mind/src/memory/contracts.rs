#![allow(dead_code)]
use crate::memory::legacy::types::{Event, EventKind, Fact};

pub type MemoryResult<T> = Result<T, String>;

pub trait MemoryStore: Send + Sync {
    fn put_event(&self, ws: &str, trace: &str, kind: EventKind, payload: &str) -> MemoryResult<()>;
    fn recent_events(&self, ws: &str, limit: usize) -> MemoryResult<Vec<Event>>;
    fn put_fact(&self, key: &str, value: &str, tags: &[String]) -> MemoryResult<()>;
    fn search_facts(&self, query: &str, limit: usize) -> MemoryResult<Vec<Fact>>;
}
