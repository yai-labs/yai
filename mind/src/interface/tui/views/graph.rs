use crate::interface::tui::app::AppState;

pub fn render(state: &AppState) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Graph backend: {}\nnodes={} edges={} depth={}\n",
        state.graph.backend,
        state.graph.stats_nodes,
        state.graph.stats_edges,
        state.graph.depth.max(1)
    ));
    out.push_str(&format!(
        "Selected: {:?} kind={:?} last_seen={}\n",
        state.graph.selected_node,
        state.graph.selected_node_kind,
        state.graph.selected_node_last_seen
    ));
    out.push_str("\nNode list (top by last_seen):\n");
    if state.graph.node_list.is_empty() {
        out.push_str("  (empty)\n");
    } else {
        for (i, id) in state.graph.node_list.iter().take(20).enumerate() {
            let marker = if i == state.graph.selected_index { ">" } else { " " };
            out.push_str(&format!("{} {}\n", marker, id));
        }
    }
    if !state.graph.selected_node_meta.is_null() {
        out.push_str("Meta:\n");
        out.push_str(&format!("{}\n", state.graph.selected_node_meta));
    }
    out.push_str("\nNeighbors preview:\n");
    if state.graph.neighbors_preview.is_empty() {
        out.push_str("  (no neighbors, select node via ': node <id>')\n");
    } else {
        for line in state.graph.neighbors_preview.iter().take(16) {
            out.push_str("  ");
            out.push_str(line);
            out.push('\n');
        }
    }
    out.push_str("\nActivation top:\n");
    if state.graph.activation_top.is_empty() {
        out.push_str("  (press 'a' after selecting a node)\n");
    } else {
        for line in state.graph.activation_top.iter().take(12) {
            out.push_str("  ");
            out.push_str(line);
            out.push('\n');
        }
    }
    out.push_str("\nActions: n=depth1/2  a=activate  : node <id>  : search <term>\n");
    out
}
