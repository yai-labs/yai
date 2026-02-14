use crate::types::graph::{GraphStore, GraphScope};
use crate::memory::graph::backend_rpc::BackendRpc;
use crate::transport::rpc_client::RpcClient; // Il tuo client RPC globale o di istanza
use anyhow::Result;

/// Questa è la funzione che la Facade chiama. 
/// In una architettura pulita, l'RpcClient verrebbe passato qui 
/// o recuperato da un contesto globale.
pub fn store_for_scope(scope: &GraphScope) -> Box<dyn GraphStore> {
    // 1. Ottieni il client RPC (qui dipende da come hai gestito lo stato globale)
    let client = RpcClient::new_default(); 

    // 2. Restituisci il backend RPC configurato con lo scope corretto
    // Usiamo Box<dyn GraphStore> perché la Facade non deve sapere 
    // che tipo di struct sta usando, le basta che rispetti il Trait.
    Box::new(BackendRpc::new(client, scope.clone()))
}

/// Helper per le etichette (usato in stats o logging)
pub fn scope_label(scope: &GraphScope) -> String {
    match scope {
        GraphScope::Global => "global".to_string(),
        GraphScope::Workspace(id) => format!("ws:{}", id),
    }
}