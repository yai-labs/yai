use super::api;
use super::types::VectorEntry;

#[test]
fn vector_build_deterministic() {
    let ws = "test_vector";
    let entries = vec![
        VectorEntry { id: "node:a".to_string(), embedding: vec![1.0, 0.0] },
        VectorEntry { id: "node:b".to_string(), embedding: vec![0.0, 1.0] },
    ];
    api::build_index(ws, entries).unwrap();
    let hits = api::search(ws, &[1.0, 0.0], 1).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].0, "node:a");
}
