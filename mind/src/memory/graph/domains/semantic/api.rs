use crate::memory::graph::domains::semantic::types::{
    ExpiredSemanticNode, NodeRetention, SemanticEdge, SemanticNode,
};
use crate::memory::graph::facade::GraphFacade;
use crate::types::graph::{GraphEdge, GraphNode, GraphScope};
use anyhow::Result;
use serde_json::Value;

/// Aggiunge un nodo semantico tramite il Facade universale
pub fn add_node(ws: &str, id: &str, kind: &str, meta: &Value) -> Result<()> {
    let scope = GraphScope::Workspace(ws.to_string());
    let node = GraphNode {
        id: id.to_string(),
        kind: kind.to_string(),
        meta: meta.clone(), // Fix: il campo si chiama 'meta', non 'metadata'
        last_seen: 0,
    };
    GraphFacade::put_node(scope, node)
}

/// Versione con retention (mappa sul put_node standard)
pub fn add_node_with_retention(
    ws: &str,
    id: &str,
    kind: &str,
    meta: &Value,
    _retention: &NodeRetention,
) -> Result<()> {
    add_node(ws, id, kind, meta)
}

/// Aggiunge un arco tra due nodi
pub fn add_edge(ws: &str, id: &str, src: &str, dst: &str, rel: &str, weight: f32) -> Result<()> {
    let scope = GraphScope::Workspace(ws.to_string());
    let edge = GraphEdge {
        id: id.to_string(),
        src: src.to_string(),
        dst: dst.to_string(),
        rel: rel.to_string(), // Fix: il campo si chiama 'rel', non 'kind'
        weight,
        meta: serde_json::json!({}), // Fix: il campo si chiama 'meta'
    };
    // Nota: Assicurati che GraphFacade abbia il metodo put_edge,
    // altrimenti va aggiunto in facade.rs
    GraphFacade::put_edge(scope, edge)
}

/// Recupera la lista dei nodi e li mappa nel tipo SemanticNode
pub fn list_nodes(ws: &str) -> Result<Vec<SemanticNode>> {
    let scope = GraphScope::Workspace(ws.to_string());
    let nodes = GraphFacade::list_nodes(scope)?;

    Ok(nodes
        .into_iter()
        .map(|n| SemanticNode {
            id: n.id,
            kind: n.kind,
            meta: n.meta,
            last_seen: n.last_seen,
            retention_policy_id: String::new(), // Fix: String invece di Option
            tombstone: false,
            created_ts: 0,    // Aggiunto campo mancante
            expires_at: None, // Aggiunto campo mancante
            compliance: None, // Aggiunto campo mancante
        })
        .collect())
}

/// Recupera la lista degli archi e li mappa nel tipo SemanticEdge
pub fn list_edges(ws: &str) -> Result<Vec<SemanticEdge>> {
    let scope = GraphScope::Workspace(ws.to_string());
    let edges = GraphFacade::list_edges(scope)?;

    Ok(edges
        .into_iter()
        .map(|e| SemanticEdge {
            id: e.id,
            src: e.src,
            dst: e.dst,
            rel: e.rel,
            weight: e.weight,
            // metadata: e.meta, // Rimuovi o cambia se SemanticEdge non ha 'meta'
        })
        .collect())
}

pub fn expire_due(_ws: &str, _now_ts: u64) -> Result<Vec<ExpiredSemanticNode>> {
    Ok(vec![])
}
