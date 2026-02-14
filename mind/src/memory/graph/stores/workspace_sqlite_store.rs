use crate::memory::graph::api::{GraphEdge, GraphNode, GraphStore};
use crate::memory::graph::domains::semantic::store::SemanticStore;
use crate::workspace::layout::WorkspaceLayout;
use anyhow::Result;

use super::mapper::{to_graph_edge, to_graph_node};

pub struct WorkspaceSqliteStore {
    layout: WorkspaceLayout,
}

impl WorkspaceSqliteStore {
    pub fn new(layout: WorkspaceLayout) -> Self {
        Self { layout }
    }

    fn open(&self) -> Result<SemanticStore> {
        // Il dominio NON decide path: layout lo fornisce
        let db_path = self.layout.semantic_sqlite();
        SemanticStore::open_at_path(db_path)
    }
}

impl GraphStore for WorkspaceSqliteStore {
    fn put_node(&self, node: &GraphNode) -> Result<()> {
        let store = self.open()?;
        store.upsert_node(&node.id, &node.kind, &node.meta)
    }

    fn put_edge(&self, edge: &GraphEdge) -> Result<()> {
        let store = self.open()?;
        store.upsert_edge(
            &edge.id,
            &edge.src,
            &edge.dst,
            &edge.rel,
            edge.weight,
        )
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
        self.layout.semantic_sqlite().display().to_string()
    }
}
