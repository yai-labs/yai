use crate::memory::graph::api::{GraphEdge, GraphNode};
use crate::memory::graph::domains::semantic::types::{SemanticEdge, SemanticNode};

pub fn to_graph_node(node: SemanticNode) -> GraphNode {
    GraphNode {
        id: node.id,
        kind: node.kind,
        meta: node.meta,
    }
}

pub fn to_graph_edge(edge: SemanticEdge) -> GraphEdge {
    GraphEdge {
        id: edge.id,
        src: edge.src,
        dst: edge.dst,
        rel: edge.rel,
        weight: edge.weight,
    }
}
