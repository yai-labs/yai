use crate::cognition::memory::graph::semantic::types::{ExpiredSemanticNode, NodeRetention};
use crate::cognition::memory::graph::{episodic, semantic, vector};
use crate::providers::embeddings;
use anyhow::Result;
use serde_json::json;

fn episode_node_id(event_type: &str, ts: u64, seq: u64) -> String {
    format!("node:episode:{}:{}:{}", event_type, ts, seq)
}

fn kernel_node_id(ws: &str) -> String {
    format!("node:kernel:{}", ws)
}

fn retention_ttl_seconds() -> u64 {
    std::env::var("YAI_RETENTION_TTL_SECONDS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(2_592_000)
}

fn should_apply_retention(compliance: &serde_json::Value) -> bool {
    let class = compliance
        .get("data_class")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    match class {
        "PERSONAL" | "SPECIAL" => true,
        "INTERNAL" => std::env::var("YAI_RETENTION_INCLUDE_INTERNAL")
            .ok()
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false),
        _ => false,
    }
}

pub fn ingest_events_to_semantic(ws: &str) -> Result<usize> {
    let episodes = episodic::api::ingest(ws)?;
    if episodes.is_empty() {
        return Ok(0);
    }

    let ttl = retention_ttl_seconds();

    for ep in &episodes {
        let node_id = episode_node_id(&ep.event_type, ep.ts, ep.seq);
        let meta = json!({
            "event_id": ep.id,
            "event_type": ep.event_type,
            "ts": ep.ts,
            "seq": ep.seq,
            "data": ep.data,
        });

        let (ttl_seconds, retention_policy_id) = if let Some(c) = ep.compliance.as_ref() {
            if should_apply_retention(c) {
                let policy = c
                    .get("retention_policy_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string();
                (Some(ttl), policy)
            } else {
                (None, "none".to_string())
            }
        } else {
            (None, "none".to_string())
        };

        let retention = NodeRetention {
            created_ts: ep.ts,
            retention_policy_id,
            ttl_seconds,
            compliance: ep.compliance.clone(),
        };
        semantic::api::add_node_with_retention(ws, &node_id, "episode", &meta, &retention)?;

        if ep.event_type == "kernel_dead" {
            let kernel_id = kernel_node_id(ws);
            semantic::api::add_node(ws, &kernel_id, "kernel", &json!({"ws": ws}))?;
            let edge_id = format!("edge:blocked_by_kernel:{}:{}", kernel_id, node_id);
            semantic::api::add_edge(ws, &edge_id, &kernel_id, &node_id, "blocked_by_kernel", 1.0)?;
        }
    }

    rebuild_vectors(ws)?;

    Ok(episodes.len())
}

pub fn expire_retained(ws: &str, now_ts: u64) -> Result<Vec<ExpiredSemanticNode>> {
    let expired = semantic::api::expire_due(ws, now_ts)?;
    if !expired.is_empty() {
        rebuild_vectors(ws)?;
    }
    Ok(expired)
}

fn rebuild_vectors(ws: &str) -> Result<()> {
    let nodes = semantic::api::list_nodes(ws)?;
    let (embedder, _source) = embeddings::build_from_env()?;
    vector::api::rebuild_from_semantic(ws, &nodes, |n| {
        let payload = format!("{} {}", n.id, n.meta);
        embedder.embed(&payload)
    })?;
    Ok(())
}
