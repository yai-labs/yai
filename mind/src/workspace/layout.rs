use std::env;
use std::path::PathBuf;

/// Layout interno ad un workspace (multi-tenant).
/// Questo è *solo* il mapping dei path. Niente logica.
#[derive(Clone, Debug)]
pub struct WorkspaceLayout {
    pub ws_id: String,
    pub root: PathBuf,
}

impl WorkspaceLayout {
    /// Costruttore esplicito (usato nei test o in contesti controllati)
    pub fn new(ws_id: impl Into<String>, root: PathBuf) -> Self {
        Self {
            ws_id: ws_id.into(),
            root,
        }
    }

    /// Costruttore standard runtime-safe.
    /// Crea:
    /// .yai/run/<ws_id>
    pub fn default_for(ws_id: impl Into<String>) -> Self {
        let ws = ws_id.into();

        let root = env::current_dir()
            .unwrap()
            .join(".yai")
            .join("run")
            .join(&ws);

        Self::new(ws, root)
    }

    /// Root directory del workspace
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// events.log
    pub fn events_log(&self) -> PathBuf {
        self.root.join("events.log")
    }

    /// semantic.sqlite
    pub fn semantic_sqlite(&self) -> PathBuf {
        self.root.join("semantic.sqlite")
    }

    /// activation.sqlite
    pub fn activation_sqlite(&self) -> PathBuf {
        self.root.join("activation.sqlite")
    }

    /// authority.sqlite
    pub fn authority_sqlite(&self) -> PathBuf {
        self.root.join("authority.sqlite")
    }

    /// vector.sqlite (se lo usi)
    pub fn vector_sqlite(&self) -> PathBuf {
        self.root.join("vector.sqlite")
    }
}
