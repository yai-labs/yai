use crate::memory::graph::domains::episodic::store::EpisodicStore;
use crate::memory::graph::domains::episodic::types::Episode;
use crate::workspace::layout::WorkspaceLayout;
use anyhow::Result;

pub fn ingest(ws: &str) -> Result<Vec<Episode>> {
    let layout = WorkspaceLayout::default_for(ws);

    let store = EpisodicStore::open(&layout)?;

    store.ingest_events()
}
