// mind/src/transport/protocol.rs

pub type TraceId = [u8; 36];
pub type WorkspaceId = [u8; 36];

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YaiCommand {
    None = 0,
    Ping = 1,
    Handshake = 2,
    StorageRpc = 0x50,
    Inference = 0x0301, // Il cuore del Blocco 1
}

#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub allowed: bool,
    pub reason: String,
}

// Helper per convertire stringhe in buffer fissi da 36 byte (usato da uds_server)
pub fn string_to_fixed_36(s: &str) -> [u8; 36] {
    let mut buffer = [0u8; 36];
    let bytes = s.as_bytes();
    let len = bytes.len().min(35);
    buffer[..len].copy_from_slice(&bytes[..len]);
    buffer
}
