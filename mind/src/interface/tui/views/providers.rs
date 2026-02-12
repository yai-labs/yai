use crate::interface::tui::app::AppState;

pub fn render(state: &AppState) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "providers count={} selected={:?} trust={}\n\n",
        state.providers.list.len(),
        state.providers.selected,
        state.providers.trust
    ));
    if state.providers.list.is_empty() {
        out.push_str("no providers in trust store\n");
        return out;
    }
    for (i, p) in state.providers.list.iter().take(30).enumerate() {
        let marker = if i == state.providers.selected_index { ">" } else { " " };
        out.push_str(&format!(
            "{} id={} trust={} last_seen={} endpoint={}\n",
            marker, p.id, p.trust_state, p.last_seen, p.endpoint
        ));
    }
    out.push_str("\nActions: use CLI for trust changes (`yai providers trust ...`) and refresh with r\n");
    out
}
