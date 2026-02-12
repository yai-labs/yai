use std::env;
use std::path::{Path, PathBuf};

pub fn home_dir() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn expand_tilde(path: &str) -> PathBuf {
    if path == "~" {
        return home_dir();
    }
    if let Some(rest) = path.strip_prefix("~/") {
        return home_dir().join(rest);
    }
    PathBuf::from(path)
}

pub fn default_workspace_root() -> PathBuf {
    home_dir().join("Developer").join("YAI").join("yai")
}

pub fn default_artifacts_root() -> PathBuf {
    home_dir().join(".yai").join("artifacts")
}

pub fn config_dir() -> PathBuf {
    home_dir().join(".yai").join("config")
}

pub fn run_dir() -> PathBuf {
    home_dir().join(".yai").join("run")
}

pub fn logs_dir() -> PathBuf {
    home_dir().join(".yai").join("logs")
}

pub fn trust_dir() -> PathBuf {
    home_dir().join(".yai").join("trust")
}

pub fn model_root() -> PathBuf {
    home_dir().join(".yai").join("models")
}

pub fn repo_root(workspace_root: &Path) -> PathBuf {
    let candidates = [
        workspace_root.to_path_buf(),
        workspace_root.join("yai"),
        workspace_root.join("yai-core"),
    ];
    for c in &candidates {
        if c.join("kernel").is_dir() && c.join("engine").is_dir() && c.join("mind").is_dir() {
            return c.clone();
        }
    }
    for c in &candidates {
        if c.join("kernel").is_dir() && c.join("engine").is_dir() {
            return c.clone();
        }
    }
    workspace_root.to_path_buf()
}

pub fn core_dir(workspace_root: &Path) -> PathBuf {
    repo_root(workspace_root)
}

pub fn mind_dir(workspace_root: &Path) -> PathBuf {
    let root = repo_root(workspace_root);
    if root.join("mind").is_dir() {
        return root.join("mind");
    }
    if root.join("yai-mind").is_dir() {
        return root.join("yai-mind");
    }
    root.join("mind")
}

pub fn law_dir(workspace_root: &Path) -> PathBuf {
    let root = repo_root(workspace_root);
    if root.join("law").is_dir() {
        return root.join("law");
    }
    root.join("yai-core").join("law")
}

pub fn ws_socket_path(base: &PathBuf, ws: &str) -> PathBuf {
    let base_str = base.display().to_string();
    if base_str.contains("{ws}") {
        return PathBuf::from(base_str.replace("{ws}", ws));
    }
    if base_str.ends_with(".sock") {
        let trimmed = base_str.trim_end_matches(".sock");
        return PathBuf::from(format!("{}_{}.sock", trimmed, ws));
    }
    PathBuf::from(format!("{}/yai_runtime_{}.sock", base_str, ws))
}
