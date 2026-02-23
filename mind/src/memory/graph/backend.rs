use crate::memory::graph::backend_rpc::BackendRpc;
use crate::transport::uds_server::EngineClient as RpcClient;
use crate::types::graph::{GraphScope, GraphStore};

pub fn store_for_scope(scope: &GraphScope) -> Box<dyn GraphStore> {
    // Determiniamo il workspace_id per la connessione UDS
    let ws_id = match scope {
        GraphScope::Global => "global",
        GraphScope::Workspace(id) => id,
    };

    // Creiamo il client connettendoci al socket specifico
    let client = RpcClient::connect(ws_id).expect("Failed to connect to Engine UDS");

    Box::new(BackendRpc::new(client, scope.clone()))
}

pub fn scope_label(scope: &GraphScope) -> String {
    match scope {
        GraphScope::Global => "global".to_string(),
        GraphScope::Workspace(id) => format!("ws:{}", id),
    }
}
