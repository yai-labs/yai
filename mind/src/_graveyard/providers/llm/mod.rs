pub mod legacy;

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct LlmRequest {
    pub prompt: String,
}

#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub text: String,
}

/// Port: LLM backend.
pub trait LlmProvider {
    fn complete(&self, req: LlmRequest) -> Result<LlmResponse>;
}

/// Temporary adapter: uses legacy implementation.
pub struct LegacyLlm;

impl LlmProvider for LegacyLlm {
    fn complete(&self, req: LlmRequest) -> Result<LlmResponse> {
        // per ora stub: dopo colleghiamo legacy::adapter
        Ok(LlmResponse { text: req.prompt })
    }
}
