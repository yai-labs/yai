use crate::cli::config::RuntimeConfig;
use crate::cli::paths;
use crate::cli::proc::{is_pid_alive, pidfile_path, read_run_state, remove_pidfile, send_signal};
use crate::control::workspace;
use crate::transport::rpc::protocol::{Request, Response};
use crate::transport::rpc::uds_client;
use anyhow::{Context, Result};
use std::fs;
use std::thread;
use std::time::Duration;

pub fn run(cfg: &RuntimeConfig, ws: &str, force: bool) -> Result<()> {
    let sock = crate::control::workspace::control_socket_path(&cfg.run_dir, ws);
    if sock.exists() {
        let req = Request::Down {
            force,
            shutdown: true,
        };
        match uds_client::send_request(&cfg.run_dir, ws, &req) {
            Ok(Response::DownOk { .. }) => {
                println!("down complete for ws={}", ws);
                return Ok(());
            }
            Ok(Response::Error { message }) => {
                eprintln!("daemon down error: {}", message);
            }
            Ok(other) => {
                eprintln!("unexpected response: {:?}", other);
            }
            Err(err) => {
                eprintln!("daemon down failed, falling back: {}", err);
            }
        }
    }

    let pidfile = pidfile_path(&cfg.run_dir, ws);
    if !pidfile.exists() {
        println!("no pidfile for ws={}", ws);
        workspace::clear_halt(&cfg.run_dir, ws);
        return Ok(());
    }

    let state = read_run_state(&pidfile)?;

    // order: mind -> engine -> boot
    for pid in [state.mind_pid, state.engine_pid, state.boot_pid] {
        if let Some(pid) = pid {
            if is_pid_alive(pid) {
                let _ = send_signal(pid, "-TERM");
            }
        }
    }

    thread::sleep(Duration::from_secs(2));

    if force {
        for pid in [state.mind_pid, state.engine_pid, state.boot_pid] {
            if let Some(pid) = pid {
                if is_pid_alive(pid) {
                    let _ = send_signal(pid, "-KILL");
                }
            }
        }
    }

    let socket_path = paths::ws_socket_path(&cfg.socket_path, ws);
    if socket_path.exists() {
        fs::remove_file(&socket_path)
            .with_context(|| format!("remove socket: {}", socket_path.display()))?;
    }

    remove_pidfile(&pidfile)?;
    workspace::clear_halt(&cfg.run_dir, ws);
    println!("down complete for ws={}", ws);
    Ok(())
}
