use crate::cognition::memory::graph::authority::types::AuthorityPolicy;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct AuthorityStore {
    path: PathBuf,
}

impl AuthorityStore {
    /// Il dominio NON decide dove sta il file.
    /// Il path viene iniettato dal layer superiore.
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Vec<AuthorityPolicy>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }

        let raw = fs::read_to_string(&self.path)
            .context("read authority.json")?;

        let data: Vec<AuthorityPolicy> =
            serde_json::from_str(&raw)
                .context("parse authority.json")?;

        Ok(data)
    }
}
