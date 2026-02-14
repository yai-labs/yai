use crate::providers::types::{EmbeddingProvider, ProviderError};
use anyhow::Result;

pub struct MockEmbedder {
    dim: usize,
    fixed: Vec<f32>,
}

impl MockEmbedder {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            fixed: vec![0.0; dim],
        }
    }

    pub fn with_fixed(vec: Vec<f32>) -> Self {
        let dim = vec.len();
        Self { dim, fixed: vec }
    }
}

impl EmbeddingProvider for MockEmbedder {
    fn dim(&self) -> usize {
        self.dim
    }

    fn embed(&self, _text: &str) -> Result<Vec<f32>, ProviderError> {
        Ok(self.fixed.clone())
    }
}
