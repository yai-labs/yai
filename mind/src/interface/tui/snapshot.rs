use crate::interface::config::RuntimeConfig;
use crate::interface::tui::app::{AppState, ViewKind};
use crate::interface::tui::datasource::chat::ChatSource;
use crate::interface::tui::datasource::contracts::ContractsSource;
use crate::interface::tui::datasource::{tick_all, DataSource};
use crate::interface::tui::datasource::db::DbSource;
use crate::interface::tui::datasource::events::EventsSource;
use crate::interface::tui::datasource::graph::GraphSource;
use crate::interface::tui::datasource::logs::FileTailSource;
use crate::interface::tui::datasource::providers::ProvidersSource;
use crate::interface::tui::datasource::runtime::RuntimeSource;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub struct ViewSnapshot {
    pub ts: u64,
    pub ws: String,
    pub view: String,
    pub payload: Value,
}

pub fn snapshot_for_view(cfg: &RuntimeConfig, ws: &str, view: ViewKind) -> Result<ViewSnapshot> {
    let mut state = AppState::new(ws.to_string());
    state.active_view = view.clone();
    let mut sources: Vec<Box<dyn DataSource>> = vec![
        Box::new(RuntimeSource),
        Box::new(EventsSource),
        Box::new(GraphSource),
        Box::new(FileTailSource),
        Box::new(DbSource),
        Box::new(ProvidersSource),
        Box::new(ContractsSource),
        Box::new(ChatSource),
    ];
    tick_all(cfg, &mut state, &mut sources);

    let payload = match view {
        ViewKind::Overview => serde_json::to_value(&state.status)?,
        ViewKind::Graph => serde_json::to_value(&state.graph)?,
        ViewKind::Events => serde_json::to_value(&state.events)?,
        ViewKind::Logs => serde_json::to_value(&state.logs)?,
        ViewKind::Db => serde_json::to_value(&state.db)?,
        ViewKind::Providers => serde_json::to_value(&state.providers)?,
        ViewKind::Contracts => serde_json::to_value(&state.contracts)?,
        ViewKind::Chat => serde_json::to_value(&state.chat)?,
    };

    Ok(ViewSnapshot {
        ts: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        ws: ws.to_string(),
        view: format!("{:?}", view).to_lowercase(),
        payload,
    })
}
