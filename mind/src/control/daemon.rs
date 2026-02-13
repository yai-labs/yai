use crate::cli::config::RuntimeConfig;
use crate::cli::paths;
use crate::cli::proc::{is_pid_alive, now_epoch};
use crate::cognition::memory::graph::bridge;
use crate::control::{chat, dsar, events::EventBus, providers, shell, workspace};
use crate::transport::rpc::protocol::{
    AliveStatus, ChatMessage, ChatRole, ChatSession, ComplianceContext, DsarStatus, ProviderInfo,
    Request, Response, SanityStatus,
};
use crate::transport::rpc::uds_server;
use anyhow::{Context, Result};
use serde_json::json;
use std::fs;
#[cfg(unix)]
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::io::BufReader;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{watch, Mutex};
use tokio::time::{self, Duration};

#[derive(Clone)]
struct AppServices {
    chat: Arc<chat::ChatEngine>,
    shell: Arc<shell::ShellService>,
    last_status_snapshot: Arc<Mutex<Option<String>>>,
}

fn daemon_log(ws: &str, message: &str) {
    eprintln!("[yai-daemon ws={ws}] {message}");
    if let Some(run_dir) = DAEMON_RUN_DIR.get() {
        let path = run_dir.join(ws).join("daemon.log");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            use std::io::Write;
            let _ = writeln!(file, "[yai-daemon ws={ws}] {message}");
        }
    }
}

static DAEMON_RUN_DIR: OnceLock<std::path::PathBuf> = OnceLock::new();

fn request_name(req: &Request) -> &'static str {
    match req {
        Request::ProtocolHandshake { .. } => "protocol.handshake",
        Request::Ping => "ping",
        Request::Status => "status",
        Request::Up { .. } => "up",
        Request::Down { .. } => "down",
        Request::ProvidersDiscover { .. } => "providers.discover",
        Request::ProvidersList => "providers.list",
        Request::ProvidersPair { .. } => "providers.pair",
        Request::ProvidersAttach { .. } => "providers.attach",
        Request::ProvidersDetach => "providers.detach",
        Request::ProvidersRevoke { .. } => "providers.revoke",
        Request:: => "providers.status",
        Request::DsarRequest { .. } => "dsar.request",
        Request::DsarStatus { .. } => "dsar.status",
        Request::DsarExecute { .. } => "dsar.execute",
        Request::ChatSessionsList => "chat.sessions.list",
        Request::ChatSessionNew { .. } => "chat.session.new",
        Request::ChatSessionSelect { .. } => "chat.session.select",
        Request::ChatHistory { .. } => "chat.history",
        Request::ChatSend { .. } => "chat.send",
        Request::ShellExec { .. } => "shell.exec",
        Request::EventsSubscribe => "events.subscribe",
    }
}

fn requires_arming(req: &Request) -> bool {
    match req {
        // UNARMED se endpoint/model sono vuoti (None o Some(""))
        // ARMED se endpoint/model hanno contenuto (effetto esterno / scan remoto)
        Request::ProvidersDiscover { endpoint, model } => {
            let ep = endpoint.as_deref().unwrap_or("");
            let mo = model.as_deref().unwrap_or("");
            !(ep.is_empty() && mo.is_empty())
        }

        // Mutazioni / effetti esterni
        Request::Up { .. }
        | Request::Down { .. }
        | Request::ShellExec { .. }
        | Request::ProvidersPair { .. }
        | Request::ProvidersAttach { .. }
        | Request::ProvidersDetach
        | Request::ProvidersRevoke { .. } => true,

        // Read-only
        _ => false,
    }
}

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
    if let Ok(evt) = evt {
        let _ = providers::record_audit_event(&transition.provider.id, &evt.event_id);
    }
}

fn dsar_compliance() -> ComplianceContext {
    ComplianceContext {
        pack_ref: "gdpr-eu/2026Q1".to_string(),
        purpose_id: "LEGAL_OBLIGATION".to_string(),
        data_class: "PERSONAL".to_string(),
        retention_policy_id: "default".to_string(),
        legal_basis: "LEGAL_OBLIGATION".to_string(),
        subject_scope: "identified".to_string(),
        processor_role: "controller".to_string(),
        audit_required: true,
    }
}

