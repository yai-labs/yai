use crate::cognition::memory::MemoryCore;
use crate::cognition::rag::context_builder::build_context;

pub fn build_prompt(user_text: &str, memory: &MemoryCore, ws: &str) -> String {
    let context = build_context(user_text, memory, ws);
    format!("{}\n\n# Task\n{}", context, user_text)
}
