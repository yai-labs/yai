use crate::interface::commands::down;
use crate::interface::config::RuntimeConfig;
use crate::control::workspace::control_socket_path;
use crate::rpc::protocol::{Request, Response};
use crate::rpc::uds_client;
use anyhow::Result;
use std::fs;

pub fn list(cfg: &RuntimeConfig) -> Result<()> {
    let mut entries = Vec::new();
    if cfg.run_dir.exists() {
        for entry in fs::read_dir(&cfg.run_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(ws) = entry.file_name().to_str() {
                    let sock = control_socket_path(&cfg.run_dir, ws);
                    if sock.exists() {
                        entries.push(ws.to_string());
                    }
                }
            }
        }
    }

    if entries.is_empty() {
        println!("no sessions");
        return Ok(());
    }

    for ws in entries {
        match uds_client::send_request(&cfg.run_dir, &ws, &Request::Status) {
            Ok(Response::Status { daemon_pid, .. }) => {
                println!("ws: {} daemon={}", ws, daemon_pid);
            }
            _ => {
                println!("ws: {} daemon=unknown", ws);
            }
        }
    }
    Ok(())
}

pub fn kill(cfg: &RuntimeConfig, ws: &str, force: bool) -> Result<()> {
    down::run(cfg, ws, force)
}
