use crate::memory::graph::api::{GraphFacade, GraphScope};

pub fn build_context(
    user_text: &str,
    scope: GraphScope,
) -> String {
    let mut out = String::new();
    out.push_str("# Context\n");
    out.push_str(&format!("User: {}\n", user_text));

    if let Ok(stats) = GraphFacade::stats(scope.clone()) {
        out.push_str(&format!(
            "Graph: {} nodes / {} edges\n",
            stats.nodes, stats.edges
        ));
    }

    out
}
