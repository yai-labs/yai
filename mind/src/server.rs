// src/server.rs
#![allow(dead_code)]
use crate::models::{IceMessage, MessageType};
use crate::core::governance::GovernanceEngine;
use crate::bridge::shm::VaultBridge;
use crate::core::runtime::{run_turn, RuntimeContext};
use crate::core::protocol::CommandId;
use crate::interface::config::{load_config, CliOverrides};
use crate::llm::adapter::build_llm_for_ws;
use crate::bridge::vault::VaultBridge as EngineVault;
use crate::core::state::GlobalState;
use crate::memory::{MemoryCore, SqliteMemoryStore};
use crate::shared::constants::DEFAULT_KNOWLEDGE_DB_PATH;

use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::env;
use std::path::PathBuf;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

pub struct IceStudioServer {
    host: String,
    port: u16,
    state: GlobalState,
    governance: Arc<GovernanceEngine>,
}

fn default_artifacts_root() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".yai").join("artifacts")
}

fn yai_api_build() -> String {
    if let Ok(val) = env::var("YAI_API_BUILD") {
        if !val.is_empty() {
            return val;
        }
    }

    let artifacts_root = env::var("YAI_ARTIFACTS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| default_artifacts_root());
    let manifest = artifacts_root.join("yai-core").join("dist").join("MANIFEST.json");
    if let Ok(data) = std::fs::read_to_string(manifest) {
        if let Ok(v) = serde_json::from_str::<Value>(&data) {
            let git_sha = v.get("git_sha").and_then(|v| v.as_str()).unwrap_or("unknown");
            let build_time = v.get("build_time").and_then(|v| v.as_str()).unwrap_or("unknown");
            return format!("{} {}", git_sha, build_time);
        }
    }

    "unknown".to_string()
}

impl IceStudioServer {
    pub fn new(host: &str, port: u16, state: GlobalState) -> Self {
        Self {
            host: host.to_string(),
            port,
            state,
            governance: Arc::new(GovernanceEngine::new()),
        }
    }

    pub async fn start(&self) -> tokio::io::Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;
        println!("🚀 YAI-MIND Rust attiva su ws://{}", addr);

        let workspace_id = env::var("YAI_WORKSPACE_ID").unwrap_or_else(|_| "arch_dev_session".to_string());
        let shm_name = format!("/yai_vault_{}", workspace_id);

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream.peer_addr().expect("connected streams should have a peer address");
            let governance = self.governance.clone();
            let shm_name_clone = shm_name.clone();
            tokio::spawn(Self::handle_connection(stream, peer, governance, shm_name_clone));
        }

        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        _addr: std::net::SocketAddr,
        governance: Arc<GovernanceEngine>,
        shm_name: String,
    ) {
        let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let (out_tx, mut out_rx) = mpsc::unbounded_channel::<Message>();
        tokio::spawn(async move {
            while let Some(msg) = out_rx.recv().await {
                if ws_sender.send(msg).await.is_err() {
                    break;
                }
            }
        });

        // Heartbeat per-connessione
        let hb_shm = shm_name.clone();
        let hb_tx = out_tx.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(250));
            loop {
                interval.tick().await;
                if let Ok(bridge) = VaultBridge::new(&hb_shm) {
                    let vault = bridge.read_live();
                    let energy_quota = vault.energy_quota as f64;
                    let energy_used = vault.energy_consumed as f64;
                    let percentage = if energy_quota > 0.0 {
                        (energy_used / energy_quota) * 100.0
                    } else {
                        0.0
                    };

                    let mut payload = HashMap::new();
                    payload.insert("status".to_string(), json!(vault.status));
                    payload.insert(
                        "energy".to_string(),
                        json!({
                            "quota": vault.energy_quota,
                            "used": vault.energy_consumed,
                            "percentage": percentage
                        }),
                    );
                    payload.insert("vault".to_string(), json!(vault.vault_name));

                    let msg = IceMessage::new(MessageType::Heartbeat, "2.0.0-lite", payload);
                    let _ = hb_tx.send(Message::Text(msg.to_json()));
                }
            }
        });

        while let Some(msg) = ws_receiver.next().await {
            if let Ok(msg) = msg {
                if msg.is_text() {
                    let raw_text = msg.to_text().unwrap();
                    let incoming: IceMessage = match serde_json::from_str(raw_text) {
                        Ok(m) => m,
                        Err(_) => continue,
                    };

                    match incoming.r#type {
                        MessageType::Handshake => {
                            let payload = build_manifest();
                            let reply = IceMessage::new(MessageType::HandshakeAck, "2.0.0-lite", payload);
                            let _ = out_tx.send(Message::Text(reply.to_json()));
                        },
                        MessageType::Intent => {
                            // 1. Check Governance (I-003)
                            let verdict = governance.validate_compliance(&incoming);
                            if !verdict.is_valid {
                                let mut payload = HashMap::new();
                                payload.insert("message".to_string(), json!(verdict.reason));
                                let reply = IceMessage::new(MessageType::Response, "2.0.0-lite", payload);
                                let _ = out_tx.send(Message::Text(reply.to_json()));
                                continue;
                            }
                            // 2. Brain Loop (Rust-native)
                            let cmd = incoming
                                .payload
                                .get("command")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");

                            let workspace_id = env::var("YAI_WORKSPACE_ID")
                                .unwrap_or_else(|_| "arch_dev_session".to_string());
                            let db_path = env::var("YAI_KNOWLEDGE_DB")
                                .unwrap_or_else(|_| DEFAULT_KNOWLEDGE_DB_PATH.to_string());
                            let vault = EngineVault::attach(&workspace_id);
                            let result = match vault {
                                Ok(v) => {
                                    let scheduler = crate::core::scheduler::Scheduler::new(v);
                                    let memory = MemoryCore::new(Box::new(SqliteMemoryStore::new(db_path)));
                                    let cfg = load_config(&CliOverrides::default())
                                        .unwrap_or_else(|_| panic!("failed to load config"));
                                    let mut ctx = RuntimeContext {
                                        scheduler,
                                        llm: build_llm_for_ws(&cfg, &workspace_id),
                                        memory,
                                        trace_id: "ws-turn".to_string(),
                                        workspace_id,
                                    };
                                    run_turn(cmd, &mut ctx).map_err(|e| format!("{:?}", e))
                                }
                                Err(e) => Err(e),
                            };

                            match result {
                                Ok(turn) => {
                                    let mut payload = HashMap::new();
                                    let response_text = if let Some(llm) = &turn.llm_response {
                                        llm.to_string()
                                    } else if !turn.agent_output.response_text.is_empty() {
                                        if turn.decision.command == CommandId::Ping {
                                            format!("{}\nEngine: {}", turn.agent_output.response_text, turn.execution.response)
                                        } else {
                                            turn.agent_output.response_text.clone()
                                        }
                                    } else if turn.decision.command == CommandId::Ping {
                                        format!("Engine: {}", turn.execution.response)
                                    } else {
                                        "OK".to_string()
                                    };
                                    payload.insert("message".to_string(), json!(response_text));
                                    let reply = IceMessage::new(MessageType::Response, "2.0.0-lite", payload);
                                    let _ = out_tx.send(Message::Text(reply.to_json()));
                                }
                                Err(err) => {
                                    let mut payload = HashMap::new();
                                    payload.insert("message".to_string(), json!(format!("YAI-MIND error: {}", err)));
                                    let reply = IceMessage::new(MessageType::Response, "2.0.0-lite", payload);
                                    let _ = out_tx.send(Message::Text(reply.to_json()));
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

fn build_manifest() -> HashMap<String, Value> {
    let mut payload = HashMap::new();
    payload.insert("system".to_string(), json!("ONLINE"));
    payload.insert("vault".to_string(), json!(env::var("YAI_WORKSPACE_ID").unwrap_or_else(|_| "arch_dev_session".to_string())));
    payload.insert("kernel_version".to_string(), json!("unknown"));
    payload.insert("vault_address".to_string(), json!("unknown"));

    let agents_env = env::var("YAI_AI_AGENTS").unwrap_or_default();
    let agents: Vec<String> = agents_env
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    payload.insert(
        "energy".to_string(),
        json!({
            "quota": 0,
            "used": 0
        }),
    );
    payload.insert(
        "ai".to_string(),
        json!({
            "agents": agents,
            "model": env::var("YAI_REMOTE_MODEL").unwrap_or_else(|_| "unknown".to_string()),
            "endpoint": env::var("YAI_REMOTE_ENDPOINT").unwrap_or_else(|_| "unknown".to_string())
        }),
    );

    payload.insert("build".to_string(), json!(yai_api_build()));

    payload
}
