use crate::providers::types::{EmbeddingProvider, ProviderError};
use anyhow::Result;

pub struct HashEmbedder {
    dim: usize,
}

impl HashEmbedder {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }

    fn hash_embed(&self, text: &str) -> Vec<f32> {
        let mut v = vec![0f32; self.dim];
        for (i, b) in text.bytes().enumerate() {
            let idx = i % self.dim;
            v[idx] += (b as f32) / 255.0;
        }
        v
    }
}

impl EmbeddingProvider for HashEmbedder {
    fn dim(&self) -> usize {
        self.dim
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>, ProviderError> {
        Ok(self.hash_embed(text))
    }
}
