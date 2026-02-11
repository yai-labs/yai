use yai_mind::memory::{MemoryCore, SqliteMemoryStore};
use yai_mind::memory::legacy::types::EventKind;
use yai_mind::rag::pipeline::build_prompt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;

fn temp_db_path() -> PathBuf {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    std::env::temp_dir().join(format!("yai_memory_{}.db", ts))
}

#[test]
fn memory_store_and_rag_context() {
    let db = temp_db_path();
    let store = SqliteMemoryStore::new(&db);
    let core = MemoryCore::new(Box::new(store));

    core.put_event("ws", "trace", EventKind::User, "hello").unwrap();
    core.put_event("ws", "trace", EventKind::Agent, "ok").unwrap();
    core.put_fact("key", "value", &[]).unwrap();

    let events = core.recent_events("ws", 10).unwrap();
    assert!(events.len() >= 2);

    let facts = core.search_facts("key", 10).unwrap();
    assert!(!facts.is_empty());

    let prompt = build_prompt("hello", &core, "ws");
    assert!(prompt.contains("Context"));
    assert!(prompt.contains("hello"));
}
