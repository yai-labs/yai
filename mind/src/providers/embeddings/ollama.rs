use super::base::{EmbeddingProvider, ProviderError};
use crate::providers::ProviderConfig;

pub struct OllamaEmbeddingProvider {
    _cfg: ProviderConfig,
}

impl OllamaEmbeddingProvider {
    pub fn new(cfg: ProviderConfig) -> Self {
        Self { _cfg: cfg }
    }
}

impl EmbeddingProvider for OllamaEmbeddingProvider {
    fn name(&self) -> &'static str {
        "ollama"
    }

    fn embed(&self, _input: &str) -> Result<Vec<f32>, ProviderError> {
        Err(ProviderError::NotImplemented)
    }
}
