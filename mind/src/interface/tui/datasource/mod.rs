use crate::interface::config::RuntimeConfig;
use crate::interface::tui::app::{AppError, AppState};
use anyhow::Result;

pub mod runtime;
pub mod logs;
pub mod events;
pub mod db;
pub mod graph;
pub mod providers;
pub mod contracts;
pub mod chat;

pub trait DataSource {
    fn tick(&mut self, cfg: &RuntimeConfig, state: &mut AppState) -> Result<()>;
}

pub fn tick_all(
    cfg: &RuntimeConfig,
    state: &mut AppState,
    sources: &mut [Box<dyn DataSource>],
) {
    for src in sources.iter_mut() {
        if let Err(e) = src.tick(cfg, state) {
            state.errors.push(AppError {
                source: "datasource".to_string(),
                message: e.to_string(),
            });
        }
    }
}
