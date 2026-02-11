use super::api::activate;
use crate::memory::graph::semantic::types::{SemanticEdge, SemanticNode};

#[test]
fn activation_deterministic() {
    let nodes = vec![
        SemanticNode { id: "node:file:n1".to_string(), kind: "file".to_string(), meta: serde_json::Value::Null, last_seen: 1 },
        SemanticNode { id: "node:error:n2".to_string(), kind: "error".to_string(), meta: serde_json::Value::Null, last_seen: 1 },
    ];
    let edges = vec![SemanticEdge { id: "edge:rel:node:file:n1:node:error:n2".to_string(), src: "node:file:n1".to_string(), dst: "node:error:n2".to_string(), rel: "rel".to_string(), weight: 1.0 }];
    let seeds = vec![("node:file:n1".to_string(), 1.0)];
    let res = activate(&nodes, &edges, &seeds, 1, 0.6, 0.05, 4);
    assert_eq!(res.nodes.len(), 2);
    assert!(res.nodes[0].activation >= res.nodes[1].activation);
}
