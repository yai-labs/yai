// src/bridge/mod.rs
#![allow(dead_code, unused_imports)]

pub mod shm;
pub mod uds;
pub mod vault;

// Esportiamo le strutture principali per comodità degli altri moduli
pub use shm::{VaultBridge, VaultState};
pub use uds::UdsConnector;
pub use vault::VaultBridge as EngineVaultBridge;
