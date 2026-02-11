use crate::core::runtime::IceError;
use crate::control::providers;
use crate::interface::config::RuntimeConfig;
use serde_json::{json, Value};
use std::env;
use std::time::Duration;

#[allow(dead_code)]
pub trait LlmClient: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String, IceError>;
}

pub struct MockLlmClient;

impl LlmClient for MockLlmClient {
    fn complete(&self, prompt: &str) -> Result<String, IceError> {
        Ok(format!("[mock-llm] {}", prompt))
    }
}

pub struct HttpLlmClient {
    endpoint: String,
    model: String,
    api_key: Option<String>,
    timeout_ms: u64,
}

impl HttpLlmClient {
    pub fn from_env() -> Option<Self> {
        let endpoint = env::var("YAI_REMOTE_ENDPOINT").ok()?;
        if endpoint.trim().is_empty() {
            return None;
        }
        let model = env::var("YAI_REMOTE_MODEL").unwrap_or_else(|_| "unknown".to_string());
        let api_key = env::var("YAI_REMOTE_API_KEY").ok();
        let timeout_ms = env::var("YAI_REMOTE_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(15_000);
        Some(Self {
            endpoint,
            model,
            api_key,
            timeout_ms,
        })
    }

    pub fn from_provider(endpoint: String, model: String) -> Option<Self> {
        if endpoint.trim().is_empty() {
            return None;
        }
        let api_key = env::var("YAI_REMOTE_API_KEY").ok();
        let timeout_ms = env::var("YAI_REMOTE_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(15_000);
        Some(Self {
            endpoint,
            model,
            api_key,
            timeout_ms,
        })
    }

    fn parse_response(value: Value) -> Result<String, IceError> {
        if let Some(content) = value
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c0| c0.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
        {
            return Ok(content.trim().to_string());
        }

        if let Some(text) = value
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c0| c0.get("text"))
            .and_then(|t| t.as_str())
        {
            return Ok(text.trim().to_string());
        }

        Err(IceError::Llm("invalid LLM response".to_string()))
    }
}

impl LlmClient for HttpLlmClient {
    fn complete(&self, prompt: &str) -> Result<String, IceError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(self.timeout_ms))
            .build()
            .map_err(|e| IceError::Llm(format!("http client error: {}", e)))?;

        let body = json!({
            "model": self.model,
            "messages": [
                { "role": "user", "content": prompt }
            ],
            "temperature": 0.2,
            "stream": false
        });

        let mut req = client.post(&self.endpoint).json(&body);
        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }

        let resp = req
            .send()
            .map_err(|e| IceError::Llm(format!("http send error: {}", e)))?;

        let status = resp.status();
        let value: Value = resp
            .json()
            .map_err(|e| IceError::Llm(format!("http json error: {}", e)))?;

        if !status.is_success() {
            return Err(IceError::Llm(format!(
                "llm http status {}",
                status.as_u16()
            )));
        }

        Self::parse_response(value)
    }
}

pub fn build_llm_from_env() -> Box<dyn LlmClient> {
    if let Some(client) = HttpLlmClient::from_env() {
        if env::var("YAI_AI_LOG").ok().as_deref() == Some("1") {
            println!(
                "[LLM] provider=remote endpoint={} model={} timeout_ms={}",
                client.endpoint, client.model, client.timeout_ms
            );
        }
        Box::new(client)
    } else {
        if env::var("YAI_AI_LOG").ok().as_deref() == Some("1") {
            println!("[LLM] provider=mock");
        }
        Box::new(MockLlmClient)
    }
}

pub fn build_llm_for_ws(cfg: &RuntimeConfig, ws: &str) -> Box<dyn LlmClient> {
    if let Ok(Some(info)) = providers::status(&cfg.run_dir, ws) {
        if let Some(client) = HttpLlmClient::from_provider(info.endpoint, info.model) {
            if env::var("YAI_AI_LOG").ok().as_deref() == Some("1") {
                println!(
                    "[LLM] provider=attached endpoint={} model={} timeout_ms={}",
                    client.endpoint, client.model, client.timeout_ms
                );
            }
            return Box::new(client);
        }
    }
    build_llm_from_env()
}
