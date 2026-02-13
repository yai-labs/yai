use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct WorkspaceLayout {
    root: PathBuf,
}

impl WorkspaceLayout {
    /// Default layout (~/.yai)
    pub fn default() -> Self {
        let root = env::var("YAI_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = dirs::home_dir().expect("cannot resolve home directory");
                home.join(".yai")
            });

        Self { root }
    }

    /// Custom layout
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn run_dir(&self) -> PathBuf {
        self.root.join("run")
    }

    pub fn run_ws(&self, ws: &str) -> PathBuf {
        self.run_dir().join(ws)
    }

    pub fn semantic_db(&self, ws: &str) -> PathBuf {
        self.run_ws(ws).join("semantic.sqlite")
    }

    pub fn activation_db(&self, ws: &str) -> PathBuf {
        self.run_ws(ws).join("activation.sqlite")
    }

    pub fn knowledge_db(&self) -> PathBuf {
        self.root.join("Data").join("knowledge.db")
    }

    pub fn model_root(&self) -> PathBuf {
        self.root.join("models")
    }

    pub fn embeddings_model_dir(&self, model: &str) -> PathBuf {
        self.model_root().join("embeddings").join(model)
    }
}
