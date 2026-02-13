use crate::{paths::Paths, types::WsId};

#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    pub ws: WsId,
    pub paths: Paths,
}

impl WorkspaceContext {
    pub fn new(ws: WsId) -> Self {
        let paths = Paths::from_env();
        Self { ws, paths }
    }
}
