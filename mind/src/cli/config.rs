use crate::cli::paths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigFile {
    pub workspace_root: Option<String>,
    pub artifacts_root: Option<String>,
    pub ws_default: Option<String>,
    pub socket_path: Option<String>,
    pub binaries: Option<BinariesFile>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BinariesFile {
    pub yai_boot: Option<String>,
    pub yai_kernel: Option<String>,
    pub yai_engine: Option<String>,
    pub yai_mind: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub workspace_root: PathBuf,
    pub artifacts_root: PathBuf,
    pub ws_default: String,
    pub socket_path: PathBuf,
    pub yai_boot: PathBuf,
    pub yai_kernel: PathBuf,
    pub yai_engine: PathBuf,
    pub yai_mind: PathBuf,
    pub config_path: PathBuf,
    pub run_dir: PathBuf,
    pub logs_dir: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct CliOverrides {
    pub workspace_root: Option<String>,
    pub artifacts_root: Option<String>,
    pub ws: Option<String>,
    pub socket_path: Option<String>,
    pub yai_boot: Option<String>,
    pub yai_kernel: Option<String>,
    pub yai_engine: Option<String>,
    pub yai_mind: Option<String>,
}

fn env_opt(key: &str) -> Option<String> {
    env::var(key).ok().filter(|v| !v.is_empty())
}

fn load_or_init_config(path: &Path) -> Result<ConfigFile> {
    if path.exists() {
        let content =
            fs::read_to_string(path).with_context(|| format!("read config: {}", path.display()))?;
        let cfg: ConfigFile = toml::from_str(&content).context("parse yai.toml")?;
        return Ok(cfg);
    }

    let default = default_config_file();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create config dir: {}", parent.display()))?;
    }
    let rendered = toml::to_string_pretty(&default).context("serialize default config")?;
    fs::write(path, rendered).with_context(|| format!("write config: {}", path.display()))?;
    Ok(default)
}

fn default_config_file() -> ConfigFile {
    let workspace_root = paths::default_workspace_root();
    let artifacts_root = paths::default_artifacts_root();
    let binaries = BinariesFile {
        yai_boot: Some(
            artifacts_root
                .join("yai-core/bin/yai-boot")
                .display()
                .to_string(),
        ),
        yai_kernel: Some(
            artifacts_root
                .join("yai-core/bin/yai-kernel")
                .display()
                .to_string(),
        ),
        yai_engine: Some(
            artifacts_root
                .join("yai-core/bin/yai-engine")
                .display()
                .to_string(),
        ),
        yai_mind: Some(
            artifacts_root
                .join("mind/target/release/yai-mind")
                .display()
                .to_string(),
        ),
    };

    ConfigFile {
        workspace_root: Some(workspace_root.display().to_string()),
        artifacts_root: Some(artifacts_root.display().to_string()),
        ws_default: Some("dev".to_string()),
        socket_path: Some("/tmp/yai_runtime.sock".to_string()),
        binaries: Some(binaries),
    }
}

pub fn load_config(overrides: &CliOverrides) -> Result<RuntimeConfig> {
    let cfg_path = paths::config_dir().join("yai.toml");
    let cfg_file = load_or_init_config(&cfg_path)?;

    let workspace_root = overrides
        .workspace_root
        .clone()
        .or_else(|| env_opt("YAI_WORKSPACE_ROOT"))
        .or_else(|| cfg_file.workspace_root.clone())
        .unwrap_or_else(|| paths::default_workspace_root().display().to_string());

    let artifacts_root = overrides
        .artifacts_root
        .clone()
        .or_else(|| env_opt("YAI_ARTIFACTS_ROOT"))
        .or_else(|| cfg_file.artifacts_root.clone())
        .unwrap_or_else(|| paths::default_artifacts_root().display().to_string());

    let ws_default = overrides
        .ws
        .clone()
        .or_else(|| env_opt("YAI_WS_DEFAULT"))
        .or_else(|| cfg_file.ws_default.clone())
        .unwrap_or_else(|| "dev".to_string());

    let socket_path = overrides
        .socket_path
        .clone()
        .or_else(|| env_opt("YAI_SOCKET_PATH"))
        .or_else(|| cfg_file.socket_path.clone())
        .unwrap_or_else(|| "/tmp/yai_runtime.sock".to_string());

    let bin_cfg = cfg_file.binaries.clone().unwrap_or_default();

    let yai_boot = overrides
        .yai_boot
        .clone()
        .or_else(|| env_opt("YAI_YAI_BOOT"))
        .or_else(|| bin_cfg.yai_boot)
        .unwrap_or_else(|| {
            paths::default_artifacts_root()
                .join("yai-core/bin/yai-boot")
                .display()
                .to_string()
        });

    let yai_kernel = overrides
        .yai_kernel
        .clone()
        .or_else(|| env_opt("YAI_YAI_KERNEL"))
        .or_else(|| bin_cfg.yai_kernel)
        .unwrap_or_else(|| {
            paths::default_artifacts_root()
                .join("yai-core/bin/yai-kernel")
                .display()
                .to_string()
        });

    let yai_engine = overrides
        .yai_engine
        .clone()
        .or_else(|| env_opt("YAI_YAI_ENGINE"))
        .or_else(|| bin_cfg.yai_engine)
        .unwrap_or_else(|| {
            paths::default_artifacts_root()
                .join("yai-core/bin/yai-engine")
                .display()
                .to_string()
        });

    let yai_mind = overrides
        .yai_mind
        .clone()
        .or_else(|| env_opt("YAI_YAI_MIND"))
        .or_else(|| bin_cfg.yai_mind)
        .unwrap_or_else(|| {
            paths::default_artifacts_root()
                .join("mind/target/release/yai-mind")
                .display()
                .to_string()
        });

    let runtime = RuntimeConfig {
        workspace_root: paths::expand_tilde(&workspace_root),
        artifacts_root: paths::expand_tilde(&artifacts_root),
        ws_default,
        socket_path: paths::expand_tilde(&socket_path),
        yai_boot: paths::expand_tilde(&yai_boot),
        yai_kernel: paths::expand_tilde(&yai_kernel),
        yai_engine: paths::expand_tilde(&yai_engine),
        yai_mind: paths::expand_tilde(&yai_mind),
        config_path: cfg_path,
        run_dir: paths::run_dir(),
        logs_dir: paths::logs_dir(),
    };

    fs::create_dir_all(&runtime.run_dir)
        .with_context(|| format!("create run dir: {}", runtime.run_dir.display()))?;
    fs::create_dir_all(&runtime.logs_dir)
        .with_context(|| format!("create logs dir: {}", runtime.logs_dir.display()))?;

    Ok(runtime)
}
