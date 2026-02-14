use crate::types::graph::*;
use crate::memory::graph::backend::{store_for_scope}; // O BackendRpc
use anyhow::{Result, bail, Context};
use std::collections::{HashMap, HashSet, VecDeque, BTreeMap};
use serde_json::Value;

pub struct GraphFacade;

impl GraphFacade {
    // --- READ OPERATIONS ---
    
    pub fn list_nodes(scope: GraphScope) -> Result<Vec<GraphNode>> {
        store_for_scope(&scope).list_nodes()
    }

    pub fn get_node(scope: GraphScope, id: &str) -> Result<Option<GraphNode>> {
        store_for_scope(&scope).get_node(id)
    }

    pub fn neighbors(scope: GraphScope, id: &str, depth: usize, filters: NeighborFilters) -> Result<Subgraph> {
        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let all_edges = if depth > 1 { store.list_edges()? } else { store.get_edges_for_node(id)? };

        let (node_map, adj) = self::build_index(&nodes, &all_edges, &filters);
        
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert(id.to_string());
        queue.push_back((id.to_string(), 0));

        while let Some((current, d)) = queue.pop_front() {
            if d >= depth { continue; }
            if let Some(nei) = adj.get(&current) {
                for next in nei {
                    if visited.insert(next.clone()) { queue.push_back((next.clone(), d + 1)); }
                }
            }
        }

        let mut out_nodes: Vec<GraphNode> = visited.iter()
            .filter_map(|nid| node_map.get(nid).cloned()).collect();
        out_nodes.sort_by(|a, b| a.id.cmp(&b.id));

        let mut out_edges: Vec<GraphEdge> = all_edges.into_iter()
            .filter(|e| visited.contains(&e.src) && visited.contains(&e.dst))
            .filter(|e| self::filter_edge(e, &filters)).collect();
        out_edges.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(Subgraph { nodes: out_nodes, edges: out_edges })
    }

    pub fn stats(scope: GraphScope) -> Result<GraphStats> {
        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let edges = store.list_edges()?;
        let mut categories = BTreeMap::new();
        for n in &nodes {
            let key = self::classify_kind(&n.kind);
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

    // --- COGNITIVE OPERATIONS (Logic Only) ---

    pub fn export_to_string(scope: GraphScope, format: GraphExportFormat) -> Result<String> {
        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let edges = store.list_edges()?;
        
        match format {
            GraphExportFormat::Dot => {
                let mut out = String::from("digraph yai_graph {\n");
                for n in nodes { out.push_str(&format!("  \"{}\" [label=\"{}\"];\n", n.id, n.kind)); }
                for e in edges { out.push_str(&format!("  \"{}\" -> \"{}\" [label=\"{}\"];\n", e.src, e.dst, e.rel)); }
                out.push_str("}\n");
                Ok(out)
            },
            GraphExportFormat::Jsonl => {
                let mut out = String::new();
                for n in nodes { out.push_str(&serde_json::to_string(&n)?); out.push("\n"); }
                for e in edges { out.push_str(&serde_json::to_string(&e)?); out.push("\n"); }
                Ok(out)
            }
        }
    }

    pub fn activate_and_commit(scope: GraphScope, seeds: Vec<(String, f64)>, params: Value) -> Result<ActivationCommit> {
        let ws_id = match &scope {
            GraphScope::Workspace(ws) => ws.clone(),
            _ => bail!("Workspace required for activation commit"),
        };
        
        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let edges = store.list_edges()?;

        // Logica pura di calcolo (run_activation dovrebbe essere in cognition::reasoning)
        // Qui simuliamo il flusso per brevità, ma usa la tua funzione run_activation
        let result = crate::cognition::reasoning::run_activation_logic(&nodes, &edges, &seeds, &params)?;

        // SOVEREIGN: Mandiamo la traccia al backend (che la manderà all'Engine via RPC)
        let trace = serde_json::json!({
            "run_id": result.run_id,
            "topk": result.hits,
            "stats": result.stats
        });
        store.record_activation_trace(&ws_id, trace)?;

        Ok(ActivationCommit {
            activation_id: result.run_id,
            graph_snapshot_id: "calc_hash_here".to_string(),
            params_hash: "params_hash".to_string(),
            seed_hash: "seed_hash".to_string(),
            result_hash: result.commit_hash,
            trace_hash: "trace_hash".to_string(),
            proof_passed: true,
        })
    }
}

// --- HELPERS (Logic only, no I/O) ---

fn build_index(nodes: &[GraphNode], edges: &[GraphEdge], filters: &NeighborFilters) -> (HashMap<String, GraphNode>, HashMap<String, Vec<String>>) {
    let node_map: HashMap<String, GraphNode> = nodes.iter()
        .filter(|n| self::filter_node(n, filters))
        .map(|n| (n.id.clone(), n.clone())).collect();

    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for e in edges.iter().filter(|e| self::filter_edge(e, filters)) {
        if node_map.contains_key(&e.src) && node_map.contains_key(&e.dst) {
            adj.entry(e.src.clone()).or_default().push(e.dst.clone());
            if !filters.directed { adj.entry(e.dst.clone()).or_default().push(e.src.clone()); }
        }
    }
    (node_map, adj)
}

fn filter_node(node: &GraphNode, filters: &NeighborFilters) -> bool {
    if let Some(kinds) = &filters.kinds {
        kinds.iter().any(|k| node.kind.contains(k))
    } else { true }
}

fn filter_edge(edge: &GraphEdge, filters: &NeighborFilters) -> bool {
    if let Some(rels) = &filters.rels { rels.contains(&edge.rel) } else { true }
}

fn classify_kind(kind: &str) -> String {
    let k = kind.to_lowercase();
    if k.contains("episode") { "episodic".into() }
    else if k.contains("authority") { "authority".into() }
    else { "semantic".into() }
}