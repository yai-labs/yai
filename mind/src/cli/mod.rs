use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};

pub mod commands;
pub mod config;
pub mod paths;
pub mod proc;

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
    Dsar(DsarArgs),
    Sessions(SessionsArgs),
    Chat(ChatArgs),
    Shell(ShellArgs),
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
pub enum DsarCommand {
    Request {
        request_type: String,
        #[arg(long)]
        subject: String,
    },
    Status {
        request_id: String,
    },
    Execute {
        request_id: String,
    },
}

#[derive(Args, Debug)]
pub struct DsarArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[command(subcommand)]
    pub command: DsarCommand,
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
pub enum ChatCommand {
    List,
    New {
        #[arg(long)]
        title: Option<String>,
    },
    Select {
        session_id: String,
    },
    History {
        #[arg(long)]
        session: Option<String>,
    },
    Send {
        #[arg(long)]
        session: Option<String>,
        #[arg(long, default_value_t = false)]
        stream: bool,
        text: String,
    },
}

#[derive(Args, Debug)]
pub struct ChatArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[command(subcommand)]
    pub command: ChatCommand,
}

#[derive(Subcommand, Debug)]
pub enum ShellCommand {
    Exec {
        #[arg(long)]
        cwd: Option<String>,
        cmd: String,
        args: Vec<String>,
    },
}

#[derive(Args, Debug)]
pub struct ShellArgs {
    #[command(flatten)]
    pub common: CommonArgs,
    #[command(subcommand)]
    pub command: ShellCommand,
}

