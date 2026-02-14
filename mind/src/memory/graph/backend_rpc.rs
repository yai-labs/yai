use crate::types::graph::{GraphEdge, GraphNode, GraphStore, GraphScope};
use crate::transport::rpc_client::RpcClient; // Il bridge verso l'Engine
use anyhow::{Result, Context};
use serde_json::{json, Value};

pub struct BackendRpc {
    client: RpcClient,
    scope: GraphScope,
}

impl BackendRpc {
    pub fn new(client: RpcClient, scope: GraphScope) -> Self {
        Self { client, scope }
    }

    /// Helper per uniformare le chiamate RPC verso lo Storage Gate dell'Engine
    fn call_storage(&self, method: &str, params: Value) -> Result<Value> {
        let payload = json!({
            "scope": self.scope,
            "method": method,
            "params": params
        });
        
        self.client.call("E_RPC_STORAGE_GATE", payload)
            .context(format!("Failed RPC call to storage: {}", method))
    }
}

impl GraphStore for BackendRpc {
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

    fn record_activation_trace(&self, ws_id: &str, trace: Value) -> Result<()> {
        // Ignoriamo ws_id qui perché è già incluso nel GraphScope del Backend
        self.call_storage("record_activation_trace", trace)?;
        Ok(())
    }

    fn descriptor(&self) -> String {
        match &self.scope {
            GraphScope::Global => "rpc://engine/global".to_string(),
            GraphScope::Workspace(id) => format!("rpc://engine/workspace/{}", id),
        }
    }
}