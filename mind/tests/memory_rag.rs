use yai_mind::cognition::memory::{EventKind, MemoryCore};
use yai_mind::cognition::rag::pipeline::build_prompt;

#[test]
fn memory_store_and_rag_context() {
    let core = MemoryCore::new();

    core.put_event("ws", "trace", EventKind::User, "hello")
        .unwrap();
    core.put_event("ws", "trace", EventKind::Agent, "ok")
        .unwrap();
    core.put_fact("ws", "key", "value", &[]).unwrap();

    let events = core.recent_events("ws", 10).unwrap();
    assert!(events.len() >= 2);

    let facts = core.search_facts("ws", "key", 10).unwrap();
    assert!(!facts.is_empty());

    let prompt = build_prompt("hello", &core, "ws");
    assert!(prompt.contains("Context"));
    assert!(prompt.contains("hello"));
}
