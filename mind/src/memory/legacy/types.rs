use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventKind {
    User,
    Agent,
    System,
}

impl EventKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventKind::User => "user",
            EventKind::Agent => "agent",
            EventKind::System => "system",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "user" => EventKind::User,
            "agent" => EventKind::Agent,
            "system" => EventKind::System,
            _ => EventKind::System,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    pub ws: String,
    pub trace: String,
    pub ts: i64,
    pub kind: EventKind,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub tags: Vec<String>,
    pub ts: i64,
}
