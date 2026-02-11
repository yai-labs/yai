use crate::memory::legacy::store::MemoryCore;

pub fn build_context(user_text: &str, memory: &MemoryCore, ws: &str) -> String {
    let mut out = String::new();
    out.push_str("# Context\n");
    out.push_str(&format!("User: {}\n", user_text));

    if let Ok(events) = memory.recent_events(ws, 5) {
        out.push_str("Recent Events:\n");
        for e in events {
            out.push_str(&format!("- [{}] {}\n", e.kind.as_str(), e.payload));
        }
    }

    if let Ok(facts) = memory.search_facts(user_text, 5) {
        out.push_str("Facts:\n");
        for f in facts {
            out.push_str(&format!("- {} = {}\n", f.key, f.value));
        }
    }

    out
}
