use crate::interface::tui::app::AppState;

pub fn render(state: &AppState) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "contracts graph={} commands={} compliance={} last_check={}\n",
        state.contracts.graph_spec_version,
        state.contracts.commands_spec_version,
        state.contracts.compliance_pack,
        state.contracts.last_check
    ));
    out.push_str(&format!("violations={}\n\n", state.contracts.violations.len()));
    if state.contracts.files.is_empty() {
        out.push_str("no contract files loaded\n");
        return out;
    }
    for f in &state.contracts.files {
        out.push_str(&format!(
            "- {}  mtime={}  checksum={}\n",
            f.path, f.modified_epoch, f.checksum
        ));
    }
    out
}
