// src/models.rs
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    Handshake,
    HandshakeAck,
    Intent,
    Response, // Aggiunto per coerenza con il flusso
    Heartbeat,
    Log,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IceMessage {
    pub r#type: MessageType,           // r# per usare la keyword riservata 'type'
    pub version: String,               // es. 2.0.0-lite
    pub payload: HashMap<String, Value>, 
    pub timestamp: f64,                // Unix timestamp
}

impl IceMessage {
    /// Crea un nuovo messaggio con timestamp automatico
    pub fn new(msg_type: MessageType, version: &str, payload: HashMap<String, Value>) -> Self {
        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs_f64();

        Self {
            r#type: msg_type,
            version: version.to_string(),
            payload,
            timestamp,
        }
    }

    /// Helper per convertire il messaggio in stringa JSON (per il WebSocket)
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
