use crate::interface::tui::app::AppState;

pub fn render(state: &AppState) -> String {
    let mut out = String::new();
    let selected_provider = state
        .providers
        .selected
        .as_ref()
        .and_then(|id| state.providers.list.iter().find(|p| &p.id == id))
        .or_else(|| state.providers.list.first());
    let selected = selected_provider
        .map(|p| p.id.as_str())
        .unwrap_or("none");
    let (trust, model) = selected_provider
        .map(|p| (p.trust_state.as_str(), p.model.as_str()))
        .unwrap_or((
            if state.providers.trust.is_empty() {
                "unknown"
            } else {
                state.providers.trust.as_str()
            },
            "unknown",
        ));
    out.push_str("=== Header ===\n");
    out.push_str(&format!(
        "ws={} provider={} trust={} model={} agent={} command={} streaming={} request_state={:?}\n\n",
        state.ws,
        selected,
        trust,
        model,
        state.chat.last_agent.as_deref().unwrap_or("n/a"),
        state.chat.last_command.as_deref().unwrap_or("n/a"),
        state.chat.streaming_enabled,
        state.chat.request_state
    ));

    out.push_str("=== Transcript (scroll) ===\n");
    if state.chat.transcript.is_empty() {
        out.push_str("  (empty)\n");
    } else {
        let start = state.chat.scroll.min(state.chat.transcript.len());
        let tail = &state.chat.transcript[start..];
        for (i, m) in tail.iter().rev().take(20).rev().enumerate() {
            let marker = if i == state.chat.selected_index { ">" } else { " " };
            out.push_str(&format!(
                "{} [{} #{} @{} | {:?}] {}\n",
                marker, m.role, m.id, m.ts, m.status, m.text
            ));
        }
    }
    out.push('\n');

    out.push_str("=== Composer (multiline) ===\n");
    if selected_provider.is_none() {
        out.push_str("No provider selected. Press p and Enter, or use ': provider select <id>'\n");
    }
    out.push_str(&state.chat.input);
    out.push_str(
        "\n\nKeys: Enter=send  Shift+Enter=newline  Ctrl+U=clear  Esc=navigator  Ctrl+P=palette\n\n",
    );

    out.push_str("=== Action / Commit Bar ===\n");
    out.push_str(
        "[C] commit  [x] discard  [r] retry  [E] edit prompt  [s] streaming  [t] cycle target\n",
    );
    out.push_str(&format!("Commit target: {:?}\n", state.chat.commit_target));
    if let Some(plan) = &state.chat.draft_plan {
        out.push_str(&format!(
            "PlanDraft: {} requires_apply={}\n",
            plan.summary, plan.requires_apply
        ));
    }
    if let Some(agent) = &state.chat.last_agent {
        out.push_str(&format!("Pipeline: routed_to_agent={}\n", agent));
    }
    if !state.chat.last_error.is_empty() {
        out.push_str(&format!("Last error: {}\n", state.chat.last_error));
    }
    out
}
