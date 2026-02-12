use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DType};
use tokenizers::Tokenizer;

use super::EmbeddingProvider;

pub struct CandleEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
    dim: usize,
}

impl CandleEmbedder {
    pub fn load(model_dir: &Path) -> Result<Self> {
        let cfg_path = model_dir.join("config.json");
        let tokenizer_path = model_dir.join("tokenizer.json");
        let weights_path = model_dir.join("model.safetensors");

        let cfg_raw = std::fs::read_to_string(&cfg_path)
            .with_context(|| format!("read config: {}", cfg_path.display()))?;
        let cfg: Config = serde_json::from_str(&cfg_raw).context("parse config")?;

        let tokenizer =
            Tokenizer::from_file(&tokenizer_path).map_err(|e| anyhow::anyhow!("tokenizer: {e}"))?;

        let device = Device::Cpu;
        let vb = VarBuilder::from_safetensors(&weights_path, DType::F32, &device)
            .with_context(|| format!("load weights: {}", weights_path.display()))?;

        let model = BertModel::load(vb, &cfg)?;
        let dim = cfg.hidden_size as usize;

        Ok(Self {
            model,
            tokenizer,
            device,
            dim,
        })
    }
}

impl EmbeddingProvider for CandleEmbedder {
    fn dim(&self) -> usize {
        self.dim
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let enc = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| anyhow::anyhow!("tokenize: {e}"))?;

        let input_ids = Tensor::new(enc.get_ids(), &self.device)?.unsqueeze(0)?;
        let attn = Tensor::new(enc.get_attention_mask(), &self.device)?.unsqueeze(0)?;

        let output = self.model.forward(&input_ids, &attn)?;
        let hidden = output.get(0)?; // [batch, seq, hidden]
        let pooled = hidden.mean(1)?; // simple mean pool
        let v: Vec<f32> = pooled.squeeze(0)?.to_vec1()?;
        Ok(v)
    }
}

pub fn default_model_dir(name: &str) -> PathBuf {
    crate::cli::paths::home_dir()
        .join(".yai")
        .join("models")
        .join("embeddings")
        .join(name)
}
