use super::api;

#[test]
fn authority_load_ok() {
    let ws = "test_auth";
    let _ = api::load(ws).unwrap_or_default();
}
