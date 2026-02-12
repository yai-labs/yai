use crate::interface::tui::actions::Action;
use crate::interface::tui::app::{AppState, ChatMessage, ChatMessageStatus, CommitTarget, FocusZone, ViewKind};
use crate::memory::{EventKind, MemoryCore};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::Tick | Action::Refresh => {}
        Action::ToggleHelp => state.show_help = !state.show_help,
        Action::FocusNext => {
            state.focus = match state.focus {
                FocusZone::Navigator => FocusZone::Body,
                FocusZone::Body => FocusZone::Details,
                FocusZone::Details => FocusZone::Composer,
                FocusZone::Composer => FocusZone::Navigator,
            };
        }
        Action::MoveUp => move_selection(state, true),
        Action::MoveDown => move_selection(state, false),
        Action::Select => on_select(state),
        Action::SwitchView(v) => {
            state.active_view = v.clone();
            state.nav_index = view_index(&v);
            state.focus = if v == ViewKind::Chat {
                FocusZone::Composer
            } else {
                FocusZone::Body
            };
        }
        Action::SelectWs(ws) => state.ws = ws,
        Action::RunCommand(cmd) => {
            apply_command(state, &cmd);
            state.palette.history.push(cmd);
            state.palette.input.clear();
            state.palette.active = false;
        }
        Action::GraphSelectNode(id) => state.graph.selected_node = Some(id),
        Action::GraphNeighbors { id, depth } => {
            state.graph.selected_node = Some(id);
            state.graph.depth = depth;
        }
        Action::GraphToggleDepth => {
            state.graph.depth = if state.graph.depth <= 1 { 2 } else { 1 };
        }
        Action::GraphActivateSelected => state.graph.activate_requested = true,
        Action::LogsSelectSource(s) => state.logs.source_selected = s,
        Action::LogsCycleSource => {
            let sources = ["events", "awareness", "kernel", "engine", "mind", "boot"];
            let idx = sources
                .iter()
                .position(|v| *v == state.logs.source_selected)
                .unwrap_or(0);
            state.logs.source_selected = sources[(idx + 1) % sources.len()].to_string();
        }
        Action::LogsToggleFollow => state.logs.follow = !state.logs.follow,
        Action::LogsSearch(s) => state.logs.search_term = s,
        Action::DbSelect(scope) => state.db.selected_db = scope,
        Action::DbSelectTable(t) => state.db.selected_table = Some(t),
        Action::ProvidersSelect(id) => state.providers.selected = Some(id),
        Action::ProvidersTrust { id, state: trust } => {
            state.providers.selected = Some(id);
            state.providers.trust = trust;
        }
        Action::ChatSetInput(input) => state.chat.input = input,
        Action::ChatSend(msg) => {
            let trimmed = msg.trim();
            if !trimmed.is_empty() {
                state.chat.request_state = crate::interface::tui::app::RequestState::Sending;
                state.chat.pending_user_message = Some(trimmed.to_string());
                state.chat.transcript.push(ChatMessage {
                    id: format!("chat:{}:{}", state.ws, state.chat.transcript.len() + 1),
                    ts: now_epoch(),
                    role: "USER".to_string(),
                    text: trimmed.to_string(),
                    status: ChatMessageStatus::User,
                    provider_id: state.providers.selected.clone(),
                    model_id: state
                        .providers
                        .list
                        .iter()
                        .find(|p| Some(&p.id) == state.providers.selected.as_ref())
                        .map(|p| p.model.clone()),
                });
                state.chat.input.clear();
                state.chat.selected_index = state.chat.transcript.len().saturating_sub(1);
            }
        }
        Action::ChatInputNewline => state.chat.input.push('\n'),
        Action::ChatClearInput => state.chat.input.clear(),
        Action::ChatToggleStreaming => state.chat.streaming_enabled = !state.chat.streaming_enabled,
        Action::ChatCycleCommitTarget => {
            state.chat.commit_target = match state.chat.commit_target {
                CommitTarget::None => CommitTarget::Events,
                CommitTarget::Events => CommitTarget::Memory,
                CommitTarget::Memory => CommitTarget::Graph,
                CommitTarget::Graph => CommitTarget::Code,
                CommitTarget::Code => CommitTarget::None,
            };
        }
        Action::ChatRetry => {
            if let Some(last_user) = state
                .chat
                .transcript
                .iter()
                .rev()
                .find(|m| m.role == "USER")
                .map(|m| m.text.clone())
            {
                reduce(state, Action::ChatSend(last_user));
            }
        }
        Action::ChatApplyDraft => {
            if let Some(plan) = state.chat.draft_plan.clone() {
                let target = format!("{:?}", state.chat.commit_target).to_lowercase();
                state.chat.transcript.push(ChatMessage {
                    id: format!("chat:{}:{}", state.ws, state.chat.transcript.len() + 1),
                    ts: now_epoch(),
                    role: "SYSTEM".to_string(),
                    text: format!("COMMITTED[{target}]: {}", plan.summary),
                    status: ChatMessageStatus::Committed,
                    provider_id: state.providers.selected.clone(),
                    model_id: state
                        .providers
                        .list
                        .iter()
                        .find(|p| Some(&p.id) == state.providers.selected.as_ref())
                        .map(|p| p.model.clone()),
                });
                let memory = MemoryCore::new();
                let _ = memory.put_event(
                    &state.ws,
                    "tui-chat",
                    EventKind::System,
                    &format!("chat.commit.applied target={target} {}", plan.summary),
                );
            }
            state.chat.request_state = crate::interface::tui::app::RequestState::Done;
            state.chat.draft_plan = None;
        }
        Action::ChatDiscardDraft => {
            if let Some(last) = state
                .chat
                .transcript
                .iter_mut()
                .rev()
                .find(|m| m.status == ChatMessageStatus::Draft)
            {
                last.status = ChatMessageStatus::Discarded;
            }
            state.chat.draft_plan = None;
        }
        Action::TogglePalette => state.palette.active = !state.palette.active,
        Action::PaletteInput(v) => state.palette.input = v,
    }
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn view_index(v: &ViewKind) -> usize {
    match v {
        ViewKind::Overview => 0,
        ViewKind::Graph => 1,
        ViewKind::Events => 2,
        ViewKind::Logs => 3,
        ViewKind::Db => 4,
        ViewKind::Providers => 5,
        ViewKind::Contracts => 6,
        ViewKind::Chat => 7,
    }
}

