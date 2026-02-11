use anyhow::Result;
use serde::Deserialize;
use std::sync::Mutex;

pub type ProviderError = anyhow::Error;

pub trait EmbeddingProvider {
    fn dim(&self) -> usize;
    fn embed(&self, text: &str) -> Result<Vec<f32>, ProviderError>;
}

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

#[derive(Debug)]
pub struct RemoteEmbedder {
    endpoint: String,
    model: String,
    dim: Mutex<Option<usize>>,
}

#[derive(Debug, Deserialize)]
struct RemoteEmbeddingResponse {
    data: Vec<RemoteEmbeddingItem>,
}

#[derive(Debug, Deserialize)]
struct RemoteEmbeddingItem {
    embedding: Vec<f32>,
}

impl RemoteEmbedder {
    pub fn new(endpoint: String, model: String) -> Self {
        Self {
            endpoint,
            model,
            dim: Mutex::new(None),
        }
    }
}

impl EmbeddingProvider for RemoteEmbedder {
    fn dim(&self) -> usize {
        self.dim.lock().ok().and_then(|d| *d).unwrap_or(0)
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>, ProviderError> {
        let client = reqwest::blocking::Client::new();
        let payload = serde_json::json!({
            "model": self.model,
            "input": text,
        });
        let resp = client.post(&self.endpoint).json(&payload).send()?;
        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("remote embed failed: {}", resp.status()));
        }
        let body: RemoteEmbeddingResponse = resp.json()?;
        let embedding = body
            .data
            .get(0)
            .map(|item| item.embedding.clone())
            .ok_or_else(|| anyhow::anyhow!("remote embed: empty response"))?;
        if let Ok(mut dim) = self.dim.lock() {
            *dim = Some(embedding.len());
        }
        Ok(embedding)
    }
}

pub fn build_from_env() -> Result<(Box<dyn EmbeddingProvider>, String)> {
    let provider = std::env::var("YAI_EMBED_PROVIDER").unwrap_or_else(|_| "hash".to_string());
    if provider == "remote" {
        let endpoint = std::env::var("YAI_EMBED_ENDPOINT")
            .map_err(|_| anyhow::anyhow!("YAI_EMBED_ENDPOINT missing"))?;
        let model = std::env::var("YAI_EMBED_MODEL").unwrap_or_else(|_| "mini".to_string());
        return Ok((Box::new(RemoteEmbedder::new(endpoint, model)), "remote".to_string()));
    }

    #[cfg(feature = "embeddings-onnx")]
    {
        let model = std::env::var("YAI_EMBED_MODEL").unwrap_or_else(|_| "all-MiniLM-L6-v2".to_string());
        let model_dir = crate::providers::embeddings::onnx::default_model_dir(&model);
        if provider == "onnx" || (provider == "hash" && model_dir.exists()) {
            let emb = crate::providers::embeddings::onnx::OnnxEmbedder::load(&model_dir)?;
            return Ok((Box::new(emb), "onnx".to_string()));
        }
    }

    Ok((Box::new(HashEmbedder::new(16)), "hash".to_string()))
}

#[cfg(feature = "embeddings-onnx")]
pub mod onnx;

#[cfg(feature = "embeddings-candle")]
pub mod candle;
