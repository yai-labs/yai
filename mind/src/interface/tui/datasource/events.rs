use crate::interface::config::RuntimeConfig;
use crate::interface::proc::log_path;
use crate::interface::tui::app::AppState;
use crate::interface::tui::datasource::DataSource;
use anyhow::Result;
use std::fs;

pub struct EventsSource;

impl DataSource for EventsSource {
    fn tick(&mut self, cfg: &RuntimeConfig, state: &mut AppState) -> Result<()> {
        let p = log_path(&cfg.run_dir, &state.ws, "events");
        if !p.exists() {
            state.events.items.clear();
            return Ok(());
        }
        let raw = fs::read_to_string(&p)?;
        let mut lines: Vec<String> = raw.lines().map(ToString::to_string).collect();
        let keep = state.events.last_n.max(1);
        if lines.len() > keep {
            lines = lines[lines.len() - keep..].to_vec();
        }
        state.events.items = lines;
        Ok(())
    }
}
