use super::api::activate;
use crate::memory::graph::semantic::types::{SemanticEdge, SemanticNode};

#[test]
fn activation_propagates_deterministically() {
    let nodes = vec![
        SemanticNode {
            id: "node:file:n1".to_string(),
            kind: "file".to_string(),
            meta: serde_json::Value::Null,
            last_seen: 1,
            created_ts: 1,
            expires_at: None,
            retention_policy_id: "default".to_string(),
            tombstone: false,
            compliance: None,
        },
        SemanticNode {
            id: "node:error:n2".to_string(),
            kind: "error".to_string(),
            meta: serde_json::Value::Null,
            last_seen: 1,
            created_ts: 1,
            expires_at: None,
            retention_policy_id: "default".to_string(),
            tombstone: false,
            compliance: None,
        },
    ];
    let edges = vec![SemanticEdge {
        id: "edge:rel:1".to_string(),
        src: "node:file:n1".to_string(),
        dst: "node:error:n2".to_string(),
        rel: "rel".to_string(),
        weight: 1.0,
    }];

    let out = activate(
        &nodes,
        &edges,
        &[("node:file:n1".to_string(), 1.0)],
        1,
        0.6,
        0.0,
        10,
    );

    assert_eq!(out.nodes.len(), 2);
    let n1 = out
        .nodes
        .iter()
        .find(|n| n.id == "node:file:n1")
        .expect("seed node exists");
    let n2 = out
        .nodes
        .iter()
        .find(|n| n.id == "node:error:n2")
        .expect("propagated node exists");

    assert!((n1.activation - 1.0).abs() < 1e-6);
    assert!((n2.activation - 0.6).abs() < 1e-6);
}
