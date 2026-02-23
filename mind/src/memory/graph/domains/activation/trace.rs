use crate::memory::graph::domains::activation::api::{
    ActivationHit, ActivationParams, ActivationSeed, ActivationStats,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivationTrace {
    pub run_id: String,
    pub created_at_unix: i64,
    pub graph_fingerprint: String,
    pub params: ActivationParams,
    pub seeds: Vec<ActivationSeed>,
    pub commit_hash: String,
    pub topk: Vec<ActivationHit>,
    pub stats: ActivationStats,
}
