// Temporary compat layer (only).
// NOTE: Real providers live under crate::cognition::providers.
// Keep only adapters here until full migration completes.
pub mod adapters;

pub mod embeddings {
    pub use crate::cognition::providers::embeddings::*;
}
pub mod llm {
    pub use crate::cognition::providers::llm::*;
}
