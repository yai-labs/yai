use crate::interface::proc::RunState;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliveStatus {
    pub boot: bool,
    pub kernel: bool,
    pub engine: bool,
    pub mind: bool,
}

impl Default for AliveStatus {
    fn default() -> Self {
        Self {
            boot: false,
            kernel: false,
            engine: false,
            mind: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Discovered,
    Paired,
    Attached,
    Detached,
    Revoked,
}

fn default_trust_state() -> TrustState {
    TrustState::Discovered
}

fn default_caps() -> Vec<String> {
    Vec::new()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub id: String,
    pub endpoint: String,
    pub model: String,
    #[serde(default = "default_trust_state")]
    pub trust_state: TrustState,
    #[serde(default)]
    pub fingerprint: Option<String>,
    #[serde(default = "default_caps")]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub last_seen: u64,
    #[serde(default)]
    pub attached_ws: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanityStatus {
    pub runtime_sock_exists: bool,
    pub control_sock_exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    Ping,
    Status,
    Up {
        build: bool,
        no_engine: bool,
        no_mind: bool,
        ai: bool,
        timeout_ms: Option<u64>,
    },
    Down {
        force: bool,
        shutdown: bool,
    },
    ProvidersDiscover {
        endpoint: Option<String>,
        model: Option<String>,
    },
    ProvidersList,
    ProvidersPair {
        id: String,
        endpoint: String,
        model: String,
    },
    ProvidersAttach {
        id: String,
        model: Option<String>,
    },
    ProvidersDetach,
    ProvidersRevoke {
        id: String,
    },
    ProvidersStatus,
    EventsSubscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Pong,
    Status {
        state: Option<RunState>,
        alive: AliveStatus,
        daemon_pid: u32,
        sanity: SanityStatus,
        halt_reason: Option<String>,
    },
    UpOk,
    DownOk {
        shutdown: bool,
    },
    Providers {
        items: Vec<ProviderInfo>,
    },
    ProviderStatus {
        active: Option<ProviderInfo>,
    },
    ProvidersOk,
    EventsStarted,
    Event {
        event: Event,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub v: u8,
    pub event_id: String,
    pub ts: u64,
    pub ws: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub level: String,
    pub msg: String,
    pub seq: u64,
    pub data: Value,
}
