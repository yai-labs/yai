use crate::memory::graph::semantic::store::SemanticStore;
use crate::memory::graph::semantic::types::{SemanticEdge, SemanticNode};
use anyhow::Result;
use serde_json::Value;

pub fn add_node(ws: &str, id: &str, kind: &str, meta: &Value) -> Result<()> {
    let store = SemanticStore::open(ws)?;
    store.upsert_node(id, kind, meta)
}

pub fn add_edge(ws: &str, id: &str, src: &str, dst: &str, rel: &str, weight: f32) -> Result<()> {
    let store = SemanticStore::open(ws)?;
    store.upsert_edge(id, src, dst, rel, weight)
}

pub fn list_nodes(ws: &str) -> Result<Vec<SemanticNode>> {
    let store = SemanticStore::open(ws)?;
    store.list_nodes()
}

pub fn list_edges(ws: &str) -> Result<Vec<SemanticEdge>> {
    let store = SemanticStore::open(ws)?;
    store.list_edges()
}
