use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticNode {
    pub id: String,
    pub kind: String,
    pub meta: Value,
    pub last_seen: u64,
    pub created_ts: u64,
    pub expires_at: Option<u64>,
    pub retention_policy_id: String,
    pub tombstone: bool,
    pub compliance: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEdge {
    pub id: String,
    pub src: String,
    pub dst: String,
    pub rel: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiredSemanticNode {
    pub id: String,
    pub retention_policy_id: String,
    pub expired_at: u64,
    pub compliance: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct NodeRetention {
    pub created_ts: u64,
    pub retention_policy_id: String,
    pub ttl_seconds: Option<u64>,
    pub compliance: Option<Value>,
}

impl Default for NodeRetention {
    fn default() -> Self {
        Self {
            created_ts: 0,
            retention_policy_id: "default".to_string(),
            ttl_seconds: None,
            compliance: None,
        }
    }
}
