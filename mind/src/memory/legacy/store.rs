#![allow(dead_code)]
use crate::memory::contracts::{MemoryStore, MemoryResult};
use crate::memory::legacy::types::{Event, EventKind, Fact};

pub struct MemoryCore {
    store: Box<dyn MemoryStore>,
}

impl MemoryCore {
    pub fn new(store: Box<dyn MemoryStore>) -> Self {
        Self { store }
    }

    pub fn put_event(&self, ws: &str, trace: &str, kind: EventKind, payload: &str) -> MemoryResult<()> {
        self.store.put_event(ws, trace, kind, payload)
    }

    pub fn recent_events(&self, ws: &str, limit: usize) -> MemoryResult<Vec<Event>> {
        self.store.recent_events(ws, limit)
    }

    pub fn put_fact(&self, key: &str, value: &str, tags: &[String]) -> MemoryResult<()> {
        self.store.put_fact(key, value, tags)
    }

    pub fn search_facts(&self, query: &str, limit: usize) -> MemoryResult<Vec<Fact>> {
        self.store.search_facts(query, limit)
    }
}