pub fn ensure_daemon(cfg: &RuntimeConfig, ws: &str) -> Result<PathBuf> {
    let sock = workspace::control_socket_path(&cfg.run_dir, ws);
    let lock = workspace::daemon_lock_path(&cfg.run_dir, ws);
    let pid_path = workspace::daemon_pid_path(&cfg.run_dir, ws);
    let wait_iters = 150u32;
    let wait_ms = 100u64;
    let lock_pid = fs::read_to_string(&lock)
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok());

    let pid = fs::read_to_string(&pid_path)
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok());

    if let Some(pid) = pid {
        if is_pid_alive(pid) {
            // Daemon alive; wait for socket to appear.
            for _ in 0..wait_iters {
                if sock.exists() {
                    if std::os::unix::net::UnixStream::connect(&sock).is_ok() {
                        return Ok(sock);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(wait_ms));
            }
            anyhow::bail!(
                "daemon starting for ws {} (pid {}), control socket not ready",
                ws,
                pid
            );
        }
        // stale pidfile
        let _ = fs::remove_file(&pid_path);
    }

    if let Some(pid) = lock_pid {
        if is_pid_alive(pid) {
            for _ in 0..wait_iters {
                if sock.exists() {
                    if std::os::unix::net::UnixStream::connect(&sock).is_ok() {
                        return Ok(sock);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(wait_ms));
            }
            anyhow::bail!(
                "daemon starting for ws {} (pid {}), control socket not ready",
                ws,
                pid
            );
        }
    }

    if lock.exists() {
        let _ = fs::remove_file(&lock);
    }

    if sock.exists() {
        if std::os::unix::net::UnixStream::connect(&sock).is_ok() {
            return Ok(sock);
        }
    }

    anyhow::bail!(
        "daemon not running for ws {} (start with: yai mind --ws {})",
        ws,
        ws
    );
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
    let _ = DAEMON_RUN_DIR.set(cfg.run_dir.clone());
    let ws_dir = workspace::ensure_ws_dir(&cfg.run_dir, &ws)?;
    let runtime_sock = paths::ws_socket_path(&cfg.socket_path, &ws);
    if !runtime_sock.exists() {
        anyhow::bail!(
            "runtime socket not found for ws {} (start runtime first: yai up --ws {})",
            ws,
            ws
        );
    }
    if UnixStream::connect(&runtime_sock).await.is_err() {
        anyhow::bail!(
            "runtime not reachable for ws {} (start runtime first: yai up --ws {})",
            ws,
            ws
        );
    }
    let lock_path = workspace::acquire_daemon_lock(&cfg.run_dir, &ws)?;
    let pid_path = workspace::daemon_pid_path(&cfg.run_dir, &ws);
    std::fs::write(&pid_path, std::process::id().to_string())
        .with_context(|| format!("write daemon pid: {}", pid_path.display()))?;

    let sock_path = workspace::control_socket_path(&cfg.run_dir, &ws);
    if sock_path.exists() {
        let _ = std::fs::remove_file(&sock_path);
    }

    let listener = UnixListener::bind(&sock_path)
        .with_context(|| format!("bind control socket: {}", sock_path.display()))?;
    daemon_log(&ws, &format!("listening on {}", sock_path.display()));

    let cfg = Arc::new(cfg);
    let ws = Arc::new(ws);
    let services = AppServices {
        chat: Arc::new(chat::ChatEngine::new()),
        shell: Arc::new(shell::ShellService::new()),
        last_status_snapshot: Arc::new(Mutex::new(None)),
    };
    let bus = Arc::new(EventBus::new(cfg.run_dir.clone(), ws.as_ref().to_string()));
    let _ = bus.emit("daemon_started", json!({ "ws": ws.as_ref() }));

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
    let bus_clone = bus.clone();
    let cfg_clone = cfg.clone();
    let ws_clone = ws.clone();
    tokio::spawn(async move {
        monitor_retention(cfg_clone, ws_clone, bus_clone).await;
    });
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    loop {
        tokio::select! {
            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    daemon_log(&ws, "shutdown requested");
                    break;
                }
            }
            accept = listener.accept() => {
                let (stream, _) = accept.context("accept control socket")?;
                let cfg = cfg.clone();
                let ws = ws.clone();
                let bus = bus.clone();
                let services = services.clone();
                let shutdown_tx = shutdown_tx.clone();
                tokio::spawn(async move {
                    if let Err(err) = handle_client(stream, cfg, ws, bus, services, shutdown_tx).await {
                        eprintln!("[yai-daemon] client error: {}", err);
                    }
                });
            }
        }
    }

    let _ = std::fs::remove_file(&sock_path);
    workspace::release_lock(&lock_path);
    let _ = std::fs::remove_file(&pid_path);
    let _ = std::fs::remove_dir(&ws_dir);
    daemon_log(&ws, "stopped");
    Ok(())
}

