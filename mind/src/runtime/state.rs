// src/core/state.rs
#![allow(dead_code)]
use crate::transport::bridge::shm::VaultState;
use std::sync::{Arc, Mutex};

pub struct SharedState {
    pub connected_clients: usize,
    pub last_vault_update: Option<VaultState>,
    pub session_id: String,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            connected_clients: 0,
            last_vault_update: None,
            session_id: "init_session".to_string(),
        }
    }
}

pub type GlobalState = Arc<Mutex<SharedState>>;
