use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub kind: String,
    pub meta: Value,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub src: String,
    pub dst: String,
    pub rel: String,
    pub weight: f32,
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GraphScope {
    Global,
    Workspace(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subgraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NeighborFilters {
    pub directed: bool,
    pub kinds: Option<Vec<String>>,
    pub rels: Option<Vec<String>>,
}

impl Default for NeighborFilters {
    fn default() -> Self {
        Self { directed: false, kinds: None, rels: None }
    }
}

pub enum GraphExportFormat { Dot, Jsonl }

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphStats {
    pub scope: String,
    pub backend: String,
    pub nodes: usize,
    pub edges: usize,
    pub categories: BTreeMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivationResult {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub scores: BTreeMap<String, f32>,
    pub metrics: Option<Value>, // Mantiene ActivationStats
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivationCommit {
    pub activation_id: String,
    pub graph_snapshot_id: String,
    pub params_hash: String,
    pub seed_hash: String,
    pub result_hash: String,
    pub trace_hash: String,
    pub proof_passed: bool,
}

pub trait GraphStore: Send + Sync {
    fn put_node(&self, node: &GraphNode) -> anyhow::Result<()>;
    fn put_edge(&self, edge: &GraphEdge) -> anyhow::Result<()>;
    fn list_nodes(&self) -> anyhow::Result<Vec<GraphNode>>;
    fn list_edges(&self) -> anyhow::Result<Vec<GraphEdge>>;
    fn get_node(&self, id: &str) -> anyhow::Result<Option<GraphNode>>;
    fn get_edges_for_node(&self, id: &str) -> anyhow::Result<Vec<GraphEdge>>;
    fn record_activation_trace(&self, ws_id: &str, trace: Value) -> anyhow::Result<()>;
    fn descriptor(&self) -> String;
}