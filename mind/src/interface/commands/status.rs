use crate::rpc::protocol::Response;
use crate::rpc::uds_client;
use crate::interface::config::RuntimeConfig;
use anyhow::Result;

pub fn run(cfg: &RuntimeConfig, ws: &str, json: bool) -> Result<()> {
    let sock = crate::control::workspace::control_socket_path(&cfg.run_dir, ws);
    if !sock.exists() {
        if json {
            println!("{{\"ws\":\"{}\",\"state\":\"down\"}}", ws);
        } else {
            println!("ws: {} (daemon down)", ws);
        }
        return Ok(());
    }

    match uds_client::send_request(&cfg.run_dir, ws, &crate::rpc::protocol::Request::Status)? {
        Response::Status {
            state,
            alive,
            daemon_pid,
            sanity,
            halt_reason,
        } => {
            if json {
                let payload = serde_json::json!({
                    "ws": ws,
                    "daemon_pid": daemon_pid,
                    "halt_reason": halt_reason,
                    "state": state,
                    "alive": alive,
                    "sanity": sanity,
                });
                println!("{}", payload);
                return Ok(());
            }
            println!("ws: {}", ws);
            println!("daemon: pid={}", daemon_pid);
            if let Some(reason) = halt_reason {
                println!("halt_reason: {}", reason);
            }
            if let Some(state) = state {
                println!("socket: {}", state.socket_path);
                println!("artifacts: {}", state.artifacts_root);
                if let Some(pgid) = state.pgid {
                    println!("pgid:  {}", pgid);
                }
                let boot_status = match state.boot_pid {
                    None => "not_started",
                    Some(_) if alive.boot => "running",
                    Some(_) => "completed",
                };
                println!("boot:   pid={:?} status={}", state.boot_pid, boot_status);
                println!("kernel: pid={:?} alive={}", state.kernel_pid, alive.kernel);
                println!("engine: pid={:?} alive={}", state.engine_pid, alive.engine);
                println!("mind:   pid={:?} alive={}", state.mind_pid, alive.mind);
                println!(
                    "sanity: runtime_sock_exists={} control_sock_exists={}",
                    sanity.runtime_sock_exists, sanity.control_sock_exists
                );
            } else {
                println!("state: down");
                println!(
                    "sanity: runtime_sock_exists={} control_sock_exists={}",
                    sanity.runtime_sock_exists, sanity.control_sock_exists
                );
            }
        }
        Response::Error { message } => {
            println!("status error: {}", message);
        }
        other => {
            println!("unexpected response: {:?}", other);
        }
    }
    Ok(())
}
