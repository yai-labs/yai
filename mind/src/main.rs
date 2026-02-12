use std::env;
use std::sync::{Arc, Mutex};


use std::path::PathBuf;

fn default_artifacts_root() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".yai").join("artifacts")
}

fn read_core_manifest() -> Option<(String, String)> {
    let artifacts_root = env::var("YAI_ARTIFACTS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| default_artifacts_root());
    let manifest = artifacts_root.join("yai-core").join("dist").join("MANIFEST.json");
    let data = std::fs::read_to_string(manifest).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    let git_sha = v.get("git_sha")?.as_str()?.to_string();
    let build_time = v.get("build_time")?.as_str()?.to_string();
    Some((git_sha, build_time))
}
use yai_mind::core::state::SharedState;
use yai_mind::server::StudioServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("YAI Core API v2.5.0 (Rust) - Foundation 1.0.0");
    if let Some((git_sha, build_time)) = read_core_manifest() {
        println!("🧩 YAI-MIND build: {} {}", git_sha, build_time);
    } else {
        println!("🧩 YAI-MIND build: unknown");
    }

    let host = env::var("YAI_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("YAI_API_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(8081);

    let endpoint = env::var("YAI_REMOTE_ENDPOINT").unwrap_or_default();
    let model = env::var("YAI_REMOTE_MODEL").unwrap_or_else(|_| "unknown".to_string());
    if !endpoint.is_empty() {
        println!("[LLM] provider=remote endpoint={} model={}", endpoint, model);
    } else {
        println!("[LLM] provider=mock");
    }

    let state = Arc::new(Mutex::new(SharedState::new()));
    let server = StudioServer::new(&host, port, state);
    server.start().await?;

    Ok(())
}
