use anyhow::Result;

#[derive(Clone, Debug)]
pub struct ProviderRequest {
    pub provider: String,
    pub model: String,
    pub payload: serde_json::Value,
}

#[derive(Clone, Debug)]
pub struct ProviderResponse {
    pub payload: serde_json::Value,
}

pub trait ProviderClient: Send + Sync {
    fn call(&self, req: ProviderRequest) -> Result<ProviderResponse>;
}
