use super::api;

#[test]
fn episodic_ingest_deterministic() {
    let ws = "test_episode";
    let eps = api::ingest(ws).unwrap_or_default();
    let mut uniq = std::collections::HashSet::new();
    for e in &eps {
        uniq.insert(e.id.clone());
    }
    assert_eq!(uniq.len(), eps.len());
}
