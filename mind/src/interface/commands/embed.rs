use crate::interface::config::RuntimeConfig;
use crate::providers::embeddings::{EmbeddingProvider, HashEmbedder, RemoteEmbedder};
use anyhow::Result;
use std::env;

pub fn run(_cfg: &RuntimeConfig, provider: &str, model: &str, endpoint: &Option<String>, text: &str) -> Result<()> {
    let mut embedder: Box<dyn EmbeddingProvider> = Box::new(HashEmbedder::new(16));
    let mut provider = provider.to_lowercase();
    let env_provider = env::var("YAI_EMBED_PROVIDER").ok();
    if provider == "hash" {
        if let Some(p) = env_provider {
            provider = p.to_lowercase();
        }
    }
    let env_endpoint = env::var("YAI_EMBED_ENDPOINT").ok();
    let env_model = env::var("YAI_EMBED_MODEL").ok();
    let resolved_model = env_model.unwrap_or_else(|| model.to_string());
    let resolved_endpoint = endpoint.clone().or(env_endpoint);

    if provider == "remote" {
        let ep = resolved_endpoint.ok_or_else(|| anyhow::anyhow!("missing --endpoint or YAI_EMBED_ENDPOINT"))?;
        embedder = Box::new(RemoteEmbedder::new(ep.clone(), resolved_model.clone()));
        println!("provider: remote");
        println!("model: {}", resolved_model);
        println!("endpoint: {}", ep);
    } else {
        let mut selected = false;
        #[cfg(feature = "embeddings-onnx")]
        {
            let model_dir = crate::providers::embeddings::onnx::default_model_dir(&resolved_model);
            if provider == "onnx" || (provider == "hash" && model_dir.exists()) {
                let emb = crate::providers::embeddings::onnx::OnnxEmbedder::load(&model_dir)?;
                embedder = Box::new(emb);
                println!("provider: onnx");
                println!("model: {}", resolved_model);
                println!("path: {}", model_dir.display());
                selected = true;
            }
        }

        #[cfg(feature = "embeddings-candle")]
        if !selected {
            let model_dir = crate::providers::embeddings::candle::default_model_dir(&resolved_model);
            if provider == "candle" || (provider == "hash" && model_dir.exists()) {
                let emb = crate::providers::embeddings::candle::CandleEmbedder::load(&model_dir)?;
                embedder = Box::new(emb);
                println!("provider: candle");
                println!("model: {}", resolved_model);
                println!("path: {}", model_dir.display());
                selected = true;
            }
        }

        if !selected {
            embedder = Box::new(HashEmbedder::new(16));
            println!("provider: hash (fallback)");
            #[cfg(not(feature = "embeddings-onnx"))]
            println!("note: onnx disabled (build with --features embeddings-onnx)");
            #[cfg(not(feature = "embeddings-candle"))]
            println!("note: candle disabled (build with --features embeddings-candle)");
        }
    }

    let v = embedder.embed(text)?;
    println!("dim: {}", embedder.dim());
    let preview: Vec<String> = v.iter().take(8).map(|f| format!("{:.4}", f)).collect();
    println!("preview: [{}]", preview.join(", "));
    Ok(())
}
