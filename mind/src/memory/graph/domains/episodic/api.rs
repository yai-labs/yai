use crate::memory::graph::domains::episodic::types::Episode;
use crate::memory::graph::facade::GraphFacade;
use crate::types::graph::GraphScope;
// WorkspaceLayout non serve se deleghiamo il caricamento all'Engine tramite Scope
use anyhow::Result;

pub fn ingest(ws: &str) -> Result<Vec<Episode>> {
    let scope = GraphScope::Workspace(ws.to_string());

    // Qui la logica cambia: non "apriamo" più un file locale.
    // Chiediamo al Facade di orchestrare l'ingestione degli eventi episodici.
    // Nota: Il metodo 'ingest_episodes' deve essere presente nel Facade.
    let episodes = GraphFacade::ingest_episodes(scope)?;

    Ok(episodes)
}
