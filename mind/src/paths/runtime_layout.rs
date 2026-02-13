use std::path::PathBuf;

use super::workspace_layout::WorkspaceLayout;

/// Runtime root (es: ~/.yai/run/dev) viene dal kernel/boot.
/// Mind non deve inventarselo: lo riceve via env/config.
#[derive(Clone, Debug)]
pub struct RuntimeLayout {
    pub root: PathBuf,
}

impl RuntimeLayout {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn ws_dir(&self, ws_id: &str) -> PathBuf {
        self.root.join(ws_id)
    }

    pub fn ws(&self, ws_id: &str) -> WorkspaceLayout {
        WorkspaceLayout::new(ws_id.to_string(), self.ws_dir(ws_id))
    }

    pub fn control_sock(&self) -> PathBuf {
        self.root.join("control.sock")
    }
}
