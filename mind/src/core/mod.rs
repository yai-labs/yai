// src/core/mod.rs
#![allow(dead_code)]

pub mod foundation;
pub mod governance;
pub mod state;
pub mod protocol;
pub mod scheduler;
pub mod planner;
pub mod executor;
pub mod runtime;
pub mod ai_client;

// Includiamo i documenti come costanti globali a tempo di compilazione
pub const FOUNDATION_RAW: &str = include_str!("../../../FOUNDATION.md");
pub const GOVERNANCE_RAW: &str = include_str!("../../../GOVERNANCE.md");
pub const SECURITY_RAW: &str = include_str!("../../../SECURITY.md");
