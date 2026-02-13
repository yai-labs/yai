use crate::cognition::memory::graph::episodic::store::EpisodicStore;
use crate::cognition::memory::graph::episodic::types::Episode;
use anyhow::Result;

pub fn ingest(ws: &str) -> Result<Vec<Episode>> {
    let store = EpisodicStore::open(ws)?;
    store.ingest_events()
}
