// src/core/foundation.rs
#![allow(dead_code)]

/// YAI Foundation Dependency - Invariant Layer
/// Questo modulo incorpora i dogmi definiti nella Foundation v1.0.0.
pub const FOUNDATION_DOC: &str = include_str!("../../../FOUNDATION.md");

pub const FOUNDATION_META: FoundationMeta = FoundationMeta {
    version: "1.0.0",
    tag: "v1.0.0-foundation",
    authority: "authoritative",
};

#[derive(Debug)]
pub struct FoundationMeta {
    pub version: &'static str,
    pub tag: &'static str,
    pub authority: &'static str,
}

/// Verifica se un'azione o un intent è compatibile con i vincoli della Foundation.
/// Implementazione del "Boundary Layer" per prevenire violazioni axiomatiche.
pub fn validate_axiom_compliance(intent: &str) -> bool {
    let intent = intent.trim();
    if intent.is_empty() {
        return true;
    }

    let hay = intent.to_ascii_lowercase();

    const FORBIDDEN_PHRASES: [&str; 12] = [
        "reinterpret the foundation",
        "extend the foundation",
        "negotiate the foundation",
        "override the foundation",
        "weaken the foundation",
        "bypass foundation authority",
        "ignore the foundation",
        "violate the foundation",
        "auto-update foundation",
        "automatic foundation update",
        "foundation drift",
        "skip foundation validation",
    ];

    if FORBIDDEN_PHRASES.iter().any(|p| hay.contains(p)) {
        return false;
    }

    if hay.contains("foundation v") && !hay.contains("v1.0.0") {
        return false;
    }

    if hay.contains("foundation version") && !hay.contains("1.0.0") {
        return false;
    }

    true
}
