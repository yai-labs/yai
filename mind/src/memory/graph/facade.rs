use crate::memory::graph::backend::store_for_scope;
use crate::memory::graph::domains::authority::types::AuthorityPolicy;
use crate::memory::graph::domains::episodic::types::Episode;
use crate::memory::graph::domains::vector::types::VectorEntry;
use crate::types::graph::*;
use anyhow::{bail, Result};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub struct GraphFacade;

impl GraphFacade {
    // --- READ OPERATIONS ---

    pub fn list_nodes(scope: GraphScope) -> Result<Vec<GraphNode>> {
        store_for_scope(&scope).list_nodes()
    }

    pub fn get_node(scope: GraphScope, id: &str) -> Result<Option<GraphNode>> {
        store_for_scope(&scope).get_node(id)
    }

    pub fn list_edges(scope: GraphScope) -> Result<Vec<GraphEdge>> {
        store_for_scope(&scope).list_edges()
    }

    pub fn stats(scope: GraphScope) -> Result<GraphStats> {
        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let edges = store.list_edges()?;
        let mut categories = BTreeMap::new();

        for n in &nodes {
            let key = classify_kind(&n.kind);
            *categories.entry(key).or_insert(0) += 1;
        }

        Ok(GraphStats {
            scope: format!("{:?}", scope),
            backend: store.descriptor(),
            nodes: nodes.len(),
            edges: edges.len(),
            categories,
        })
    }

    // --- WRITE OPERATIONS ---

    pub fn put_node(scope: GraphScope, node: GraphNode) -> Result<()> {
        store_for_scope(&scope).put_node(&node)
    }

    pub fn put_edge(scope: GraphScope, edge: GraphEdge) -> Result<()> {
        store_for_scope(&scope).put_edge(&edge)
    }

    // --- DOMAIN SPECIFIC (VECTOR, EPISODIC, AUTHORITY) ---

    pub fn put_vector_entries(scope: GraphScope, entries: Vec<VectorEntry>) -> Result<()> {
        store_for_scope(&scope).put_vector_entries(entries)
    }

    pub fn get_vector_entries(scope: GraphScope) -> Result<Vec<VectorEntry>> {
        store_for_scope(&scope).get_vector_entries()
    }

    pub fn ingest_episodes(scope: GraphScope) -> Result<Vec<Episode>> {
        store_for_scope(&scope).ingest_episodes()
    }

    pub fn list_authority_policies(scope: GraphScope) -> Result<Vec<AuthorityPolicy>> {
        store_for_scope(&scope).list_authority_policies()
    }

    // --- COGNITIVE OPERATIONS ---

    pub fn activate_and_commit(
        scope: GraphScope,
        seeds: Vec<(String, f64)>,
        params: Value,
    ) -> Result<ActivationCommit> {
        let ws_id = match &scope {
            GraphScope::Workspace(ws) => ws.clone(),
            _ => bail!("Workspace required for activation commit"),
        };

        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let edges = store.list_edges()?;

        // Utilizziamo il path completo per la logica di attivazione
        use crate::cognition::reasoning::roles::run_activation_logic;
        let result = run_activation_logic(&nodes, &edges, &seeds, &params)?;

        let trace = json!({
            "run_id": result.run_id,
            "hits": result.hits,
            "stats": result.stats
        });

        store.record_activation_trace(&ws_id, trace)?;

        Ok(ActivationCommit {
            activation_id: result.run_id,
            graph_snapshot_id: "snapshot".to_string(),
            params_hash: "params".to_string(),
            seed_hash: "seed".to_string(),
            result_hash: result.commit_hash,
            trace_hash: "trace".to_string(),
            proof_passed: true,
        })
    }
}

// --- HELPERS INTERNI ---

fn classify_kind(kind: &str) -> String {
    let k = kind.to_lowercase();
    if k.contains("episode") {
        "episodic".into()
    } else if k.contains("authority") {
        "authority".into()
    } else {
        "semantic".into()
    }
}
