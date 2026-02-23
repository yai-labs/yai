// Fix import: GraphFacade dal modulo facade, GraphScope dai types centralizzati
use crate::memory::graph::facade::GraphFacade;
use crate::types::graph::GraphScope;

pub fn build_context(user_text: &str, scope: GraphScope) -> String {
    let mut out = String::new();
    out.push_str("# Context\n");
    out.push_str(&format!("User: {}\n", user_text));

    // Nota: GraphFacade deve implementare il metodo .stats()
    // per restituire il conteggio di nodi e archi
    if let Ok(stats) = GraphFacade::stats(scope.clone()) {
        out.push_str(&format!(
            "Graph: {} nodes / {} edges\n",
            stats.nodes, stats.edges
        ));
    }

    out
}
