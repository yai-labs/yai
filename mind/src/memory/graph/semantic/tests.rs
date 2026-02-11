use super::api;
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
