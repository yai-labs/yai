use crate::llm::adapter::LlmClient;
use crate::providers::embeddings::{EmbeddingProvider, ProviderError};
use crate::core::runtime::IceError;

#[allow(dead_code)]
pub struct AiClient {
    pub embeddings: Box<dyn EmbeddingProvider>,
    pub llm: Box<dyn LlmClient>,
}

#[allow(dead_code)]
impl AiClient {
    pub fn new(embeddings: Box<dyn EmbeddingProvider>, llm: Box<dyn LlmClient>) -> Self {
        Self { embeddings, llm }
    }

    pub fn embed(&self, input: &str) -> Result<Vec<f32>, ProviderError> {
        self.embeddings.embed(input)
    }

    pub fn complete(&self, prompt: &str) -> Result<String, IceError> {
        self.llm.complete(prompt)
    }
}
