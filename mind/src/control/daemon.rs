use crate::control::{events::EventBus, providers, workspace};
use crate::memory::graph::bridge;
use crate::interface::paths;
use crate::rpc::protocol::{AliveStatus, ProviderInfo, Request, Response, SanityStatus};
use crate::rpc::uds_server;
use crate::interface::config::RuntimeConfig;
use crate::interface::proc::{is_pid_alive, send_signal};
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use std::fs;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use tokio::io::BufReader;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::watch;
use tokio::time::{self, Duration};
use serde_json::json;

fn emit_provider_transition(
    bus: &EventBus,
    ws: &str,
    event_kind: &str,
    transition: &providers::TrustTransition,
) {
    let evt = bus.emit(
        event_kind,
        json!({
            "ws": ws,
            "provider_id": transition.provider.id,
            "endpoint": transition.provider.endpoint,
            "model": transition.provider.model,
            "previous_state": transition.from_state.as_ref().map(|s| format!("{:?}", s).to_lowercase()),
            "new_state": format!("{:?}", transition.to_state).to_lowercase(),
            "trust_snapshot_hash": transition.trust_snapshot_hash,
        }),
    );
    let _ = providers::record_audit_event(&transition.provider.id, &evt.event_id);
}

pub fn ensure_daemon(cfg: &RuntimeConfig, ws: &str) -> Result<PathBuf> {
    let sock = workspace::control_socket_path(&cfg.run_dir, ws);
    let lock = workspace::lock_path(&cfg.run_dir, ws);
    let pid_path = workspace::daemon_pid_path(&cfg.run_dir, ws);

    let pid = fs::read_to_string(&pid_path)
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok());

    if let Some(pid) = pid {
        if is_pid_alive(pid) {
            // Daemon alive; wait briefly for socket to appear.
            for _ in 0..10 {
                if sock.exists() {
                    if std::os::unix::net::UnixStream::connect(&sock).is_ok() {
                        return Ok(sock);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            // Daemon appears wedged: terminate and recover.
            let _ = send_signal(pid, "-TERM");
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
        // stale pidfile
        let _ = fs::remove_file(&pid_path);
    }

    if lock.exists() {
        let _ = fs::remove_file(&lock);
    }

    if sock.exists() {
        if std::os::unix::net::UnixStream::connect(&sock).is_ok() {
            return Ok(sock);
        }
        let _ = std::fs::remove_file(&sock);
    }

    let exe = std::env::current_exe().context("resolve current executable")?;
    let mut cmd = std::process::Command::new(exe);
    cmd.arg("daemon").arg("--ws").arg(ws);
    #[cfg(unix)]
    {
        unsafe {
            cmd.pre_exec(|| {
                libc::setsid();
                Ok(())
            });
        }
    }
    cmd.spawn().context("spawn yai daemon")?;

    for _ in 0..30 {
        if sock.exists() {
            if std::os::unix::net::UnixStream::connect(&sock).is_ok() {
                return Ok(sock);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    anyhow::bail!("daemon not ready for ws {}", ws);
}

pub fn run_daemon(cfg: &RuntimeConfig, ws: &str) -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("build tokio runtime")?;
    let cfg = cfg.clone();
    let ws = ws.to_string();
    rt.block_on(async move { run_daemon_async(cfg, ws).await })
}

async fn run_daemon_async(cfg: RuntimeConfig, ws: String) -> Result<()> {
    let ws_dir = workspace::ensure_ws_dir(&cfg.run_dir, &ws)?;
    let lock_path = workspace::acquire_lock(&cfg.run_dir, &ws)?;
    let pid_path = workspace::daemon_pid_path(&cfg.run_dir, &ws);
    std::fs::write(&pid_path, std::process::id().to_string())
        .with_context(|| format!("write daemon pid: {}", pid_path.display()))?;

    let sock_path = workspace::control_socket_path(&cfg.run_dir, &ws);
    if sock_path.exists() {
        let _ = std::fs::remove_file(&sock_path);
    }

    let listener = UnixListener::bind(&sock_path)
        .with_context(|| format!("bind control socket: {}", sock_path.display()))?;

    let cfg = Arc::new(cfg);
    let ws = Arc::new(ws);
    let bus = Arc::new(EventBus::new(cfg.run_dir.clone(), ws.as_ref().to_string()));
    bus.emit("daemon_started", json!({ "ws": ws.as_ref() }));

    let bus_clone = bus.clone();
    let cfg_clone = cfg.clone();
    let ws_clone = ws.clone();
    tokio::spawn(async move {
        monitor_processes(cfg_clone, ws_clone, bus_clone).await;
    });
    let bus_clone = bus.clone();
    let cfg_clone = cfg.clone();
    let ws_clone = ws.clone();
    tokio::spawn(async move {
        monitor_engine_cortex_events(cfg_clone, ws_clone, bus_clone).await;
    });
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    loop {
        tokio::select! {
            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    break;
                }
            }
            accept = listener.accept() => {
                let (stream, _) = accept.context("accept control socket")?;
                let cfg = cfg.clone();
                let ws = ws.clone();
                let bus = bus.clone();
                let shutdown_tx = shutdown_tx.clone();
                tokio::spawn(async move {
                    if let Err(err) = handle_client(stream, cfg, ws, bus, shutdown_tx).await {
                        eprintln!("daemon client error: {}", err);
                    }
                });
            }
        }
    }

    let _ = std::fs::remove_file(&sock_path);
    workspace::release_lock(&lock_path);
    let _ = std::fs::remove_file(&pid_path);
    let _ = std::fs::remove_dir(&ws_dir);
    Ok(())
}

async fn handle_client(
    stream: UnixStream,
    cfg: Arc<RuntimeConfig>,
    ws: Arc<String>,
    bus: Arc<EventBus>,
    shutdown_tx: watch::Sender<bool>,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let req = match uds_server::read_request(&mut reader).await {
        Ok(req) => req,
        Err(_) => return Ok(()),
    };
    let resp = match req {
        Request::Ping => Response::Pong,
        Request::Status => build_status(&cfg, &ws),
        Request::EventsSubscribe => {
            let mut rx = bus.subscribe();
            uds_server::write_response(&mut writer, &Response::EventsStarted).await?;
            loop {
                match rx.recv().await {
                    Ok(event) => {
                        let _ = uds_server::write_response(&mut writer, &Response::Event { event }).await;
                    }
                    Err(_) => break,
                }
            }
            return Ok(());
        }
        Request::ProvidersDiscover { endpoint, model } => match providers::discover(endpoint, model) {
            Ok(transitions) => {
                let mut items = Vec::new();
                for t in &transitions {
                    emit_provider_transition(&bus, ws.as_ref(), "provider_discovered", t);
                    let _ = providers::sync_graph(&ws, &t.provider);
                    items.push(t.provider.clone());
                }
                Response::Providers { items }
            }
            Err(err) => Response::Error {
                message: err.to_string(),
            },
        },
        Request::ProvidersList => match providers::list_all() {
            Ok(items) => Response::Providers { items },
            Err(err) => Response::Error {
                message: err.to_string(),
            },
        },
        Request::ProvidersPair { id, endpoint, model } => {
            let info = ProviderInfo {
                id,
                endpoint,
                model,
                trust_state: crate::rpc::protocol::TrustState::Paired,
                fingerprint: None,
                capabilities: Vec::new(),
                last_seen: 0,
                attached_ws: None,
            };
            match providers::pair(info) {
                Ok(t) => {
                    emit_provider_transition(&bus, ws.as_ref(), "provider_paired", &t);
                    let _ = providers::sync_graph(&ws, &t.provider);
                    Response::ProvidersOk
                }
                Err(err) => Response::Error {
                    message: err.to_string(),
                },
            }
        }
        Request::ProvidersAttach { id, model } => {
            match providers::get(&id) {
                Ok(Some(mut info)) => {
                    if let Some(m) = model {
                        info.model = m;
                    }
                    match providers::attach(&cfg.run_dir, &ws, info) {
                        Ok(t) => {
                            emit_provider_transition(&bus, ws.as_ref(), "provider_attached", &t);
                            Response::ProvidersOk
                        }
                        Err(err) => Response::Error {
                            message: err.to_string(),
                        },
                    }
                }
                Ok(None) => Response::Error {
                    message: "provider not found (pair first)".to_string(),
                },
                Err(err) => Response::Error {
                    message: err.to_string(),
                },
            }
        }
        Request::ProvidersDetach => match providers::detach(&cfg.run_dir, &ws) {
            Ok(Some(t)) => {
                emit_provider_transition(&bus, ws.as_ref(), "provider_detached", &t);
                Response::ProvidersOk
            }
            Ok(None) => Response::ProvidersOk,
            Err(err) => Response::Error {
                message: err.to_string(),
            },
        },
        Request::ProvidersStatus => match providers::status(&cfg.run_dir, &ws) {
            Ok(active) => Response::ProviderStatus { active },
            Err(err) => Response::Error {
                message: err.to_string(),
            },
        },
        Request::ProvidersRevoke { id } => match providers::revoke(&id) {
            Ok(Some(t)) => {
                emit_provider_transition(&bus, ws.as_ref(), "provider_revoked", &t);
                if let Ok(Some(p)) = providers::get(&id) {
                    let _ = providers::sync_graph(&ws, &p);
                }
                Response::ProvidersOk
            }
            Ok(None) => Response::ProvidersOk,
            Err(err) => Response::Error {
                message: err.to_string(),
            },
        },
        Request::Up {
            build,
            no_engine,
            no_mind,
            ai,
            timeout_ms,
        } => {
            let cfg = cfg.clone();
            let ws_name = ws.to_string();
            let ws_for_task = ws_name.clone();
            let ws_for_start = ws_name.clone();
            let opts = workspace::StartOpts {
                build,
                no_engine,
                no_mind,
                ai,
                timeout_ms: timeout_ms.unwrap_or(5000),
            };
            let bus_clone = bus.clone();
            tokio::task::spawn_blocking(move || {
                bus_clone.emit("ws_up_started", json!({ "ws": ws_for_task }));
                workspace::start_stack(&cfg, &ws_for_start, &opts, &bus_clone)
            })
                .await
                .context("spawn start")??;
            bus.emit("ws_up_complete", json!({ "ws": ws_name }));
            Response::UpOk
        }
        Request::Down { force, shutdown } => {
            let cfg = cfg.clone();
            let ws = ws.to_string();
            let bus_clone = bus.clone();
            let stopping = workspace::stopping_path(&cfg.run_dir, &ws);
            let _ = std::fs::write(&stopping, "1");
            workspace::clear_halt(&cfg.run_dir, &ws);
            tokio::task::spawn_blocking(move || {
                workspace::stop_stack(&cfg, &ws, force, &bus_clone)
            })
                .await
                .context("spawn down")??;
            if shutdown {
                let _ = shutdown_tx.send(true);
            }
            Response::DownOk { shutdown }
        }
    };

    uds_server::write_response(&mut writer, &resp).await?;
    Ok(())
}

async fn monitor_processes(cfg: Arc<RuntimeConfig>, ws: Arc<String>, bus: Arc<EventBus>) {
    let mut interval = time::interval(Duration::from_millis(500));
    let mut last = AliveStatus::default();
    loop {
        interval.tick().await;
        let state = workspace::read_state(&cfg, &ws);
        if state.is_none() {
            continue;
        }
        let st = state.unwrap();
        let socket_path = paths::ws_socket_path(&cfg.socket_path, &ws);
        let mut alive = AliveStatus::default();
        if let Some(pid) = st.boot_pid { alive.boot = is_pid_alive(pid); }
        if let Some(pid) = st.kernel_pid { alive.kernel = is_pid_alive(pid); }
        if let Some(pid) = st.engine_pid { alive.engine = is_pid_alive(pid); }
        if let Some(pid) = st.mind_pid { alive.mind = is_pid_alive(pid); }
        let runtime_sock_exists = std::path::Path::new(&st.socket_path).exists();

        if last.boot && !alive.boot {
            bus.emit("proc_exit", json!({ "proc": "boot" }));
        }
        if last.kernel && !alive.kernel {
            bus.emit("proc_exit", json!({ "proc": "kernel" }));
            let cfg_clone = cfg.clone();
            let ws_clone = ws.to_string();
            let bus_clone = bus.clone();
            let sock = socket_path.clone();
            tokio::task::spawn_blocking(move || {
                bus_clone.emit("kernel_dead", json!({ "ws": ws_clone, "reason": "kernel_dead" }));
                workspace::write_halt(&cfg_clone.run_dir, &ws_clone, "kernel_dead");
                if sock.exists() {
                    let _ = std::fs::remove_file(&sock);
                }
                let stopping = workspace::stopping_path(&cfg_clone.run_dir, &ws_clone);
                if stopping.exists() {
                    return;
                }
                workspace::stop_stack(&cfg_clone, &ws_clone, true, &bus_clone).ok();
                let _ = bridge::ingest_events_to_semantic(&ws_clone);
            });
        }
        if !alive.kernel && st.kernel_pid.is_some() {
            let stopping = workspace::stopping_path(&cfg.run_dir, &ws);
            let halted = workspace::read_halt(&cfg.run_dir, &ws);
            if !stopping.exists() && halted.is_none() {
                let cfg_clone = cfg.clone();
                let ws_clone = ws.to_string();
                let bus_clone = bus.clone();
                let sock = socket_path.clone();
                tokio::task::spawn_blocking(move || {
                    bus_clone.emit("kernel_dead", json!({ "ws": ws_clone, "reason": "kernel_dead" }));
                    workspace::write_halt(&cfg_clone.run_dir, &ws_clone, "kernel_dead");
                    if sock.exists() {
                        let _ = std::fs::remove_file(&sock);
                    }
                    let stopping = workspace::stopping_path(&cfg_clone.run_dir, &ws_clone);
                    if stopping.exists() {
                        return;
                    }
                    workspace::stop_stack(&cfg_clone, &ws_clone, true, &bus_clone).ok();
                    let _ = bridge::ingest_events_to_semantic(&ws_clone);
                });
            }
        }
        if last.kernel && alive.kernel && !runtime_sock_exists {
            let cfg_clone = cfg.clone();
            let ws_clone = ws.to_string();
            let bus_clone = bus.clone();
            let sock = socket_path.clone();
            tokio::task::spawn_blocking(move || {
                bus_clone.emit("kernel_dead", json!({ "ws": ws_clone, "reason": "runtime_sock_missing" }));
                workspace::write_halt(&cfg_clone.run_dir, &ws_clone, "runtime_sock_missing");
                if sock.exists() {
                    let _ = std::fs::remove_file(&sock);
                }
                let stopping = workspace::stopping_path(&cfg_clone.run_dir, &ws_clone);
                if stopping.exists() {
                    return;
                }
                workspace::stop_stack(&cfg_clone, &ws_clone, true, &bus_clone).ok();
                let _ = bridge::ingest_events_to_semantic(&ws_clone);
            });
        }
        if last.engine && !alive.engine {
            bus.emit("proc_exit", json!({ "proc": "engine" }));
        }
        if last.mind && !alive.mind {
            bus.emit("proc_exit", json!({ "proc": "mind" }));
        }

        if alive.boot != last.boot || alive.kernel != last.kernel || alive.engine != last.engine || alive.mind != last.mind {
            bus.emit("status_changed", json!({
                "kernel": alive.kernel,
                "engine": alive.engine,
                "mind": alive.mind,
                "boot": alive.boot
            }));
        }
        last = alive;
    }
}

async fn monitor_engine_cortex_events(cfg: Arc<RuntimeConfig>, ws: Arc<String>, bus: Arc<EventBus>) {
    let mut interval = time::interval(Duration::from_millis(250));
    let mut offset: usize = 0;
    loop {
        interval.tick().await;
        let path = cfg.run_dir.join(ws.as_ref()).join("engine.log");
        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };
        if offset > content.len() {
            offset = 0;
        }
        if content.len() == offset {
            continue;
        }
        let new_part = &content[offset..];
        for line in new_part.lines() {
            let line = line.trim();
            let Some(json_payload) = line.strip_prefix("[YAI_CORTEX_EVENT] ") else {
                continue;
            };
            let Ok(mut value) = serde_json::from_str::<serde_json::Value>(json_payload) else {
                continue;
            };
            let kind = value
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if kind.is_empty() {
                continue;
            }
            if let Some(obj) = value.as_object_mut() {
                obj.remove("type");
            }
            bus.emit(&kind, value);
        }
        offset = content.len();
    }
}

fn build_status(cfg: &RuntimeConfig, ws: &str) -> Response {
    let state = workspace::read_state(cfg, ws);
    let mut alive = AliveStatus::default();
    let mut runtime_sock_exists = false;
    let control_sock_exists = workspace::control_socket_path(&cfg.run_dir, ws).exists();
    if let Some(st) = &state {
        if let Some(pid) = st.boot_pid {
            alive.boot = is_pid_alive(pid);
        }
        if let Some(pid) = st.kernel_pid {
            alive.kernel = is_pid_alive(pid);
        }
        if let Some(pid) = st.engine_pid {
            alive.engine = is_pid_alive(pid);
        }
        if let Some(pid) = st.mind_pid {
            alive.mind = is_pid_alive(pid);
        }
        runtime_sock_exists = std::path::Path::new(&st.socket_path).exists();
    }
    let halt_reason = workspace::read_halt(&cfg.run_dir, ws);
    let sanity = SanityStatus {
        runtime_sock_exists,
        control_sock_exists,
    };
    Response::Status {
        state,
        alive,
        daemon_pid: std::process::id(),
        sanity,
        halt_reason,
    }
}
