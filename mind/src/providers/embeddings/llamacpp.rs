use super::base::{EmbeddingProvider, ProviderError};
use crate::providers::ProviderConfig;

pub struct LlamaCppEmbeddingProvider {
    _cfg: ProviderConfig,
}

impl LlamaCppEmbeddingProvider {
    pub fn new(cfg: ProviderConfig) -> Self {
        Self { _cfg: cfg }
    }
}

impl EmbeddingProvider for LlamaCppEmbeddingProvider {
    fn name(&self) -> &'static str {
        "llamacpp"
    }

    fn embed(&self, _input: &str) -> Result<Vec<f32>, ProviderError> {
        Err(ProviderError::NotImplemented)
    }
}
