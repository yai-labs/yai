use crate::interface::tui::app::AppState;

pub fn render(state: &AppState) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "events last_n={} buffered={}\n\n",
        state.events.last_n,
        state.events.items.len()
    ));
    if state.events.items.is_empty() {
        out.push_str("no events yet\n");
        return out;
    }
    for (i, line) in state.events.items.iter().rev().take(80).rev().enumerate() {
        let marker = if i == state.events.selected { ">" } else { " " };
        out.push_str(marker);
        out.push(' ');
        if state.events.expanded || i == state.events.selected {
            out.push_str(line);
        } else {
            out.push_str(&line.chars().take(140).collect::<String>());
            if line.len() > 140 {
                out.push_str("...");
            }
        }
        out.push('\n');
    }
    out.push_str("\nKeys: ↑/↓ select  Enter expand/collapse\n");
    out
}
