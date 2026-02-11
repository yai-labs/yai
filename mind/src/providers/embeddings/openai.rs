use super::base::{EmbeddingProvider, ProviderError};
use crate::providers::ProviderConfig;

pub struct OpenAiEmbeddingProvider {
    cfg: ProviderConfig,
}

impl OpenAiEmbeddingProvider {
    pub fn new(cfg: ProviderConfig) -> Self {
        Self { cfg }
    }
}

impl EmbeddingProvider for OpenAiEmbeddingProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn embed(&self, _input: &str) -> Result<Vec<f32>, ProviderError> {
        if self.cfg.api_key.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            return Err(ProviderError::NotConfigured);
        }
        Err(ProviderError::NotImplemented)
    }
}