#[derive(Subcommand, Debug)]
pub enum ProvidersCommand {
    Discover,
    List,
    Trust {
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        endpoint: Option<String>,
        #[arg(long, value_enum)]
        state: ProvidersTrustState,
    },
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

#[derive(ValueEnum, Debug, Clone)]
pub enum ProvidersTrustState {
    Discovered,
    Trusted,
    Revoked,
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
        #[arg(long, default_value_t = false)]
        global: bool,
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
        #[arg(long, default_value_t = false)]
        global: bool,
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
        #[arg(long, default_value_t = false)]
        global: bool,
        #[arg(long)]
        text: String,
        #[arg(long, default_value_t = 8)]
        k: usize,
    },
    Stats {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long, default_value_t = false)]
        global: bool,
    },
    Node {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long, default_value_t = false)]
        global: bool,
        id: String,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    Neighbors {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long, default_value_t = false)]
        global: bool,
        id: String,
        #[arg(long, default_value_t = 1)]
        depth: usize,
        #[arg(long = "rel")]
        rels: Vec<String>,
        #[arg(long = "kind")]
        kinds: Vec<String>,
    },
    Export {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long, default_value_t = false)]
        global: bool,
        #[arg(long)]
        format: String,
        #[arg(long)]
        out: String,
    },
    Awareness {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long, default_value_t = 250)]
        tick_ms: u64,
        #[arg(long)]
        max_steps: Option<u64>,
    },
    Activate {
        #[arg(long)]
        ws: Option<String>,
        #[arg(long, default_value_t = false)]
        global: bool,
        #[arg(long = "seed")]
        seeds: Vec<String>,
        #[arg(long, default_value_t = 20)]
        topk: usize,
        #[arg(long, default_value_t = 0.15)]
        alpha: f64,
        #[arg(long, default_value_t = 1e-6)]
        epsilon: f64,
        #[arg(long, default_value_t = false)]
        no_trace: bool,
    },
    Trace {
        #[command(subcommand)]
        command: GraphTraceCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum GraphTraceCommand {
    Show { run_id: String },
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
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
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
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
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
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            commands::down::run(&cfg, &ws, args.force)
        }
        Command::Status(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            commands::status::run(&cfg, &ws, args.json)
        }
        Command::Logs(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
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
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            commands::monitor::run(&cfg, &ws)
        }
        Command::Events(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
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
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            let timeout_ms = args.timeout_ms.unwrap_or(5000);
            match args.target {
                TestTarget::Smoke => commands::verify::test_smoke(&cfg, &ws, timeout_ms),
            }
        }
        Command::Providers(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            match args.command {
                ProvidersCommand::Discover => commands::providers::discover(&cfg, &ws),
                ProvidersCommand::List => commands::providers::list(&cfg, &ws),
                ProvidersCommand::Trust {
                    id,
                    endpoint,
                    state,
                } => commands::providers::trust(&cfg, &ws, id, endpoint, state),
                ProvidersCommand::Pair {
                    id,
                    endpoint,
                    model,
                } => commands::providers::pair(&cfg, &ws, id, endpoint, model),
                ProvidersCommand::Attach { id, model } => {
                    commands::providers::attach(&cfg, &ws, id, model)
                }
                ProvidersCommand::Detach => commands::providers::detach(&cfg, &ws),
                ProvidersCommand::Revoke { id } => commands::providers::revoke(&cfg, &ws, id),
                ProvidersCommand::Status => commands::providers::status(&cfg, &ws),
            }
        }
        Command::Dsar(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            match args.command {
                DsarCommand::Request {
                    request_type,
                    subject,
                } => commands::dsar::request(&cfg, &ws, request_type, subject),
                DsarCommand::Status { request_id } => commands::dsar::status(&cfg, &ws, request_id),
                DsarCommand::Execute { request_id } => {
                    commands::dsar::execute(&cfg, &ws, request_id)
                }
            }
        }
        Command::Sessions(args) => {
            let cfg = config::load_config(&config::CliOverrides::default())?;
            match args.command {
                SessionsCommand::List => commands::sessions::list(&cfg),
                SessionsCommand::Kill { ws, force } => commands::sessions::kill(&cfg, &ws, force),
            }
        }
        Command::Chat(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            match args.command {
                ChatCommand::List => commands::chat::list(&cfg, &ws),
                ChatCommand::New { title } => commands::chat::new(&cfg, &ws, title),
                ChatCommand::Select { session_id } => {
                    commands::chat::select(&cfg, &ws, &session_id)
                }
                ChatCommand::History { session } => commands::chat::history(&cfg, &ws, session),
                ChatCommand::Send {
                    session,
                    stream,
                    text,
                } => commands::chat::send(&cfg, &ws, session, stream, &text),
            }
        }
        Command::Shell(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            match args.command {
                ShellCommand::Exec { cwd, cmd, args } => {
                    commands::shell::exec(&cfg, &ws, &cmd, &args, cwd)
                }
            }
        }
        Command::Embed(args) => {
            let cfg = config::load_config(&config::CliOverrides::default())?;
            commands::embed::run(
                &cfg,
                &args.provider,
                &args.model,
                &args.endpoint,
                &args.text,
            )
        }
        Command::Graph(args) => {
            let cfg = config::load_config(&config::CliOverrides::default())?;
            match args.command {
                GraphCommand::AddNode {
                    ws,
                    global,
                    id,
                    kind,
                    meta,
                } => commands::graph::add_node(&cfg, ws.as_deref(), global, &id, &kind, &meta),
                GraphCommand::AddEdge {
                    ws,
                    global,
                    src,
                    dst,
                    rel,
                    weight,
                } => {
                    commands::graph::add_edge(&cfg, ws.as_deref(), global, &src, &dst, &rel, weight)
                }
                GraphCommand::Query {
                    ws,
                    global,
                    text,
                    k,
                } => commands::graph::query(&cfg, ws.as_deref(), global, &text, k),
                GraphCommand::Stats { ws, global } => {
                    commands::graph::stats(&cfg, ws.as_deref(), global)
                }
                GraphCommand::Node {
                    ws,
                    global,
                    id,
                    limit,
                } => commands::graph::node(&cfg, ws.as_deref(), global, &id, limit),
                GraphCommand::Neighbors {
                    ws,
                    global,
                    id,
                    depth,
                    rels,
                    kinds,
                } => commands::graph::neighbors(
                    &cfg,
                    ws.as_deref(),
                    global,
                    &id,
                    depth,
                    &rels,
                    &kinds,
                ),
                GraphCommand::Export {
                    ws,
                    global,
                    format,
                    out,
                } => commands::graph::export(&cfg, ws.as_deref(), global, &format, &out),
                GraphCommand::Awareness {
                    ws,
                    tick_ms,
                    max_steps,
                } => {
                    let ws = ws.unwrap_or_else(|| cfg.ws_default.clone());
                    commands::graph::awareness(&cfg, &ws, tick_ms, max_steps)
                }
                GraphCommand::Activate {
                    ws,
                    global,
                    seeds,
                    topk,
                    alpha,
                    epsilon,
                    no_trace,
                } => commands::graph::activate(
                    &cfg,
                    ws.as_deref(),
                    global,
                    &seeds,
                    topk,
                    alpha,
                    epsilon,
                    no_trace,
                ),
                GraphCommand::Trace { command } => match command {
                    GraphTraceCommand::Show { run_id } => commands::graph::trace_show(&run_id),
                },
            }
        }
        Command::Daemon(args) => {
            let cfg = config::load_config(&overrides_from(&args.common))?;
            let ws = args
                .common
                .ws
                .clone()
                .unwrap_or_else(|| cfg.ws_default.clone());
            crate::control::daemon::run_daemon(&cfg, &ws)
        }
    }
}
