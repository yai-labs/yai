use crate::interface::config::RuntimeConfig;
use crate::interface::tui::app::AppState;
use crate::interface::tui::datasource::DataSource;
use crate::rpc::protocol::{Request, Response};
use crate::rpc::uds_client;
use anyhow::Result;
use std::fs;

pub struct RuntimeSource;

impl DataSource for RuntimeSource {
    fn tick(&mut self, cfg: &RuntimeConfig, state: &mut AppState) -> Result<()> {
        let resp = uds_client::send_request(&cfg.run_dir, &state.ws, &Request::Status)?;
        if let Response::Status { alive, sanity, .. } = resp {
            state.status.boot_alive = alive.boot;
            state.status.kernel_alive = alive.kernel;
            state.status.engine_alive = alive.engine;
            state.status.mind_alive = alive.mind;
            state.status.runtime_sock_exists = sanity.runtime_sock_exists;
            state.status.control_sock_exists = sanity.control_sock_exists;
        }
        let awareness_path = cfg.run_dir.join(&state.ws).join("awareness.log");
        if awareness_path.exists() {
            if let Ok(raw) = fs::read_to_string(&awareness_path) {
                let last = raw.lines().last().unwrap_or("").to_string();
                state.status.awareness_active = !last.is_empty();
                state.status.awareness_last_line = last;
            }
        } else {
            state.status.awareness_active = false;
            state.status.awareness_last_line.clear();
        }
        Ok(())
    }
}
