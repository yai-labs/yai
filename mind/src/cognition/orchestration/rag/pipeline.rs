use crate::cognition::orchestration::rag::context_builder::build_context;
use crate::memory::graph::api::{GraphFacade, GraphScope};

pub fn build_prompt(user_text: &str, ws: &str) -> String {
    let scope = GraphScope::Workspace(ws.to_string());

    let context = build_context(user_text, scope);

    format!("{}\n\n# Task\n{}", context, user_text)
}
