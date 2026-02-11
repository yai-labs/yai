#![cfg(feature = "legacy-providers")]

use yai_mind::providers::{build_provider, ProviderConfig, ProviderKind};

#[test]
fn mock_embedding_deterministic() {
    let provider = build_provider(ProviderKind::Mock, ProviderConfig::default());
    let a = provider.embed("hello").expect("embed failed");
    let b = provider.embed("hello").expect("embed failed");
    assert_eq!(a, b);
    assert!(!a.is_empty());

    let c = provider.embed("world").expect("embed failed");
    assert_ne!(a, c);
}

#[test]
fn registry_builds_mock() {
    let provider = build_provider(ProviderKind::Mock, ProviderConfig::default());
    let v = provider.embed("x").expect("embed failed");
    assert!(!v.is_empty());
}
