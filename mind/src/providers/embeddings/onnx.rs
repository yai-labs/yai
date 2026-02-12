use super::EmbeddingProvider;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokenizers::Tokenizer;
use tract_onnx::prelude::*;

pub struct OnnxEmbedder {
    tokenizer: Tokenizer,
    model: TypedSimplePlan<TypedModel>,
    input_count: usize,
    dim: AtomicUsize,
}

pub fn default_model_dir(model: &str) -> PathBuf {
    crate::cli::paths::model_root()
        .join("embeddings")
        .join(model)
}

impl OnnxEmbedder {
    pub fn load(model_dir: &Path) -> Result<Self> {
        let model_path = model_dir.join("model.onnx");
        let tok_path = model_dir.join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(&tok_path)
            .map_err(|e| anyhow::anyhow!("load tokenizer: {}", e))?;

        let model = tract_onnx::onnx()
            .model_for_path(&model_path)
            .with_context(|| format!("load onnx: {}", model_path.display()))?;
        let input_count = model.input_outlets()?.len();
        let typed = model.into_optimized()?;
        let dim = infer_dim_from_model(&typed).unwrap_or(0);
        let model: TypedSimplePlan<TypedModel> = typed.into_runnable()?;

        Ok(Self {
            tokenizer,
            model,
            input_count,
            dim: AtomicUsize::new(dim),
        })
    }

    fn embed_inner(&self, text: &str) -> Result<Vec<f32>> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| anyhow::anyhow!("tokenize: {}", e))?;

        let ids: Vec<i64> = encoding.get_ids().iter().map(|v| *v as i64).collect();
        let attn: Vec<i64> = encoding
            .get_attention_mask()
            .iter()
            .map(|v| *v as i64)
            .collect();
        let token_type: Vec<i64> = encoding.get_type_ids().iter().map(|v| *v as i64).collect();

        let seq_len = ids.len();
        let ids = Tensor::from(tract_ndarray::Array2::from_shape_vec((1, seq_len), ids)?);
        let attn = Tensor::from(tract_ndarray::Array2::from_shape_vec((1, seq_len), attn)?);
        let token_type = Tensor::from(tract_ndarray::Array2::from_shape_vec(
            (1, seq_len),
            token_type,
        )?);

        let inputs = match self.input_count {
            1 => tvec![ids],
            2 => tvec![ids, attn],
            _ => tvec![ids, attn, token_type],
        };

        let outputs = self
            .model
            .run(inputs.into_iter().map(|t| t.into_tvalue()).collect())?;
        let out = outputs
            .first()
            .ok_or_else(|| anyhow::anyhow!("onnx embed: empty output"))?;

        if out.rank() == 2 {
            let arr = out.to_array_view::<f32>()?;
            let row = arr.index_axis(tract_ndarray::Axis(0), 0);
            return Ok(row.to_owned().into_raw_vec());
        }

        if out.rank() == 3 {
            let arr = out.to_array_view::<f32>()?;
            let tokens = arr.index_axis(tract_ndarray::Axis(0), 0);
            let mut sum = vec![0f32; tokens.shape()[1]];
            let mut count = 0f32;
            let mask = encoding.get_attention_mask();
            for (i, tok) in tokens.outer_iter().enumerate() {
                if i < mask.len() && mask[i] == 0 {
                    continue;
                }
                for (j, v) in tok.iter().enumerate() {
                    sum[j] += *v;
                }
                count += 1.0;
            }
            if count > 0.0 {
                for v in &mut sum {
                    *v /= count;
                }
            }
            return Ok(sum);
        }

        Err(anyhow::anyhow!(
            "onnx embed: unexpected output rank {}",
            out.rank()
        ))
    }
}

impl EmbeddingProvider for OnnxEmbedder {
    fn dim(&self) -> usize {
        self.dim.load(Ordering::Relaxed)
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>, super::ProviderError> {
        let v = self.embed_inner(text)?;
        if self.dim.load(Ordering::Relaxed) == 0 {
            let _ = self
                .dim
                .compare_exchange(0, v.len(), Ordering::Relaxed, Ordering::Relaxed);
        }
        Ok(v)
    }
}

fn infer_dim_from_model(model: &TypedModel) -> Option<usize> {
    let outlet = model.output_outlets().ok()?.get(0)?;
    let fact = model.outlet_fact(*outlet).ok()?;
    let shape = fact.shape.as_concrete()?;
    if shape.len() == 2 {
        return shape.get(1).cloned();
    }
    if shape.len() == 3 {
        return shape.get(2).cloned();
    }
    None
}
