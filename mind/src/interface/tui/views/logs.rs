use crate::interface::tui::app::AppState;

pub fn render(state: &AppState) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "source={} lines={} follow={} search='{}'\n",
        state.logs.source_selected,
        state.logs.lines,
        state.logs.follow,
        state.logs.search_term
    ));
    out.push_str("Filters: s=cycle-source  /=search via palette (: search <term>)  r=refresh\n\n");
    if state.logs.tail_buffer.is_empty() {
        out.push_str(&format!(
            "no logs yet for source={}\n",
            state.logs.source_selected
        ));
        return out;
    }
    for (i, line) in state.logs.tail_buffer.iter().rev().take(50).rev().enumerate() {
        let marker = if i == state.logs.selected { ">" } else { " " };
        out.push_str(marker);
        out.push(' ');
        if line.contains("ERROR") || line.contains("error") {
            out.push_str("[ERR] ");
        } else if line.contains("WARN") || line.contains("warn") {
            out.push_str("[WRN] ");
        } else {
            out.push_str("[INF] ");
        }
        out.push_str(line);
        out.push('\n');
    }
    out.push_str("\nKeys: ↑/↓ select  s source  / search  f follow\n");
    out
}
