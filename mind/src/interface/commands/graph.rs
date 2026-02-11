use crate::interface::config::RuntimeConfig;
use crate::memory::graph::activation::api::activate;
use crate::memory::graph::bridge;
use crate::memory::graph::ids::{EdgeId, NodeId};
use crate::memory::graph::semantic;
use crate::memory::graph::semantic::types::SemanticNode;
use crate::memory::graph::vector;
use crate::providers::embeddings;
use anyhow::Result;
use serde_json::Value;
pub fn add_node(_cfg: &RuntimeConfig, ws: &str, id: &str, kind: &str, meta: &str) -> Result<()> {
    let _ = NodeId::parse(id)?;
    let meta_val: Value = serde_json::from_str(meta).unwrap_or(Value::Null);
    semantic::api::add_node(ws, id, kind, &meta_val)?;
    Ok(())
}

pub fn add_edge(_cfg: &RuntimeConfig, ws: &str, src: &str, dst: &str, rel: &str, weight: f32) -> Result<()> {
    let _ = NodeId::parse(src)?;
    let _ = NodeId::parse(dst)?;
    let edge_id = format!("edge:{}:{}:{}", rel, src, dst);
    let _ = EdgeId::parse(&edge_id)?;
    semantic::api::add_edge(ws, &edge_id, src, dst, rel, weight)?;
    Ok(())
}

pub fn query(_cfg: &RuntimeConfig, ws: &str, text: &str, k: usize) -> Result<()> {
    let _ = bridge::ingest_events_to_semantic(ws);
    let nodes = semantic::api::list_nodes(ws)?;
    let edges = semantic::api::list_edges(ws)?;
    let (embedder, source) = embeddings::build_from_env()?;

    if !nodes.is_empty() {
        vector::api::rebuild_from_semantic(ws, &nodes, |n: &SemanticNode| {
            let payload = format!("{} {}", n.id, n.meta);
            embedder.embed(&payload)
        })?;
    }

    let query_vec = embedder.embed(text)?;
    println!("embedder: {}", source);
    let hits = vector::api::search(ws, &query_vec, k)?;
    let seeds: Vec<(String, f32)> = hits.into_iter().map(|h| (h.0, h.1)).collect();
    let result = activate(&nodes, &edges, &seeds, 1, 0.6, 0.05, k * 2);

    println!("nodes: {}", result.nodes.len());
    for node in result.nodes {
        let last = node
            .last_seen
            .map(|v: u64| v.to_string())
            .unwrap_or_else(|| "none".to_string());
        println!("node {} kind={} activation={:.4} last_seen={}", node.id, node.kind, node.activation, last);
    }
    println!("edges: {}", result.edges.len());
    for edge in result.edges {
        println!("edge {} -> {} rel={} w={}", edge.src, edge.dst, edge.rel, edge.weight);
    }
    Ok(())
}
