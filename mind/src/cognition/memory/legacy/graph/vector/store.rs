use crate::cli::paths;
use crate::cognition::memory::graph::vector::types::VectorEntry;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct VectorStore {
    path: PathBuf,
    entries: Vec<VectorEntry>,
}

impl VectorStore {
    pub fn open(ws: &str) -> Result<Self> {
        let base = paths::run_dir().join(ws);
        fs::create_dir_all(&base).context("create ws dir")?;
        let path = base.join("vector.usearch");
        let entries = if path.exists() {
            let raw = fs::read_to_string(&path).context("read vector.usearch")?;
            serde_json::from_str::<Vec<VectorEntry>>(&raw).unwrap_or_default()
        } else {
            vec![]
        };
        Ok(Self { path, entries })
    }

    pub fn set(&mut self, entries: Vec<VectorEntry>) -> Result<()> {
        self.entries = entries;
        self.save()
    }

    pub fn entries(&self) -> &[VectorEntry] {
        &self.entries
    }

    pub fn save(&self) -> Result<()> {
        let raw = serde_json::to_string_pretty(&self.entries).context("serialize vector")?;
        fs::write(&self.path, raw).context("write vector.usearch")?;
        Ok(())
    }
}
