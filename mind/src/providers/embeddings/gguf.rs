use super::base::{EmbeddingProvider, ProviderError};
use crate::providers::ProviderConfig;

pub struct GgufEmbeddingProvider {
    _cfg: ProviderConfig,
}

impl GgufEmbeddingProvider {
    pub fn new(cfg: ProviderConfig) -> Self {
        Self { _cfg: cfg }
    }
}

impl EmbeddingProvider for GgufEmbeddingProvider {
    fn name(&self) -> &'static str {
        "gguf"
    }

    fn embed(&self, _input: &str) -> Result<Vec<f32>, ProviderError> {
        Err(ProviderError::NotImplemented)
    }
}
