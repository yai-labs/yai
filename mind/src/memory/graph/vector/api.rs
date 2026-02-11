use crate::memory::graph::semantic::types::SemanticNode;
use crate::memory::graph::vector::index::VectorIndex;
use crate::memory::graph::vector::store::VectorStore;
use crate::memory::graph::vector::types::VectorEntry;
use anyhow::Result;

pub fn build_index(ws: &str, entries: Vec<VectorEntry>) -> Result<()> {
    let mut store = VectorStore::open(ws)?;
    store.set(entries)?;
    Ok(())
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
    let store = VectorStore::open(ws)?;
    let dim = store.entries().first().map(|e| e.embedding.len()).unwrap_or(16);
    let mut index = VectorIndex::new(dim);
    let items: Vec<(String, Vec<f32>)> = store
        .entries()
        .iter()
        .map(|e| (e.id.clone(), e.embedding.clone()))
        .collect();
    index.build(&items);
    Ok(index.search(query, k))
}
