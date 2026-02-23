use crate::providers::embedders::base::HashEmbedder;
use crate::providers::embedders::mock::MockEmbedder;
use crate::providers::types::EmbeddingProvider;
use anyhow::{bail, Result};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ProviderRegistry {
    default_embed_dim: usize,
}

impl ProviderRegistry {
    pub fn new(default_embed_dim: usize) -> Self {
        Self { default_embed_dim }
    }

    pub fn resolve_embedding(&self, id: &str) -> Result<Arc<dyn EmbeddingProvider>> {
        match id {
            // deterministic fallback (sempre disponibile)
            "hash" | "hash_v1" => Ok(Arc::new(HashEmbedder::new(self.default_embed_dim))),

            // mock per test
            "mock" | "mock_v1" => Ok(Arc::new(MockEmbedder::new(self.default_embed_dim))),

            other => bail!("unknown embedding provider id: {}", other),
        }
    }
}
