use yai_mind::types::memory::EventKind;

#[test]
fn rag_prompt_and_event_kind_roundtrip() {
    assert_eq!(EventKind::from_str("user"), EventKind::User);
    assert_eq!(EventKind::from_str("agent"), EventKind::Agent);
    assert_eq!(EventKind::from_str("system"), EventKind::System);
    assert_eq!(EventKind::User.as_str(), "user");
}
