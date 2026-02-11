use crate::interface::commands::up::UpRuntime;
use crate::interface::config::RuntimeConfig;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RestartRuntime {
    pub ws: String,
    pub monitor: bool,
    pub ai: bool,
    pub no_engine: bool,
    pub no_mind: bool,
    pub detach: bool,
    pub build: bool,
    pub timeout_ms: Option<u64>,
    pub force: bool,
}

pub fn run(cfg: &RuntimeConfig, args: &RestartRuntime) -> Result<()> {
    crate::interface::commands::down::run(cfg, &args.ws, args.force)?;
    let up = UpRuntime {
        ws: args.ws.clone(),
        monitor: args.monitor,
        ai: args.ai,
        no_engine: args.no_engine,
        no_mind: args.no_mind,
        detach: args.detach,
        build: args.build,
        timeout_ms: args.timeout_ms,
    };
    crate::interface::commands::up::run(cfg, &up)
}
