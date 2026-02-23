use super::api;
use crate::memory::graph::domains::semantic::types::NodeRetention;
use serde_json::json;

#[test]
fn semantic_add_list_deterministic() {
    let ws = "test_semantic";
    let _ = api::add_node(ws, "node:file:a", "file", &json!({"path":"a"}));
    let _ = api::add_node(ws, "node:error:b", "error", &json!({"code":"E"}));
    let nodes = api::list_nodes(ws).unwrap();
    assert!(nodes.len() >= 2);
    assert!(nodes[0].id <= nodes[1].id);
}

#[test]
fn retention_expire_is_deterministic_and_tombstones() {
    let ws = "test_semantic_retention";
    let node_id = "node:episode:test:10:1";
    let retention = NodeRetention {
        created_ts: 10,
        retention_policy_id: "default".to_string(),
        ttl_seconds: Some(2),
        compliance: Some(json!({
            "pack_ref": "gdpr-eu/2026Q1",
            "purpose_id": "LEGAL_OBLIGATION",
            "data_class": "PERSONAL",
            "retention_policy_id": "default",
            "legal_basis": "LEGAL_OBLIGATION",
            "subject_scope": "identified",
            "processor_role": "controller",
            "audit_required": true
        })),
    };

    let _ = api::add_node_with_retention(
        ws,
        node_id,
        "episode",
        &json!({"event_type":"DATA_EXPORT","ts":10}),
        &retention,
    );

    let first = api::expire_due(ws, 11).unwrap();
    assert!(first.is_empty());

    let second = api::expire_due(ws, 12).unwrap();
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].id, node_id);
    assert_eq!(second[0].expired_at, 12);

    let third = api::expire_due(ws, 12).unwrap();
    assert!(third.is_empty());

    let active_nodes = api::list_nodes(ws).unwrap();
    assert!(active_nodes.iter().all(|n| n.id != node_id));
}

#[test]
fn retention_skips_records_without_compliance() {
    let ws = "test_semantic_no_compliance_retention";
    let node_id = "node:episode:test:20:1";
    let retention = NodeRetention {
        created_ts: 20,
        retention_policy_id: "none".to_string(),
        ttl_seconds: Some(1),
        compliance: None,
    };

    let _ = api::add_node_with_retention(
        ws,
        node_id,
        "episode",
        &json!({"event_type":"daemon_started","ts":20}),
        &retention,
    );

    let expired = api::expire_due(ws, 999).unwrap();
    assert!(expired.is_empty());

    let active_nodes = api::list_nodes(ws).unwrap();
    assert!(active_nodes.iter().any(|n| n.id == node_id));
}
