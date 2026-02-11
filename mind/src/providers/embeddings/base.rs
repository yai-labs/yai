use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ProviderError {
    NotConfigured,
    Transport(String),
    Provider(String),
    NotImplemented,
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderError::NotConfigured => write!(f, "provider not configured"),
            ProviderError::Transport(msg) => write!(f, "transport: {}", msg),
            ProviderError::Provider(msg) => write!(f, "provider: {}", msg),
            ProviderError::NotImplemented => write!(f, "not implemented"),
        }
    }
}

impl Error for ProviderError {}

pub trait EmbeddingProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn embed(&self, input: &str) -> Result<Vec<f32>, ProviderError>;

    fn embed_batch(&self, inputs: &[String]) -> Result<Vec<Vec<f32>>, ProviderError> {
        inputs.iter().map(|s| self.embed(s)).collect()
    }
}
