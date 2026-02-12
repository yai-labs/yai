use crate::memory::graph::facade::{GraphEdge, GraphNode, GraphStore};
use crate::memory::graph::semantic::store::SemanticStore;
use crate::memory::graph::semantic::types::{SemanticEdge, SemanticNode};
use crate::shared::constants::DEFAULT_KNOWLEDGE_DB_PATH;
use anyhow::Result;
use serde_json::Value;
use std::env;
use std::path::PathBuf;

pub struct GlobalKnowledgeStore {
    db_path: PathBuf,
}

impl GlobalKnowledgeStore {
    pub fn new() -> Self {
        let db_path = env::var("YAI_KNOWLEDGE_DB")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(DEFAULT_KNOWLEDGE_DB_PATH));
        Self { db_path }
    }

    fn open(&self) -> Result<SemanticStore> {
        SemanticStore::open_at_path(self.db_path.clone())
    }
}

impl GraphStore for GlobalKnowledgeStore {
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
        self.db_path.display().to_string()
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
