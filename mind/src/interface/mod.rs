use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};

pub mod config;
pub mod paths;
pub mod proc;
pub mod commands;
pub mod tui;

#[derive(Parser, Debug)]
#[command(name = "yai", version, about = "YAI unified control suite")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Up(UpArgs),
    Restart(RestartArgs),
    Down(DownArgs),
    Status(StatusArgs),
    Logs(LogsArgs),
    Monitor(MonitorArgs),
    Events(EventsArgs),
    Verify(VerifyArgs),
    Test(TestArgs),
    Providers(ProvidersArgs),
    Sessions(SessionsArgs),
    Graph(GraphArgs),
    Embed(EmbedArgs),
    #[command(hide = true)]
    Daemon(DaemonArgs),
}

#[derive(Args, Debug, Clone, Default)]
pub struct CommonArgs {
    #[arg(long)]
    pub ws: Option<String>,
    #[arg(long)]
    pub workspace_root: Option<String>,
    #[arg(long)]
    pub artifacts_root: Option<String>,
    #[arg(long)]
    pub socket_path: Option<String>,
    #[arg(long)]
    pub yai_boot: Option<String>,
    #[arg(long)]
    pub yai_kernel: Option<String>,
    #[arg(long)]
    pub yai_engine: Option<String>,
    #[arg(long)]
    pub yai_mind: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[arg(long)]
    pub monitor: bool,
    #[arg(long)]
    pub ai: bool,
    #[arg(long)]
    pub no_engine: bool,
    #[arg(long)]
    pub no_mind: bool,
    #[arg(long)]
    pub detach: bool,
    #[arg(long)]
    pub build: bool,
    #[arg(long)]
    pub timeout_ms: Option<u64>,
}

#[derive(Args, Debug)]
pub struct RestartArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[arg(long)]
    pub monitor: bool,
    #[arg(long)]
    pub ai: bool,
    #[arg(long)]
    pub no_engine: bool,
    #[arg(long)]
    pub no_mind: bool,
    #[arg(long)]
    pub detach: bool,
    #[arg(long)]
    pub build: bool,
    #[arg(long)]
    pub timeout_ms: Option<u64>,
    #[arg(long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct DownArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[arg(long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct StatusArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[arg(long)]
    pub json: bool,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum LogComponent {
    Kernel,
    Engine,
    Mind,
    Boot,
}

#[derive(Args, Debug)]
pub struct LogsArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[arg(value_enum)]
    pub component: LogComponent,
    #[arg(long)]
    pub follow: bool,
}

#[derive(Args, Debug)]
pub struct MonitorArgs {
    #[command(flatten)]
    pub common: CommonArgs,
}

#[derive(Args, Debug)]
pub struct EventsArgs {
    #[command(flatten)]
    pub common: CommonArgs,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum VerifyTarget {
    Core,
    Full,
}

#[derive(Args, Debug)]
pub struct VerifyArgs {
    #[arg(value_enum)]
    pub target: VerifyTarget,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum TestTarget {
    Smoke,
}

#[derive(Args, Debug)]
pub struct TestArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[arg(value_enum)]
    pub target: TestTarget,
    #[arg(long)]
    pub timeout_ms: Option<u64>,
}

#[derive(Subcommand, Debug)]
pub enum SessionsCommand {
    List,
    Kill {
        ws: String,
        #[arg(long)]
        force: bool,
    },
}

#[derive(Args, Debug)]
pub struct SessionsArgs {
    #[command(subcommand)]
    pub command: SessionsCommand,
}

#[derive(Subcommand, Debug)]
pub enum ProvidersCommand {
    Discover,
    List,
    Pair {
        id: String,
        endpoint: String,
        model: String,
    },
    Attach {
        id: String,
        #[arg(long)]
        model: Option<String>,
    },
    Detach,
    Revoke {
        id: String,
    },
    Status,
}

#[derive(Args, Debug)]
pub struct ProvidersArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[command(subcommand)]
    pub command: ProvidersCommand,
}

#[derive(Subcommand, Debug)]
pub enum GraphCommand {
    AddNode {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long)]
        id: String,
        #[arg(long)]
        kind: String,
        #[arg(long, default_value = "{}")]
        meta: String,
    },
    AddEdge {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long)]
        src: String,
        #[arg(long)]
        dst: String,
        #[arg(long)]
        rel: String,
        #[arg(long, default_value_t = 1.0)]
        weight: f32,
    },
    Query {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long)]
        text: String,
        #[arg(long, default_value_t = 8)]
        k: usize,
    },
}

#[derive(Args, Debug)]
pub struct GraphArgs {
    #[command(subcommand)]
    pub command: GraphCommand,
}

#[derive(Args, Debug)]
pub struct EmbedArgs {
    #[arg(long, default_value = "hash")]
    pub provider: String,
    #[arg(long, default_value = "all-MiniLM-L6-v2")]
    pub model: String,
    #[arg(long)]
    pub endpoint: Option<String>,
    #[arg(long)]
    pub text: String,
}