fn normalize_opt_string(v: Option<String>) -> Option<String> {
    v.and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() {
            None
        } else {
            Some(t)
        }
    })
}

async fn handle_client(
    stream: UnixStream,
    cfg: Arc<RuntimeConfig>,
    ws: Arc<String>,
    bus: Arc<EventBus>,
    services: AppServices,
    shutdown_tx: watch::Sender<bool>,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    loop {
        let inbound = match uds_server::read_request(&mut reader).await {
            Ok(req) => req,
            Err(err) => {
                // EOF: client ha chiuso la connessione -> fine pulita
                if err.to_string().contains("rpc eof") {
                    return Ok(());
                }
                // Errore di parse o altro: rispondi con error e chiudi
                let _ = uds_server::write_response(
                    &mut writer,
                    &Response::Error {
                        message: err.to_string(),
                    },
                )
                .await;
                return Ok(());
            }
        };

        let req = inbound.request;
        let is_status = matches!(req, Request::Status);

        if !is_status {
            daemon_log(ws.as_ref(), &format!("request {}", request_name(&req)));
        }

        // ws_id check
        if let Some(ws_id) = inbound.ws_id.as_deref() {
            if ws_id != ws.as_ref() {
                uds_server::write_response(
                    &mut writer,
                    &Response::Error {
                        message: "ws_mismatch".to_string(),
                    },
                )
                .await?;
                continue; // non chiudiamo la connessione, rispondiamo e continuiamo
            }
        }

        // arming gate
        if requires_arming(&req) {
            if !inbound.arming {
                uds_server::write_response(
                    &mut writer,
                    &Response::Error {
                        message: "arming_required".to_string(),
                    },
                )
                .await?;
                continue;
            }
            if inbound.role.as_deref() != Some("operator") {
                uds_server::write_response(
                    &mut writer,
                    &Response::Error {
                        message: "role_required".to_string(),
                    },
                )
                .await?;
                continue;
            }
        }

        // Se Down con shutdown=true, dopo la response chiudiamo la connessione
        let mut close_after = false;

        let resp = match req {
            Request::ProtocolHandshake { .. } => Response::ProtocolHandshake {
                protocol_version: crate::transport::rpc::protocol::RPC_PROTOCOL_VERSION,
                server_version: env!("CARGO_PKG_VERSION").to_string(),
            },

            Request::Ping => Response::Pong,

            Request::Status => build_status(&cfg, &ws),

            Request::ChatSessionsList => {
                let items = services
                    .chat
                    .store()
                    .sessions()
                    .into_iter()
                    .map(to_rpc_chat_session)
                    .collect::<Vec<_>>();
                let selected = services.chat.store().selected_session();
                Response::ChatSessions { items, selected }
            }

            Request::ChatSessionNew { title } => {
                let sess = services.chat.store().create_session(title);
                let _ = services.chat.store().select_session(&sess.id);
                Response::ChatSession {
                    session: to_rpc_chat_session(sess),
                }
            }

            Request::ChatSessionSelect { session_id } => {
                match services.chat.store().select_session(&session_id) {
                    Ok(()) => {
                        let selected = services.chat.store().selected_session();
                        let items = services
                            .chat
                            .store()
                            .sessions()
                            .into_iter()
                            .map(to_rpc_chat_session)
                            .collect::<Vec<_>>();
                        Response::ChatSessions { items, selected }
                    }
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }

            Request::ChatHistory { session_id } => {
                let sid = match session_id.or_else(|| services.chat.store().selected_session()) {
                    Some(v) => v,
                    None => {
                        let s = services.chat.store().create_session(None);
                        let _ = services.chat.store().select_session(&s.id);
                        s.id
                    }
                };
                match services.chat.store().history(&sid) {
                    Ok(items) => Response::ChatHistory {
                        session_id: sid,
                        items: items.into_iter().map(to_rpc_chat_message).collect(),
                    },
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }

            Request::ChatSend {
                session_id,
                text,
                stream,
            } => {
                let sid = match session_id.or_else(|| services.chat.store().selected_session()) {
                    Some(v) => v,
                    None => {
                        let s = services.chat.store().create_session(None);
                        let _ = services.chat.store().select_session(&s.id);
                        s.id
                    }
                };
                match services.chat.send_echo(&sid, &text) {
                    Ok(msg) => {
                        if stream {
                            for chunk in msg.content.split_whitespace() {
                                let _ = bus.emit(
                                    "chat.delta",
                                    json!({ "session_id": sid, "delta": format!("{chunk} ") }),
                                );
                            }
                        }
                        let _ = bus.emit(
                            "chat.message",
                            json!({ "session_id": sid, "id": msg.id, "content": msg.content }),
                        );
                        Response::ChatSend {
                            message: to_rpc_chat_message(msg),
                        }
                    }
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }

            Request::ShellExec { cmd, args, cwd } => {
                match services.shell.exec(&cmd, &args, cwd.as_deref()).await {
                    Ok(out) => Response::ShellExec {
                        exit_code: out.exit_code,
                        stdout: out.stdout,
                        stderr: out.stderr,
                    },
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }

            // Questo è “streaming”: questa connessione resta occupata a mandare eventi.
            Request::EventsSubscribe => {
                let mut rx = bus.subscribe();
                uds_server::write_response(&mut writer, &Response::EventsStarted).await?;
                loop {
                    match rx.recv().await {
                        Ok(event) => {
                            let _ =
                                uds_server::write_response(&mut writer, &Response::Event { event })
                                    .await;
                        }
                        Err(_) => break,
                    }
                }
                return Ok(());
            }

            // ✅ ProvidersDiscover: normalizza "" -> None
            Request::ProvidersDiscover { endpoint, model } => {
                let endpoint = normalize_opt_string(endpoint);
                let model = normalize_opt_string(model);

                match providers::discover(endpoint, model) {
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
                }
            }

            Request::ProvidersList => match providers::list_all() {
                Ok(items) => Response::Providers { items },
                Err(err) => Response::Error {
                    message: err.to_string(),
                },
            },

            Request::ProvidersPair {
                id,
                endpoint,
                model,
            } => {
                let info = ProviderInfo {
                    id,
                    endpoint,
                    model,
                    trust_state: crate::transport::rpc::protocol::TrustState::Paired,
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

            Request::ProvidersAttach { id, model } => match providers::get(&id) {
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
            },

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

            Request::DsarRequest {
                request_type,
                subject_ref,
            } => match dsar::create_request(&cfg.run_dir, &ws, &request_type, &subject_ref) {
                Ok(request) => {
                    let event_type = if request_type == "export" {
                        "DATA_EXPORT"
                    } else {
                        "DATA_ERASE"
                    };
                    if let Err(err) = bus.emit_with_compliance(
                        event_type,
                        json!({
                            "ws": ws.as_ref(),
                            "request_id": request.request_id,
                            "subject_ref": request.subject_ref,
                            "request_type": request.request_type,
                            "status": format!("{:?}", request.status).to_lowercase()
                        }),
                        Some(dsar_compliance()),
                    ) {
                        Response::Error {
                            message: err.to_string(),
                        }
                    } else {
                        let _ = bus.emit_with_compliance(
                            "PROCESSING_DECLARED",
                            json!({
                                "ws": ws.as_ref(),
                                "request_id": request.request_id,
                                "subject_ref": request.subject_ref,
                                "request_type": request.request_type,
                            }),
                            Some(dsar_compliance()),
                        );
                        Response::DsarCreated { request }
                    }
                }
                Err(err) => Response::Error {
                    message: err.to_string(),
                },
            },

            Request::DsarStatus { request_id } => {
                match dsar::get_request(&cfg.run_dir, &ws, &request_id) {
                    Ok(request) => Response::DsarState { request },
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }

            Request::DsarExecute { request_id } => {
                match dsar::set_status(&cfg.run_dir, &ws, &request_id, DsarStatus::Executed) {
                    Ok(Some(request)) => {
                        let event_type = if request.request_type == "export" {
                            "DATA_EXPORT"
                        } else {
                            "DATA_ERASE"
                        };
                        if let Err(err) = bus.emit_with_compliance(
                            event_type,
                            json!({
                                "ws": ws.as_ref(),
                                "request_id": request.request_id,
                                "subject_ref": request.subject_ref,
                                "request_type": request.request_type,
                                "status": format!("{:?}", request.status).to_lowercase()
                            }),
                            Some(dsar_compliance()),
                        ) {
                            Response::Error {
                                message: err.to_string(),
                            }
                        } else {
                            Response::DsarExecuted { request }
                        }
                    }
                    Ok(None) => Response::Error {
                        message: "dsar request not found".to_string(),
                    },
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }

            Request::Up {
                build,
                no_engine,
                no_mind: _,
                ai,
                timeout_ms,
            } => {
                let cfg2 = cfg.clone();
                let ws_name = ws.to_string();
                let ws_for_task = ws_name.clone();
                let ws_for_start = ws_name.clone();
                let opts = workspace::StartOpts {
                    build,
                    no_engine,
                    no_mind: true,
                    ai,
                    timeout_ms: timeout_ms.unwrap_or(5000),
                };
                let bus_clone = bus.clone();
                tokio::task::spawn_blocking(move || {
                    let _ = bus_clone.emit("ws_up_started", json!({ "ws": ws_for_task }));
                    workspace::start_stack(&cfg2, &ws_for_start, &opts, &bus_clone)
                })
                .await
                .context("spawn start")??;
                let _ = bus.emit("ws_up_complete", json!({ "ws": ws_name }));
                Response::UpOk
            }

            Request::Down { force, shutdown } => {
                let cfg2 = cfg.clone();
                let ws_name = ws.to_string();
                let bus_clone = bus.clone();
                let stopping = workspace::stopping_path(&cfg2.run_dir, &ws_name);
                let _ = std::fs::write(&stopping, "1");
                workspace::clear_halt(&cfg2.run_dir, &ws_name);

                tokio::task::spawn_blocking(move || {
                    workspace::stop_stack(&cfg2, &ws_name, force, &bus_clone)
                })
                .await
                .context("spawn down")??;

                if shutdown {
                    let _ = shutdown_tx.send(true);
                    close_after = true;
                }

                Response::DownOk { shutdown }
            }
        };

        // status snapshot logging (uguale al tuo, solo dentro loop)
        if is_status {
            let snapshot =
                serde_json::to_string(&resp).unwrap_or_else(|_| "status-encode-error".to_string());
            let mut last = services.last_status_snapshot.lock().await;
            if last.as_ref() != Some(&snapshot) {
                daemon_log(ws.as_ref(), "request status (changed)");
                *last = Some(snapshot);
            }
        }

        uds_server::write_response(&mut writer, &resp).await?;

        if close_after {
            return Ok(());
        }
    }
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
        let runtime_age_secs = now_epoch().saturating_sub(st.started_at_epoch);
        let socket_path = paths::ws_socket_path(&cfg.socket_path, &ws);
        let mut alive = AliveStatus::default();
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
        let runtime_sock_exists = std::path::Path::new(&st.socket_path).exists();

        if last.boot && !alive.boot {
            let _ = bus.emit("proc_exit", json!({ "proc": "boot" }));
        }
        if last.kernel && !alive.kernel {
            let _ = bus.emit("proc_exit", json!({ "proc": "kernel" }));
            let cfg_clone = cfg.clone();
            let ws_clone = ws.to_string();
            let bus_clone = bus.clone();
            let sock = socket_path.clone();
            tokio::task::spawn_blocking(move || {
                let _ = bus_clone.emit(
                    "kernel_dead",
                    json!({ "ws": ws_clone, "reason": "kernel_dead" }),
                );
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
                    let _ = bus_clone.emit(
                        "kernel_dead",
                        json!({ "ws": ws_clone, "reason": "kernel_dead" }),
                    );
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
            if runtime_age_secs < 5 {
                last = alive;
                continue;
            }
            let cfg_clone = cfg.clone();
            let ws_clone = ws.to_string();
            let bus_clone = bus.clone();
            let sock = socket_path.clone();
            tokio::task::spawn_blocking(move || {
                let _ = bus_clone.emit(
                    "kernel_dead",
                    json!({ "ws": ws_clone, "reason": "runtime_sock_missing" }),
                );
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
            let _ = bus.emit("proc_exit", json!({ "proc": "engine" }));
        }
        if last.mind && !alive.mind {
            let _ = bus.emit("proc_exit", json!({ "proc": "mind" }));
        }

        if alive.boot != last.boot
            || alive.kernel != last.kernel
            || alive.engine != last.engine
            || alive.mind != last.mind
        {
            let _ = bus.emit(
                "status_changed",
                json!({
                    "kernel": alive.kernel,
                    "engine": alive.engine,
                    "mind": alive.mind,
                    "boot": alive.boot
                }),
            );
        }
        last = alive;
    }
}

fn latest_event_ts(path: &std::path::Path) -> Option<u64> {
    let raw = std::fs::read_to_string(path).ok()?;
    for line in raw.lines().rev() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if let Some(ts) = v.get("ts").and_then(|v| v.as_u64()) {
            return Some(ts);
        }
    }
    None
}

fn parse_compliance(v: &serde_json::Value) -> Option<ComplianceContext> {
    serde_json::from_value::<ComplianceContext>(v.clone()).ok()
}

async fn monitor_retention(cfg: Arc<RuntimeConfig>, ws: Arc<String>, bus: Arc<EventBus>) {
    let mut interval = time::interval(Duration::from_millis(1000));
    loop {
        interval.tick().await;
        let events_path = cfg.run_dir.join(ws.as_ref()).join("events.log");
        let Some(now_ts) = latest_event_ts(&events_path) else {
            continue;
        };

        let Ok(expired) = bridge::expire_retained(ws.as_ref(), now_ts) else {
            continue;
        };
        for record in expired {
            let compliance = record.compliance.as_ref().and_then(parse_compliance);
            let _ = bus.emit_with_compliance(
                "RETENTION_EXPIRE",
                json!({
                    "ws": ws.as_ref(),
                    "resource_id": record.id,
                    "retention_policy_id": record.retention_policy_id,
                    "expired_at": record.expired_at,
                    "expiry_anchor": "event_ts"
                }),
                compliance,
            );
        }
    }
}

async fn monitor_engine_cortex_events(
    cfg: Arc<RuntimeConfig>,
    ws: Arc<String>,
    bus: Arc<EventBus>,
) {
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
            let _ = bus.emit(&kind, value);
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

fn to_rpc_chat_role(role: &chat::Role) -> ChatRole {
    match role {
        chat::Role::System => ChatRole::System,
        chat::Role::User => ChatRole::User,
        chat::Role::Assistant => ChatRole::Assistant,
        chat::Role::Tool => ChatRole::Tool,
    }
}

fn to_rpc_chat_message(msg: chat::Message) -> ChatMessage {
    ChatMessage {
        id: msg.id,
        ts_ms: msg.ts_ms,
        role: to_rpc_chat_role(&msg.role),
        content: msg.content,
    }
}

fn to_rpc_chat_session(sess: chat::ChatSession) -> ChatSession {
    ChatSession {
        id: sess.id,
        title: sess.title,
        created_ts_ms: sess.created_ts_ms,
        last_ts_ms: sess.last_ts_ms,
    }
}
