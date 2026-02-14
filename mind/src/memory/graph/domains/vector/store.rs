use crate::memory::graph::domains::vector::types::VectorEntry;
use crate::workspace::layout::WorkspaceLayout;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct VectorStore {
    path: PathBuf,
    entries: Vec<VectorEntry>,
}

impl VectorStore {
    pub fn open(layout: &WorkspaceLayout) -> Result<Self> {
        let path = layout.vector_store();
        let entries = if path.exists() {
            let raw = fs::read_to_string(&path).context("read vector store")?;
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
        fs::write(&self.path, raw).context("write vector store")?;
        Ok(())
    }
}
