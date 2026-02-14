use crate::memory::graph::domains::semantic::store::SemanticStore;
use crate::memory::graph::domains::semantic::types::{
    ExpiredSemanticNode, NodeRetention, SemanticEdge, SemanticNode,
};
use anyhow::Result;
use serde_json::Value;

pub fn add_node(ws: &str, id: &str, kind: &str, meta: &Value) -> Result<()> {
    let store = SemanticStore::open(ws)?;
    store.upsert_node(id, kind, meta)
}

pub fn add_node_with_retention(
    ws: &str,
    id: &str,
    kind: &str,
    meta: &Value,
    retention: &NodeRetention,
) -> Result<()> {
    let store = SemanticStore::open(ws)?;
    store.upsert_node_with_retention(id, kind, meta, retention)
}

pub fn add_edge(ws: &str, id: &str, src: &str, dst: &str, rel: &str, weight: f32) -> Result<()> {
    let store = SemanticStore::open(ws)?;
    store.upsert_edge(id, src, dst, rel, weight)
}

pub fn expire_due(ws: &str, now_ts: u64) -> Result<Vec<ExpiredSemanticNode>> {
    let store = SemanticStore::open(ws)?;
    store.expire_due(now_ts)
}

pub fn list_nodes(ws: &str) -> Result<Vec<SemanticNode>> {
    let store = SemanticStore::open(ws)?;
    store.list_nodes()
}

pub fn list_edges(ws: &str) -> Result<Vec<SemanticEdge>> {
    let store = SemanticStore::open(ws)?;
    store.list_edges()
}
