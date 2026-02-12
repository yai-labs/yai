use crate::interface::tui::app::AppState;

pub fn render(state: &AppState) -> String {
    let mut out = String::new();
    out.push_str(&format!("Workspace: {}\n", state.ws));
    out.push_str(&format!(
        "Runtime: boot={} kernel={} engine={} mind={}\n",
        state.status.boot_alive,
        state.status.kernel_alive,
        state.status.engine_alive,
        state.status.mind_alive
    ));
    out.push_str(&format!(
        "Sockets: runtime={} control={}\n",
        state.status.runtime_sock_exists,
        state.status.control_sock_exists
    ));
    out.push_str(&format!(
        "Awareness: active={} last='{}'\n",
        state.status.awareness_active,
        trim_line(&state.status.awareness_last_line)
    ));
    out.push_str(&format!(
        "Providers: count={} selected={:?} trust={}\n",
        state.providers.list.len(),
        state.providers.selected,
        state.providers.trust
    ));
    out.push_str(&format!(
        "Memory: nodes={} edges={} db_tables={} logs_lines={}\n",
        state.graph.stats_nodes,
        state.graph.stats_edges,
        state.db.tables.len(),
        state.logs.tail_buffer.len()
    ));
    out.push_str("\nRecent Events:\n");
    if state.events.items.is_empty() {
        out.push_str("  (no events yet)\n");
    } else {
        for line in state.events.items.iter().rev().take(10).rev() {
            out.push_str("  ");
            out.push_str(&trim_line(line));
            out.push('\n');
        }
    }
    out
}

fn trim_line(s: &str) -> String {
    const MAX: usize = 140;
    if s.len() <= MAX {
        s.to_string()
    } else {
        format!("{}...", &s[..MAX])
    }
}
