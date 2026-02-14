use anyhow::Result;

pub type ProviderError = anyhow::Error;

#[derive(Clone, Debug)]
pub enum ProviderKind {
    Embeddings,
    Llm,
}

#[derive(Clone, Debug)]
pub struct ProviderId(pub String);

#[derive(Clone, Debug)]
pub struct ModelId(pub String);

pub trait EmbeddingProvider: Send + Sync {
    fn dim(&self) -> usize;
    fn embed(&self, text: &str) -> Result<Vec<f32>, ProviderError>;
}

pub trait LlmProvider: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String, ProviderError>;
}
