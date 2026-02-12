use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViewKind {
    Overview,
    Graph,
    Events,
    Logs,
    Db,
    Providers,
    Contracts,
    Chat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FocusZone {
    Navigator,
    Body,
    Details,
    Composer,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeStatus {
    pub boot_alive: bool,
    pub kernel_alive: bool,
    pub engine_alive: bool,
    pub mind_alive: bool,
    pub runtime_sock_exists: bool,
    pub control_sock_exists: bool,
    pub awareness_active: bool,
    pub awareness_last_line: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubgraphState {
    pub nodes: usize,
    pub edges: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivationState {
    pub nodes: usize,
    pub edges: usize,
    pub top_scores: BTreeMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphState {
    pub selected_node: Option<String>,
    pub selected_node_kind: Option<String>,
    pub selected_node_meta: Value,
    pub selected_node_last_seen: u64,
    pub node_list: Vec<String>,
    pub neighbors_preview: Vec<String>,
    pub activation_top: Vec<String>,
    pub last_subgraph: Option<SubgraphState>,
    pub last_activation: Option<ActivationState>,
    pub stats_nodes: usize,
    pub stats_edges: usize,
    pub backend: String,
    pub depth: usize,
    pub activate_requested: bool,
    pub selected_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventsState {
    pub last_n: usize,
    pub filters: Vec<String>,
    pub items: Vec<String>,
    pub selected: usize,
    pub expanded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogsState {
    pub source_selected: String,
    pub tail_buffer: Vec<String>,
    pub search_term: String,
    pub lines: usize,
    pub follow: bool,
    pub selected: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DbScope {
    Global,
    Workspace,
}

impl Default for DbScope {
    fn default() -> Self {
        Self::Workspace
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DbState {
    pub selected_db: DbScope,
    pub tables: Vec<String>,
    pub counts: BTreeMap<String, i64>,
    pub preview: Vec<Value>,
    pub selected_table: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderItem {
    pub id: String,
    pub endpoint: String,
    pub model: String,
    pub trust_state: String,
    pub last_seen: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProvidersState {
    pub list: Vec<ProviderItem>,
    pub selected: Option<String>,
    pub trust: String,
    pub selected_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractsState {
    pub graph_spec_version: String,
    pub commands_spec_version: String,
    pub compliance_pack: String,
    pub last_check: String,
    pub violations: Vec<String>,
    pub files: Vec<ContractFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractFile {
    pub path: String,
    pub checksum: String,
    pub modified_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChatMessageStatus {
    User,
    Draft,
    Committed,
    Discarded,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub ts: u64,
    pub role: String,
    pub text: String,
    pub status: ChatMessageStatus,
    pub provider_id: Option<String>,
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanDraft {
    pub summary: String,
    pub requires_apply: bool,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommitTarget {
    None,
    Events,
    Memory,
    Graph,
    Code,
}

impl Default for CommitTarget {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RequestState {
    Idle,
    Sending,
    Streaming,
    Done,
    Error,
}

impl Default for RequestState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatState {
    pub transcript: Vec<ChatMessage>,
    pub input: String,
    pub pending_user_message: Option<String>,
    pub draft_plan: Option<PlanDraft>,
    pub last_agent: Option<String>,
    pub last_command: Option<String>,
    pub context_preview: String,
    pub is_streaming: bool,
    pub streaming_enabled: bool,
    pub last_error: String,
    pub request_state: RequestState,
    pub selected_index: usize,
    pub scroll: usize,
    pub commit_target: CommitTarget,
    pub provider_session: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandPaletteState {
    pub active: bool,
    pub input: String,
    pub history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppError {
    pub source: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub ws: String,
    pub active_view: ViewKind,
    pub focus: FocusZone,
    pub show_help: bool,
    pub nav_index: usize,
    pub status: RuntimeStatus,
    pub graph: GraphState,
    pub events: EventsState,
    pub logs: LogsState,
    pub db: DbState,
    pub providers: ProvidersState,
    pub contracts: ContractsState,
    pub chat: ChatState,
    pub palette: CommandPaletteState,
    pub errors: Vec<AppError>,
}

impl AppState {
    pub fn new(ws: String) -> Self {
        Self {
            ws,
            active_view: ViewKind::Overview,
            focus: FocusZone::Body,
            show_help: false,
            nav_index: 0,
            status: RuntimeStatus::default(),
            graph: GraphState::default(),
            events: EventsState {
                last_n: 100,
                ..EventsState::default()
            },
            logs: LogsState {
                source_selected: "events".to_string(),
                lines: 200,
                ..LogsState::default()
            },
            db: DbState::default(),
            providers: ProvidersState::default(),
            contracts: ContractsState {
                graph_spec_version: "graph.v1".to_string(),
                commands_spec_version: "commands.v1".to_string(),
                compliance_pack: "baseline".to_string(),
                ..ContractsState::default()
            },
            chat: ChatState {
                streaming_enabled: true,
                ..ChatState::default()
            },
            palette: CommandPaletteState::default(),
            errors: Vec::new(),
        }
    }
}
