use std::path::PathBuf;

/// Layout interno ad un workspace (multi-tenant).
/// Questo è *solo* il mapping dei path. Niente logica.
#[derive(Clone, Debug)]
pub struct WorkspaceLayout {
    pub ws_id: String,
    pub root: PathBuf,
}

impl WorkspaceLayout {
    pub fn new(ws_id: impl Into<String>, root: PathBuf) -> Self {
        Self {
            ws_id: ws_id.into(),
            root,
        }
    }

    pub fn events_log(&self) -> PathBuf {
        self.root.join("events.log")
    }

    pub fn semantic_sqlite(&self) -> PathBuf {
        self.root.join("semantic.sqlite")
    }
}
