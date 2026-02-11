use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub ts: u64,
    pub seq: u64,
    pub event_type: String,
    pub data: Value,
}
