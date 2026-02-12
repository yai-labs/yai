use crate::interface::tui::app::AppState;

pub fn render(state: &AppState) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "db scope={:?} tables={} selected={:?}\n\n",
        state.db.selected_db,
        state.db.tables.len(),
        state.db.selected_table
    ));
    if state.db.tables.is_empty() {
        out.push_str("no tables available\n");
        return out;
    }
    out.push_str("Tables:\n");
    for t in state.db.tables.iter().take(20) {
        let c = state.db.counts.get(t).copied().unwrap_or(0);
        out.push_str(&format!("  - {} ({})\n", t, c));
    }
    out.push_str("\nPreview:\n");
    if state.db.preview.is_empty() {
        out.push_str("  (no preview rows)\n");
    } else {
        for row in state.db.preview.iter().take(10) {
            out.push_str("  ");
            out.push_str(&row.to_string());
            out.push('\n');
        }
    }
    out
}
