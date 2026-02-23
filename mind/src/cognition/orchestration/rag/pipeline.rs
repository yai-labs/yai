use crate::cognition::orchestration::rag::context_builder::build_context;
// Fix import: GraphFacade è nel modulo facade, GraphScope è nei types
use crate::types::graph::GraphScope;

pub fn build_prompt(user_text: &str, ws: &str) -> String {
    let scope = GraphScope::Workspace(ws.to_string());

    // Nota: Assicurati che build_context accetti GraphScope come secondo argomento
    let context = build_context(user_text, scope);

    format!("{}\n\n# Task\n{}", context, user_text)
}
