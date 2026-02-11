use crate::memory::graph::{episodic, semantic, vector};
use crate::providers::embeddings;
use anyhow::Result;
use serde_json::json;

fn episode_node_id(event_type: &str, ts: u64, seq: u64) -> String {
    format!("node:episode:{}:{}:{}", event_type, ts, seq)
}

fn kernel_node_id(ws: &str) -> String {
    format!("node:kernel:{}", ws)
}

pub fn ingest_events_to_semantic(ws: &str) -> Result<usize> {
    let episodes = episodic::api::ingest(ws)?;
    if episodes.is_empty() {
        return Ok(0);
    }

    for ep in &episodes {
        let node_id = episode_node_id(&ep.event_type, ep.ts, ep.seq);
        let meta = json!({
            "event_id": ep.id,
            "event_type": ep.event_type,
            "ts": ep.ts,
            "seq": ep.seq,
            "data": ep.data,
        });
        semantic::api::add_node(ws, &node_id, "episode", &meta)?;

        if ep.event_type == "kernel_dead" {
            let kernel_id = kernel_node_id(ws);
            semantic::api::add_node(ws, &kernel_id, "kernel", &json!({"ws": ws}))?;
            let edge_id = format!("edge:blocked_by_kernel:{}:{}", kernel_id, node_id);
            semantic::api::add_edge(ws, &edge_id, &kernel_id, &node_id, "blocked_by_kernel", 1.0)?;
        }
    }

    let nodes = semantic::api::list_nodes(ws)?;
    let (embedder, _source) = embeddings::build_from_env()?;
    vector::api::rebuild_from_semantic(ws, &nodes, |n| {
        let payload = format!("{} {}", n.id, n.meta);
        embedder.embed(&payload)
    })?;

    Ok(episodes.len())
}
