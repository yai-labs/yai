use crate::cli::proc::RunState;
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
pub struct ComplianceContext {
    pub pack_ref: String,
    pub purpose_id: String,
    pub data_class: String,
    pub retention_policy_id: String,
    pub legal_basis: String,
    pub subject_scope: String,
    pub processor_role: String,
    pub audit_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DsarStatus {
    Requested,
    Verified,
    Approved,
    Executed,
    Rejected,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DsarRecord {
    pub request_id: String,
    pub subject_ref: String,
    pub request_type: String,
    pub status: DsarStatus,
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
    DsarRequest {
        request_type: String,
        subject_ref: String,
    },
    DsarStatus {
        request_id: String,
    },
    DsarExecute {
        request_id: String,
    },
    ChatSessionsList,
    ChatSessionNew {
        title: Option<String>,
    },
    ChatSessionSelect {
        session_id: String,
    },
    ChatHistory {
        session_id: Option<String>,
    },
    ChatSend {
        session_id: Option<String>,
        text: String,
        stream: bool,
    },
    ShellExec {
        cmd: String,
        args: Vec<String>,
        cwd: Option<String>,
    },
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
    DsarCreated {
        request: DsarRecord,
    },
    DsarState {
        request: Option<DsarRecord>,
    },
    DsarExecuted {
        request: DsarRecord,
    },
    ChatSessions {
        items: Vec<ChatSession>,
        selected: Option<String>,
    },
    ChatSession {
        session: ChatSession,
    },
    ChatHistory {
        session_id: String,
        items: Vec<ChatMessage>,
    },
    ChatSend {
        message: ChatMessage,
    },
    ShellExec {
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
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
    #[serde(default)]
    pub compliance: Option<ComplianceContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub ts_ms: u64,
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: Option<String>,
    pub created_ts_ms: u64,
    pub last_ts_ms: u64,
}
