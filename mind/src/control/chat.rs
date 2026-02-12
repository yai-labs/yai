use anyhow::{bail, Result};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub ts_ms: u64,
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub id: String,
    pub title: Option<String>,
    pub created_ts_ms: u64,
    pub last_ts_ms: u64,
}

#[derive(Default)]
pub struct ChatStore {
    sessions: Mutex<HashMap<String, ChatSession>>,
    messages: Mutex<HashMap<String, Vec<Message>>>,
    selected: Mutex<Option<String>>,
}

impl ChatStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sessions(&self) -> Vec<ChatSession> {
        let mut out: Vec<ChatSession> = self.sessions.lock().unwrap().values().cloned().collect();
        out.sort_by_key(|s| s.last_ts_ms);
        out.reverse();
        out
    }

    pub fn create_session(&self, title: Option<String>) -> ChatSession {
        let now = now_ms();
        let id = format!("chat-{}", now);
        let session = ChatSession {
            id: id.clone(),
            title,
            created_ts_ms: now,
            last_ts_ms: now,
        };
        self.sessions
            .lock()
            .unwrap()
            .insert(id.clone(), session.clone());
        self.messages.lock().unwrap().insert(id, Vec::new());
        session
    }

    pub fn select_session(&self, session_id: &str) -> Result<()> {
        if !self.sessions.lock().unwrap().contains_key(session_id) {
            bail!("chat session not found: {session_id}");
        }
        *self.selected.lock().unwrap() = Some(session_id.to_string());
        Ok(())
    }

    pub fn selected_session(&self) -> Option<String> {
        self.selected.lock().unwrap().clone()
    }

    pub fn history(&self, session_id: &str) -> Result<Vec<Message>> {
        let map = self.messages.lock().unwrap();
        map.get(session_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("chat session not found: {session_id}"))
    }

    pub fn append(&self, session_id: &str, role: Role, content: String) -> Result<Message> {
        let now = now_ms();
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("chat session not found: {session_id}"))?;
        let msg = Message {
            id: format!("{}-{}", session_id, now),
            ts_ms: now,
            role,
            content,
        };
        let mut messages = self.messages.lock().unwrap();
        let history = messages
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("chat session not found: {session_id}"))?;
        history.push(msg.clone());
        session.last_ts_ms = now;
        Ok(msg)
    }
}

pub struct ChatEngine {
    store: ChatStore,
}

impl Default for ChatEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatEngine {
    pub fn new() -> Self {
        Self {
            store: ChatStore::new(),
        }
    }

    pub fn store(&self) -> &ChatStore {
        &self.store
    }

    pub fn send_echo(&self, session_id: &str, text: &str) -> Result<Message> {
        if text.trim().is_empty() {
            bail!("chat input is empty");
        }
        self.store
            .append(session_id, Role::User, text.to_string())?;
        let reply = format!("echo: {text}");
        self.store.append(session_id, Role::Assistant, reply)
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
