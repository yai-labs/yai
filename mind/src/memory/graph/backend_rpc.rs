use crate::providers::client::{ProviderClient, ProviderRequest};
use crate::transport::uds_server::EngineClient as RpcClient;
use crate::types::graph::{GraphEdge, GraphNode, GraphScope, GraphStore};
// Import necessari per i nuovi metodi
use crate::memory::graph::domains::authority::types::AuthorityPolicy;
use crate::memory::graph::domains::episodic::types::Episode;
use crate::memory::graph::domains::vector::types::VectorEntry;
use anyhow::{Context, Result};
use serde_json::{json, Value};

pub struct BackendRpc {
    client: RpcClient,
    scope: GraphScope,
}

impl BackendRpc {
    pub fn new(client: RpcClient, scope: GraphScope) -> Self {
        Self { client, scope }
    }

    fn call_storage(&self, method: &str, params: Value) -> Result<Value> {
        let req = ProviderRequest {
            provider: "E_RPC_STORAGE_GATE".to_string(),
            model: "internal".to_string(),
            payload: json!({
                "scope": self.scope,
                "method": method,
                "params": params
            }),
        };

        let response = ProviderClient::call(&self.client, req)
            .context(format!("Failed RPC call to storage: {}", method))?;

        Ok(response.payload)
    }
}

impl GraphStore for BackendRpc {
    // ... (metodi put_node, put_edge, list_nodes, list_edges, get_node, get_edges_for_node, record_activation_trace rimangono uguali)

    fn put_node(&self, node: &GraphNode) -> Result<()> {
        self.call_storage("put_node", json!(node))?;
        Ok(())
    }

    fn put_edge(&self, edge: &GraphEdge) -> Result<()> {
        self.call_storage("put_edge", json!(edge))?;
        Ok(())
    }

    fn list_nodes(&self) -> Result<Vec<GraphNode>> {
        let res = self.call_storage("list_nodes", json!({}))?;
        Ok(serde_json::from_value(res)?)
    }

    fn list_edges(&self) -> Result<Vec<GraphEdge>> {
        let res = self.call_storage("list_edges", json!({}))?;
        Ok(serde_json::from_value(res)?)
    }

    fn get_node(&self, id: &str) -> Result<Option<GraphNode>> {
        let res = self.call_storage("get_node", json!({ "id": id }))?;
        if res.is_null() {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_value(res)?))
        }
    }

    fn get_edges_for_node(&self, id: &str) -> Result<Vec<GraphEdge>> {
        let res = self.call_storage("get_edges_for_node", json!({ "id": id }))?;
        Ok(serde_json::from_value(res)?)
    }

    fn record_activation_trace(&self, _ws_id: &str, trace: Value) -> Result<()> {
        self.call_storage("record_activation_trace", trace)?;
        Ok(())
    }

    fn descriptor(&self) -> String {
        match &self.scope {
            GraphScope::Global => "rpc://engine/global".to_string(),
            GraphScope::Workspace(id) => format!("rpc://engine/workspace/{}", id),
        }
    }

    // --- NUOVE IMPLEMENTAZIONI RICHIESTE ---

    fn put_vector_entries(&self, entries: Vec<VectorEntry>) -> Result<()> {
        self.call_storage("put_vector_entries", json!(entries))?;
        Ok(())
    }

    fn get_vector_entries(&self) -> Result<Vec<VectorEntry>> {
        let res = self.call_storage("get_vector_entries", json!({}))?;
        Ok(serde_json::from_value(res)?)
    }

    fn ingest_episodes(&self) -> Result<Vec<Episode>> {
        let res = self.call_storage("ingest_episodes", json!({}))?;
        Ok(serde_json::from_value(res)?)
    }

    fn list_authority_policies(&self) -> Result<Vec<AuthorityPolicy>> {
        let res = self.call_storage("list_authority_policies", json!({}))?;
        Ok(serde_json::from_value(res)?)
    }
}
