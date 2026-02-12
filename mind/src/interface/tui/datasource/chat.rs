use crate::interface::config::RuntimeConfig;
use crate::interface::tui::app::{ChatMessage, ChatMessageStatus, PlanDraft, RequestState};
use crate::interface::tui::app::AppState;
use crate::interface::tui::datasource::DataSource;
use crate::core::governance::RoutingEngine;
use crate::core::protocol::{AgentId, CommandId};
use crate::llm::adapter::build_llm_for_ws;
use crate::memory::{EventKind, MemoryCore};
use crate::rag::pipeline::build_prompt;
use anyhow::Result;
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ChatSource;

impl DataSource for ChatSource {
    fn tick(&mut self, cfg: &RuntimeConfig, state: &mut AppState) -> Result<()> {
        let log_path = chat_log_path(cfg, &state.ws);
        if let Ok(existing) = load_chat_log(&log_path) {
            state.chat.transcript = existing;
            if !state.chat.transcript.is_empty() {
                state.chat.selected_index = state.chat.transcript.len().saturating_sub(1);
            }
        }

        if let Some(user_text) = state.chat.pending_user_message.take() {
            let memory = MemoryCore::new();
            let decision = RoutingEngine::route_intent(&user_text);
            let agent = agent_name(decision.agent).to_string();
            let command = command_name(decision.command).to_string();
            state.chat.last_agent = Some(agent.clone());
            state.chat.last_command = Some(command.clone());
            let _ = memory.put_event(
                &state.ws,
                "tui-chat",
                EventKind::System,
                &format!("agent.turn.start agent={} command={}", agent, command),
            );
            let _ = memory.put_event(
                &state.ws,
                "tui-chat",
                EventKind::User,
                &format!("chat.message.sent {}", user_text),
            );
            state.chat.request_state = RequestState::Sending;
            let provider = state
                .providers
                .selected
                .as_ref()
                .and_then(|id| state.providers.list.iter().find(|p| &p.id == id))
                .or_else(|| state.providers.list.first());
            let provider_id = provider.map(|p| p.id.clone());
            let provider_model = provider.map(|p| p.model.clone());
            if provider.is_none() {
                state.chat.transcript.push(ChatMessage {
                    id: format!("chat:{}:{}", state.ws, state.chat.transcript.len() + 1),
                    ts: now_epoch(),
                    role: "SYSTEM".to_string(),
                    text: "No provider selected. Press p to select provider or use ': provider select <id>'.".to_string(),
                    status: ChatMessageStatus::Info,
                    provider_id,
                    model_id: provider_model,
                });
                state.chat.last_error = "no provider selected".to_string();
                state.chat.request_state = RequestState::Error;
                save_chat_log(&log_path, &state.chat.transcript)?;
                return Ok(());
            }
            state.chat.is_streaming = true;
            state.chat.request_state = if state.chat.streaming_enabled {
                RequestState::Streaming
            } else {
                RequestState::Sending
            };
            let prompt = build_prompt(&user_text, &memory, &state.ws);
            state.chat.context_preview = prompt.lines().take(8).collect::<Vec<_>>().join("\n");
            let llm = build_llm_for_ws(cfg, &state.ws);
            let mut streamed = String::new();
            let stream_draft_idx = if state.chat.streaming_enabled {
                state.chat.transcript.push(ChatMessage {
                    id: format!("chat:{}:{}", state.ws, state.chat.transcript.len() + 1),
                    ts: now_epoch(),
                    role: "AGENT".to_string(),
                    text: String::new(),
                    status: ChatMessageStatus::Draft,
                    provider_id: provider_id.clone(),
                    model_id: provider_model.clone(),
                });
                Some(state.chat.transcript.len() - 1)
            } else {
                None
            };
            let result = if state.chat.streaming_enabled {
                let mut on_delta = |delta: &str| {
                    streamed.push_str(delta);
                    let _ = memory.put_event(&state.ws, "tui-chat", EventKind::Agent, "chat.message.delta");
                    if let Some(idx) = stream_draft_idx {
                        if let Some(msg) = state.chat.transcript.get_mut(idx) {
                            msg.text.push_str(delta);
                        }
                        state.chat.selected_index = idx;
                    }
                };
                llm.complete_stream(&prompt, &mut on_delta)
            } else {
                llm.complete(&prompt)
            };
            match result {
                Ok(resp) => {
                    let final_text = if state.chat.streaming_enabled {
                        if streamed.is_empty() { resp } else { streamed }
                    } else {
                        resp
                    };
                    if state.chat.streaming_enabled {
                        if let Some(idx) = stream_draft_idx {
                            if let Some(msg) = state.chat.transcript.get_mut(idx) {
                                if msg.text.is_empty() {
                                    msg.text = final_text.clone();
                                }
                            }
                            state.chat.selected_index = idx;
                        }
                    } else {
                        state.chat.transcript.push(ChatMessage {
                            id: format!("chat:{}:{}", state.ws, state.chat.transcript.len() + 1),
                            ts: now_epoch(),
                            role: "AGENT".to_string(),
                            text: final_text.clone(),
                            status: ChatMessageStatus::Draft,
                            provider_id: provider_id.clone(),
                            model_id: provider_model.clone(),
                        });
                    }
                    state.chat.draft_plan = Some(infer_plan(&final_text));
                    state.chat.last_error.clear();
                    let _ = memory.put_event(
                        &state.ws,
                        "tui-chat",
                        EventKind::Agent,
                        "chat.message.final",
                    );
                    let _ = memory.put_event(
                        &state.ws,
                        "tui-chat",
                        EventKind::System,
                        &format!("agent.turn.end agent={} command={} status=ok", agent, command),
                    );
                    state.chat.request_state = RequestState::Done;
                }
                Err(e) => {
                    state.chat.last_error = e.to_string();
                    state.chat.transcript.push(ChatMessage {
                        id: format!("chat:{}:{}", state.ws, state.chat.transcript.len() + 1),
                        ts: now_epoch(),
                        role: "SYSTEM".to_string(),
                        text: format!("LLM error: {}", state.chat.last_error),
                        status: ChatMessageStatus::Info,
                        provider_id: provider_id.clone(),
                        model_id: provider_model.clone(),
                    });
                    let _ = memory.put_event(&state.ws, "tui-chat", EventKind::System, "chat.message.error");
                    let _ = memory.put_event(
                        &state.ws,
                        "tui-chat",
                        EventKind::System,
                        &format!("agent.turn.end agent={} command={} status=error", agent, command),
                    );
                    state.chat.request_state = RequestState::Error;
                }
            }
            state.chat.is_streaming = false;
            save_chat_log(&log_path, &state.chat.transcript)?;
        }
        Ok(())
    }
}

