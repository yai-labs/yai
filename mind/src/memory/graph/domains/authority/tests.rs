use super::api;

#[test]
#[ignore = "requires engine UDS socket"]
fn authority_load_ok() {
    let ws = "test_auth";
    let _ = api::load(ws).unwrap_or_default();
}
