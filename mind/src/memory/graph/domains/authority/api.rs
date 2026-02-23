use crate::memory::graph::domains::authority::types::AuthorityPolicy;
use crate::memory::graph::facade::GraphFacade;
use crate::types::graph::GraphScope;
use anyhow::Result;

pub fn load(ws: &str) -> Result<Vec<AuthorityPolicy>> {
    let scope = GraphScope::Workspace(ws.to_string());

    // Deleghiamo al Facade il compito di recuperare le policy tramite l'Engine
    // Nota: Assicurati che GraphFacade implementi list_authority_policies
    let policies = GraphFacade::list_authority_policies(scope)?;

    Ok(policies)
}