fn chat_log_path(cfg: &RuntimeConfig, ws: &str) -> PathBuf {
    cfg.run_dir.join(ws).join("chat.log")
}

fn load_chat_log(path: &PathBuf) -> Result<Vec<ChatMessage>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(path)?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(line)?;
        let mut msg: ChatMessage = serde_json::from_value(value)?;
        if msg.role.eq_ignore_ascii_case("assistant") {
            msg.role = "AGENT".to_string();
        }
        out.push(msg);
    }
    compact_transcript(&mut out);
    Ok(out)
}

fn save_chat_log(path: &PathBuf, messages: &[ChatMessage]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    for m in messages {
        writeln!(file, "{}", serde_json::to_string(m)?)?;
    }
    Ok(())
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn infer_plan(resp: &str) -> PlanDraft {
    let lower = resp.to_ascii_lowercase();
    let requires_apply = ["write", "delete", "import", "attach", "detach", "up ", "down "]
        .iter()
        .any(|k| lower.contains(k));
    let summary = resp.lines().next().unwrap_or("no summary").trim().to_string();
    let mut actions = Vec::new();
    if requires_apply {
        actions.push("deterministic commit required".to_string());
    } else {
        actions.push("advisory response only".to_string());
    }
    PlanDraft {
        summary,
        requires_apply,
        actions,
    }
}

fn compact_transcript(messages: &mut Vec<ChatMessage>) {
    if messages.is_empty() {
        return;
    }
    let mut compacted: Vec<ChatMessage> = Vec::with_capacity(messages.len());
    for msg in messages.drain(..) {
        let can_merge = compacted.last().map(|last| {
            let role_ok = (last.role == "AGENT" || last.role == "ASSISTANT")
                && (msg.role == "AGENT" || msg.role == "ASSISTANT");
            let draft_ok = last.status == ChatMessageStatus::Draft && msg.status == ChatMessageStatus::Draft;
            let provider_ok = last.provider_id == msg.provider_id;
            let near_ts = msg.ts.saturating_sub(last.ts) <= 300;
            let chunk_like = is_chunk_like(&last.text, &msg.text);
            role_ok && draft_ok && provider_ok && near_ts && chunk_like
        }).unwrap_or(false);

        if can_merge {
            if let Some(last) = compacted.last_mut() {
                last.text.push_str(&msg.text);
                if msg.ts > last.ts {
                    last.ts = msg.ts;
                }
            }
        } else {
            compacted.push(msg);
        }
    }
    *messages = compacted;
}

fn is_chunk_like(prev: &str, next: &str) -> bool {
    let p = prev.trim_end();
    let n = next.trim_start();
    if p.is_empty() || n.is_empty() {
        return true;
    }
    let prev_short = p.chars().count() <= 24;
    let next_short = n.chars().count() <= 24;
    let prev_open = !p.ends_with('.') && !p.ends_with('!') && !p.ends_with('?');
    (prev_short || next_short) && prev_open
}

fn agent_name(agent: AgentId) -> &'static str {
    match agent {
        AgentId::System => "system",
        AgentId::Code => "code",
        AgentId::Historian => "historian",
        AgentId::Validator => "validator",
        AgentId::Knowledge => "knowledge",
    }
}

fn command_name(command: CommandId) -> &'static str {
    match command {
        CommandId::None => "none",
        CommandId::Ping => "ping",
        CommandId::Noop => "noop",
    }
}