#[derive(Args, Debug)]
pub struct DaemonArgs {
    #[command(flatten)]
    pub common: CommonArgs,
}

fn overrides_from(common: &CommonArgs) -> config::CliOverrides {
    config::CliOverrides {
        workspace_root: common.workspace_root.clone(),
        artifacts_root: common.artifacts_root.clone(),
        ws: common.ws.clone(),
        socket_path: common.socket_path.clone(),
        yai_boot: common.yai_boot.clone(),
        yai_kernel: common.yai_kernel.clone(),
        yai_engine: common.yai_engine.clone(),
        yai_mind: common.yai_mind.clone(),
    }
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Up(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            let runtime = commands::up::UpRuntime {
                ws,
                monitor: args.monitor,
                ai: args.ai,
                no_engine: args.no_engine,
                no_mind: args.no_mind,
                detach: args.detach,
                build: args.build,
                timeout_ms: args.timeout_ms,
            };
            commands::up::run(&cfg, &runtime)
        }
        Command::Restart(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            let runtime = commands::restart::RestartRuntime {
                ws,
                monitor: args.monitor,
                ai: args.ai,
                no_engine: args.no_engine,
                no_mind: args.no_mind,
                detach: args.detach,
                build: args.build,
                timeout_ms: args.timeout_ms,
                force: args.force,
            };
            commands::restart::run(&cfg, &runtime)
        }
        Command::Down(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            commands::down::run(&cfg, &ws, args.force)
        }
        Command::Status(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            commands::status::run(&cfg, &ws, args.json)
        }
        Command::Logs(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            let component = match args.component {
                LogComponent::Kernel => "kernel",
                LogComponent::Engine => "engine",
                LogComponent::Mind => "mind",
                LogComponent::Boot => "boot",
            };
            commands::logs::run(&cfg, &ws, component, args.follow, 200)
        }
        Command::Monitor(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            commands::monitor::run(&cfg, &ws)
        }
        Command::Events(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            commands::events::run(&cfg, &ws)
        }
        Command::Verify(args) => {
            let cfg = config::load_config(&config::CliOverrides::default())?;
            match args.target {
                VerifyTarget::Core => commands::verify::verify_core(&cfg),
                VerifyTarget::Full => commands::verify::verify_full(&cfg),
            }
        }
        Command::Test(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            let timeout_ms = args.timeout_ms.unwrap_or(5000);
            match args.target {
                TestTarget::Smoke => commands::verify::test_smoke(&cfg, &ws, timeout_ms),
            }
        }
        Command::Providers(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            match args.command {
                ProvidersCommand::Discover => commands::providers::discover(&cfg, &ws),
                ProvidersCommand::List => commands::providers::list(&cfg, &ws),
                ProvidersCommand::Pair { id, endpoint, model } => {
                    commands::providers::pair(&cfg, &ws, id, endpoint, model)
                }
                ProvidersCommand::Attach { id, model } => {
                    commands::providers::attach(&cfg, &ws, id, model)
                }
                ProvidersCommand::Detach => commands::providers::detach(&cfg, &ws),
                ProvidersCommand::Revoke { id } => commands::providers::revoke(&cfg, &ws, id),
                ProvidersCommand::Status => commands::providers::status(&cfg, &ws),
            }
        }
        Command::Sessions(args) => {
            let cfg = config::load_config(&config::CliOverrides::default())?;
            match args.command {
                SessionsCommand::List => commands::sessions::list(&cfg),
                SessionsCommand::Kill { ws, force } => commands::sessions::kill(&cfg, &ws, force),
            }
        }
        Command::Embed(args) => {
            let cfg = config::load_config(&config::CliOverrides::default())?;
            commands::embed::run(&cfg, &args.provider, &args.model, &args.endpoint, &args.text)
        }
        Command::Graph(args) => {
            let cfg = config::load_config(&config::CliOverrides::default())?;
            match args.command {
                GraphCommand::AddNode { ws, id, kind, meta } => {
                    let ws = ws.unwrap_or_else(|| cfg.ws_default.clone());
                    commands::graph::add_node(&cfg, &ws, &id, &kind, &meta)
                }
                GraphCommand::AddEdge { ws, src, dst, rel, weight } => {
                    let ws = ws.unwrap_or_else(|| cfg.ws_default.clone());
                    commands::graph::add_edge(&cfg, &ws, &src, &dst, &rel, weight)
                }
                GraphCommand::Query { ws, text, k } => {
                    let ws = ws.unwrap_or_else(|| cfg.ws_default.clone());
                    commands::graph::query(&cfg, &ws, &text, k)
                }
            }
        }
        Command::Daemon(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args.common.ws.clone().unwrap_or_else(|| cfg.ws_default.clone());
            crate::control::daemon::run_daemon(&cfg, &ws)
        }
    }
}