fn apply_command(state: &mut AppState, cmd: &str) {
    let trimmed = cmd.trim();
    if let Some(rest) = trimmed.strip_prefix("go ") {
        let view = match rest.trim() {
            "overview" | "o" => Some(ViewKind::Overview),
            "graph" | "g" => Some(ViewKind::Graph),
            "events" | "e" => Some(ViewKind::Events),
            "logs" | "l" => Some(ViewKind::Logs),
            "db" | "d" => Some(ViewKind::Db),
            "providers" | "p" => Some(ViewKind::Providers),
            "contracts" | "c" => Some(ViewKind::Contracts),
            "chat" | "h" => Some(ViewKind::Chat),
            _ => None,
        };
        if let Some(v) = view {
            reduce(state, Action::SwitchView(v));
        }
        return;
    }
    if let Some(rest) = trimmed.strip_prefix("ask ") {
        reduce(state, Action::ChatSend(rest.to_string()));
        return;
    }
    if trimmed == "chat send" {
        let msg = state.chat.input.clone();
        reduce(state, Action::ChatSend(msg));
        return;
    }
    if trimmed == "apply" {
        reduce(state, Action::ChatApplyDraft);
        return;
    }
    if let Some(rest) = trimmed.strip_prefix("search ") {
        reduce(state, Action::LogsSearch(rest.to_string()));
        return;
    }
    if let Some(rest) = trimmed.strip_prefix("source ") {
        reduce(state, Action::LogsSelectSource(rest.to_string()));
        return;
    }
    if let Some(rest) = trimmed.strip_prefix("provider select ") {
        reduce(state, Action::ProvidersSelect(rest.to_string()));
        return;
    }
    if let Some(rest) = trimmed.strip_prefix("ws set ") {
        reduce(state, Action::SelectWs(rest.to_string()));
        return;
    }
    if trimmed == "discard" {
        reduce(state, Action::ChatDiscardDraft);
        return;
    }
    if let Some(rest) = trimmed.strip_prefix("node ") {
        reduce(state, Action::GraphSelectNode(rest.to_string()));
    }
}

fn move_selection(state: &mut AppState, up: bool) {
    match state.focus {
        FocusZone::Navigator => {
            let max = 7usize;
            if up {
                state.nav_index = state.nav_index.saturating_sub(1);
            } else {
                state.nav_index = (state.nav_index + 1).min(max);
            }
        }
        FocusZone::Body => match state.active_view {
            ViewKind::Logs => {
                if up {
                    state.logs.selected = state.logs.selected.saturating_sub(1);
                } else {
                    state.logs.selected = state.logs.selected.saturating_add(1);
                }
            }
            ViewKind::Events => {
                if up {
                    state.events.selected = state.events.selected.saturating_sub(1);
                } else {
                    state.events.selected = state.events.selected.saturating_add(1);
                }
            }
            ViewKind::Graph => {
                if up {
                    state.graph.selected_index = state.graph.selected_index.saturating_sub(1);
                } else if !state.graph.node_list.is_empty() {
                    state.graph.selected_index =
                        (state.graph.selected_index + 1).min(state.graph.node_list.len() - 1);
                }
            }
            ViewKind::Providers => {
                if up {
                    state.providers.selected_index = state.providers.selected_index.saturating_sub(1);
                } else if !state.providers.list.is_empty() {
                    state.providers.selected_index =
                        (state.providers.selected_index + 1).min(state.providers.list.len() - 1);
                }
            }
            _ => {}
        },
        FocusZone::Details | FocusZone::Composer => {}
    }
}

fn on_select(state: &mut AppState) {
    match state.focus {
        FocusZone::Navigator => {
            let order = [
                ViewKind::Overview,
                ViewKind::Graph,
                ViewKind::Events,
                ViewKind::Logs,
                ViewKind::Db,
                ViewKind::Providers,
                ViewKind::Contracts,
                ViewKind::Chat,
            ];
            state.active_view = order[state.nav_index.min(order.len() - 1)].clone();
        }
        FocusZone::Body => match state.active_view {
            ViewKind::Events => state.events.expanded = !state.events.expanded,
            ViewKind::Graph => {
                if let Some(id) = state.graph.node_list.get(state.graph.selected_index).cloned() {
                    state.graph.selected_node = Some(id);
                }
            }
            ViewKind::Providers => {
                if let Some(p) = state.providers.list.get(state.providers.selected_index) {
                    state.providers.selected = Some(p.id.clone());
                }
            }
            _ => {}
        },
        FocusZone::Details | FocusZone::Composer => {}
    }
}
