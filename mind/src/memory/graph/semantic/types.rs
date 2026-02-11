use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticNode {
    pub id: String,
    pub kind: String,
    pub meta: Value,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEdge {
    pub id: String,
    pub src: String,
    pub dst: String,
    pub rel: String,
    pub weight: f32,
}
