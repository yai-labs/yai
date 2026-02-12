use crate::interface::config::RuntimeConfig;
use crate::interface::proc::log_path;
use crate::interface::tui::app::AppState;
use crate::interface::tui::datasource::DataSource;
use anyhow::Result;
use std::fs;

pub struct FileTailSource;

impl DataSource for FileTailSource {
    fn tick(&mut self, cfg: &RuntimeConfig, state: &mut AppState) -> Result<()> {
        let candidates = ["events", "awareness", "kernel", "engine", "mind", "boot"];
        let component = if candidates.contains(&state.logs.source_selected.as_str()) {
            state.logs.source_selected.clone()
        } else {
            "events".to_string()
        };
        let p = log_path(&cfg.run_dir, &state.ws, &component);
        if !p.exists() {
            state.logs.tail_buffer.clear();
            return Ok(());
        }
        let raw = fs::read_to_string(&p)?;
        let mut lines: Vec<String> = raw.lines().map(ToString::to_string).collect();
        if !state.logs.search_term.is_empty() {
            let q = state.logs.search_term.to_lowercase();
            lines.retain(|l| l.to_lowercase().contains(&q));
        }
        let keep = state.logs.lines.max(1);
        if lines.len() > keep {
            lines = lines[lines.len() - keep..].to_vec();
        }
        state.logs.tail_buffer = lines;
        Ok(())
    }
}
