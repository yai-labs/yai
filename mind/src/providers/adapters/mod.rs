use serde_json::{json, Value};
use std::io::{BufRead, BufReader};
use std::time::Duration;

pub trait LlmClient: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String, LlmError>;

    fn complete_stream(
        &self,
        prompt: &str,
        on_delta: &mut dyn FnMut(&str),
    ) -> Result<String, LlmError> {
        let text = self.complete(prompt)?;
        on_delta(&text);
        Ok(text)
    }
}

#[derive(Debug)]
pub enum LlmError {
    Http(String),
    Parse(String),
    Other(String),
}

pub struct MockLlmClient;

impl LlmClient for MockLlmClient {
    fn complete(&self, prompt: &str) -> Result<String, LlmError> {
        Ok(format!("[mock-llm] {}", prompt))
    }
}

pub struct HttpLlmClient {
    pub endpoint: String,
    pub model: String,
    pub api_key: Option<String>,
    pub timeout_ms: u64,
}

impl LlmClient for HttpLlmClient {
    fn complete(&self, prompt: &str) -> Result<String, LlmError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(self.timeout_ms))
            .build()
            .map_err(|e: reqwest::Error| LlmError::Http(e.to_string()))?;


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

        let resp = req.send().map_err(|e: reqwest::Error| LlmError::Http(e.to_string()))?;


        if !resp.status().is_success() {
            return Err(LlmError::Http(format!("status {}", resp.status().as_u16())));
        }

        let value: serde_json::Value =
             resp.json().map_err(|e: reqwest::Error| LlmError::Parse(e.to_string()))?;


        let content = value["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

    }
}
