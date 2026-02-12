use crate::cli::commands::down;
use crate::cli::commands::up::{self, UpRuntime};
use crate::cli::config::RuntimeConfig;
use crate::cli::paths;
use anyhow::{Context, Result};
use std::process::Command;

pub fn verify_core(cfg: &RuntimeConfig) -> Result<()> {
    let core_dir = paths::core_dir(&cfg.workspace_root);
    let status = Command::new("bash")
        .arg("-lc")
        .arg("./scripts/verify/core.sh")
        .current_dir(&core_dir)
        .status()
        .with_context(|| format!("run verify-core in {}", core_dir.display()))?;
    if !status.success() {
        anyhow::bail!("verify core failed");
    }
    Ok(())
}

pub fn verify_full(cfg: &RuntimeConfig) -> Result<()> {
    verify_core(cfg)?;
    let mind_dir = paths::mind_dir(&cfg.workspace_root);
    let status = Command::new("cargo")
        .arg("test")
        .current_dir(&mind_dir)
        .status()
        .with_context(|| format!("cargo test in {}", mind_dir.display()))?;
    if !status.success() {
        anyhow::bail!("verify full failed: cargo test");
    }
    Ok(())
}

pub fn test_smoke(cfg: &RuntimeConfig, ws: &str, timeout_ms: u64) -> Result<()> {
    let runtime = UpRuntime {
        ws: ws.to_string(),
        monitor: false,
        ai: false,
        build: true,
        no_engine: false,
        no_mind: false,
        detach: true,
        timeout_ms: Some(timeout_ms),
    };
    up::run(cfg, &runtime)?;

    let mind_dir = paths::mind_dir(&cfg.workspace_root);
    let status = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("yai")
        .arg("--")
        .arg("status")
        .arg("--ws")
        .arg(ws)
        .current_dir(&mind_dir)
        .status()
        .with_context(|| format!("run status in {}", mind_dir.display()))?;
    let _ = down::run(cfg, ws, true);

    if !status.success() {
        anyhow::bail!("smoke test failed");
    }
    Ok(())
}
