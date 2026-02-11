use super::base::{EmbeddingProvider, ProviderError};

pub struct MockEmbeddingProvider {
    dims: usize,
}

impl MockEmbeddingProvider {
    pub fn new(dims: usize) -> Self {
        Self { dims }
    }

    fn hash_bytes(input: &str) -> u64 {
        // FNV-1a 64-bit
        let mut hash: u64 = 0xcbf29ce484222325;
        for b in input.as_bytes() {
            hash ^= *b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
}

impl Default for MockEmbeddingProvider {
    fn default() -> Self {
        Self { dims: 8 }
    }
}

impl EmbeddingProvider for MockEmbeddingProvider {
    fn name(&self) -> &'static str {
        "mock"
    }

    fn embed(&self, input: &str) -> Result<Vec<f32>, ProviderError> {
        let mut out = Vec::with_capacity(self.dims);
        let mut h = Self::hash_bytes(input);
        for i in 0..self.dims {
            let byte = ((h >> ((i % 8) * 8)) & 0xFF) as f32;
            out.push(byte / 255.0);
            h = h.rotate_left(5) ^ (i as u64 + 1);
        }
        Ok(out)
    }
}
