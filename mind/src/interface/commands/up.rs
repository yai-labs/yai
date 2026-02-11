use crate::control::daemon;
use crate::rpc::protocol::{Request, Response};
use crate::rpc::uds_client;
use crate::interface::commands::logs;
use crate::interface::config::RuntimeConfig;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct UpRuntime {
    pub ws: String,
    pub monitor: bool,
    pub ai: bool,
    pub no_engine: bool,
    pub no_mind: bool,
    pub detach: bool,
    pub build: bool,
    pub timeout_ms: Option<u64>,
}

pub fn run(cfg: &RuntimeConfig, args: &UpRuntime) -> Result<()> {
    let ws = &args.ws;

    if args.monitor {
        super::monitor::spawn_external_terminal(cfg, ws)?;
    }

    daemon::ensure_daemon(cfg, ws)?;
    let req = Request::Up {
        build: args.build,
        no_engine: args.no_engine,
        no_mind: args.no_mind,
        ai: args.ai,
        timeout_ms: args.timeout_ms.or(Some(5000)),
    };
    let resp = uds_client::send_request(&cfg.run_dir, ws, &req)?;
    match resp {
        Response::UpOk => {}
        Response::Error { message } => anyhow::bail!("up failed: {}", message),
        other => anyhow::bail!("unexpected response: {:?}", other),
    }

    if args.detach {
        println!("up complete for ws={} (detached)", ws);
        return Ok(());
    }

    // Attach mode: follow logs
    let component = if !args.no_mind {
        "mind"
    } else if !args.no_engine {
        "engine"
    } else {
        "boot"
    };
    logs::run(cfg, ws, component, true, 200)?;
    Ok(())
}
