use crate::paths::Paths;
use crate::cognition::memory::legacy::paths as legacy_paths;
use crate::cognition::memory::graph::facade::{GraphEdge, GraphNode, GraphStore};
use crate::cognition::memory::graph::semantic::store::SemanticStore;
use crate::cognition::memory::graph::semantic::types::{SemanticEdge, SemanticNode};
use anyhow::Result;
use serde_json::Value;

pub struct WorkspaceSqliteStore {
    ws: String,
}

impl WorkspaceSqliteStore {
    pub fn new(ws: String) -> Self {
        Self { ws }
    }

    fn open(&self) -> Result<SemanticStore> {
        SemanticStore::open(&self.ws)
    }
}

impl GraphStore for WorkspaceSqliteStore {
    fn put_node(&self, node: &GraphNode) -> Result<()> {
        let store = self.open()?;
        store.upsert_node(&node.id, &node.kind, &node.meta)
    }

    fn put_edge(&self, edge: &GraphEdge) -> Result<()> {
        let store = self.open()?;
        store.upsert_edge(&edge.id, &edge.src, &edge.dst, &edge.rel, edge.weight)
    }

    fn list_nodes(&self) -> Result<Vec<GraphNode>> {
        let store = self.open()?;
        let nodes = store.list_nodes()?;
        Ok(nodes.into_iter().map(to_graph_node).collect())
    }

    fn list_edges(&self) -> Result<Vec<GraphEdge>> {
        let store = self.open()?;
        let edges = store.list_edges()?;
        Ok(edges.into_iter().map(to_graph_edge).collect())
    }

    fn get_node(&self, id: &str) -> Result<Option<GraphNode>> {
        let store = self.open()?;
        Ok(store.get_node(id)?.map(to_graph_node))
    }

    fn get_edges_for_node(&self, id: &str) -> Result<Vec<GraphEdge>> {
        let store = self.open()?;
        let edges = store.get_edges_for_node(id)?;
        Ok(edges.into_iter().map(to_graph_edge).collect())
    }

    fn descriptor(&self) -> String {
        paths::run_dir()
            .join(&self.ws)
            ;// moved
            
        let p = Paths::new_default(ws.to_string());
        let db_path = legacy_paths::semantic_sqlite(&p)
            .display()
            .to_string()
    }
}

fn to_graph_node(node: SemanticNode) -> GraphNode {
    GraphNode {
        id: node.id,
        kind: node.kind,
        meta: node.meta,
        last_seen: node.last_seen,
    }
}

fn to_graph_edge(edge: SemanticEdge) -> GraphEdge {
    GraphEdge {
        id: edge.id,
        src: edge.src,
        dst: edge.dst,
        rel: edge.rel,
        weight: edge.weight,
        meta: Value::Null,
    }
}
