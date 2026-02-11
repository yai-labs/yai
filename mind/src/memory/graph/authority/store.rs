use crate::interface::paths;
use crate::memory::graph::authority::types::AuthorityPolicy;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

pub struct AuthorityStore {
    path: PathBuf,
}

impl AuthorityStore {
    pub fn open(_ws: &str) -> Result<Self> {
        let workspace_root = env::var("YAI_WORKSPACE_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| paths::default_workspace_root());
        let path = paths::law_dir(&workspace_root)
            .join("specs")
            .join("control")
            .join("authority.json");
        Ok(Self { path })
    }

    pub fn load(&self) -> Result<Vec<AuthorityPolicy>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let raw = fs::read_to_string(&self.path).context("read authority.json")?;
        let data = serde_json::from_str(&raw).unwrap_or_default();
        Ok(data)
    }
}
