use crate::memory::graph::domains::semantic::types::SemanticNode;
use crate::memory::graph::domains::vector::index::VectorIndex;
use crate::memory::graph::domains::vector::types::VectorEntry;
use crate::memory::graph::facade::GraphFacade; // Usiamo il Facade
use crate::types::graph::GraphScope;
use anyhow::Result;

pub fn build_index(ws: &str, entries: Vec<VectorEntry>) -> Result<()> {
    let scope = GraphScope::Workspace(ws.to_string());

    // Deleghiamo all'Engine il salvataggio dei vettori
    // Nota: Il Facade deve implementare put_vector_entries
    GraphFacade::put_vector_entries(scope, entries)
}

pub fn rebuild_from_semantic<F>(ws: &str, nodes: &[SemanticNode], mut embed: F) -> Result<usize>
where
    F: FnMut(&SemanticNode) -> Result<Vec<f32>>,
{
    let mut entries = Vec::with_capacity(nodes.len());
    for node in nodes {
        let embedding = embed(node)?;
        entries.push(VectorEntry {
            id: node.id.clone(),
            embedding,
        });
    }
    build_index(ws, entries)?;
    Ok(nodes.len())
}

pub fn search(ws: &str, query: &[f32], k: usize) -> Result<Vec<(String, f32)>> {
    let scope = GraphScope::Workspace(ws.to_string());

    // Recuperiamo le entries tramite il Facade invece dello store locale
    let entries = GraphFacade::get_vector_entries(scope)?;

    if entries.is_empty() {
        return Ok(vec![]);
    }

    let dim = entries[0].embedding.len();
    let mut index = VectorIndex::new(dim);

    let items: Vec<(String, Vec<f32>)> = entries.into_iter().map(|e| (e.id, e.embedding)).collect();

    index.build(&items);
    Ok(index.search(query, k))
}
