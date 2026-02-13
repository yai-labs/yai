use crate::cognition::memory::graph::activation::api::{
    canonicalize_seeds, hash_params, hash_seeds, run_activation, ActivationGraph, ActivationMethod,
    ActivationParams, ActivationResult as EngineActivationResult, ActivationSeed, ActivationStats,
    NodeId,
};
use crate::cognition::memory::graph::activation::store::{
    ActivationResultRow, ActivationRunMeta, ActivationTraceStore,
};
use crate::cognition::memory::graph::activation::trace::ActivationTrace;
use crate::cognition::memory::graph::store::global_knowledge_store::GlobalKnowledgeStore;
use crate::cognition::memory::graph::store::workspace_sqlite_store::WorkspaceSqliteStore;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum GraphScope {
    Workspace(String),
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub kind: String,
    pub meta: Value,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub src: String,
    pub dst: String,
    pub rel: String,
    pub weight: f32,
    pub meta: Value,
}

#[derive(Debug, Clone, Default)]
pub struct NeighborFilters {
    pub rels: Option<HashSet<String>>,
    pub kinds: Option<HashSet<String>>,
    pub directed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Subgraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphStats {
    pub scope: String,
    pub backend: String,
    pub nodes: usize,
    pub edges: usize,
    pub categories: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum GraphExportFormat {
    Dot,
    Jsonl,
}

#[derive(Debug, Clone)]
pub struct ActivationPolicy {
    pub hops: usize,
    pub decay: f32,
    pub threshold: f32,
    pub top_n: usize,
}

impl Default for ActivationPolicy {
    fn default() -> Self {
        Self {
            hops: 2,
            decay: 0.6,
            threshold: 0.05,
            top_n: 16,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ActivationResult {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub scores: BTreeMap<String, f32>,
    pub metrics: Option<ActivationStats>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActivationCommit {
    pub activation_id: String,
    pub graph_snapshot_id: String,
    pub params_hash: String,
    pub seed_hash: String,
    pub result_hash: String,
    pub trace_hash: String,
    pub algo_id: String,
    pub metrics: ActivationStats,
    pub proof: ActivationProof,
    pub result: EngineActivationResult,
    pub trace: ActivationTrace,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActivationProof {
    pub residual_mass: f64,
    pub pushed: usize,
    pub visited: usize,
    pub converged: bool,
    pub invariants_passed: bool,
    pub invariant_errors: Vec<String>,
}

pub trait GraphStore {
    fn put_node(&self, node: &GraphNode) -> Result<()>;
    fn put_edge(&self, edge: &GraphEdge) -> Result<()>;
    fn list_nodes(&self) -> Result<Vec<GraphNode>>;
    fn list_edges(&self) -> Result<Vec<GraphEdge>>;
    fn get_node(&self, id: &str) -> Result<Option<GraphNode>> {
        Ok(self.list_nodes()?.into_iter().find(|n| n.id == id))
    }
    fn get_edges_for_node(&self, id: &str) -> Result<Vec<GraphEdge>> {
        Ok(self
            .list_edges()?
            .into_iter()
            .filter(|e| e.src == id || e.dst == id)
            .collect())
    }
    fn descriptor(&self) -> String;
}

pub struct GraphFacade;

impl GraphFacade {
    pub fn list_nodes(scope: GraphScope) -> Result<Vec<GraphNode>> {
        let store = store_for_scope(&scope);
        store.list_nodes()
    }

    pub fn list_edges(scope: GraphScope) -> Result<Vec<GraphEdge>> {
        let store = store_for_scope(&scope);
        store.list_edges()
    }

    pub fn put_node(scope: GraphScope, node: GraphNode) -> Result<()> {
        let store = store_for_scope(&scope);
        store.put_node(&node)
    }

    pub fn put_edge(scope: GraphScope, edge: GraphEdge) -> Result<()> {
        let store = store_for_scope(&scope);
        store.put_edge(&edge)
    }

    pub fn get_node(scope: GraphScope, id: &str) -> Result<Option<GraphNode>> {
        let store = store_for_scope(&scope);
        store.get_node(id)
    }

    pub fn semantic_neighbors_out(scope: GraphScope, n: NodeId) -> Result<Vec<(NodeId, f64)>> {
        semantic_neighbors_out(scope, n)
    }

    pub fn semantic_out_norm(scope: GraphScope, n: NodeId) -> Result<f64> {
        semantic_out_norm(scope, n)
    }

    pub fn graph_fingerprint(scope: GraphScope) -> Result<String> {
        graph_fingerprint(scope)
    }

    pub fn neighbors(
        scope: GraphScope,
        id: &str,
        depth: usize,
        filters: NeighborFilters,
    ) -> Result<Subgraph> {
        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let mut edges = store.get_edges_for_node(id)?;
        if depth > 1 {
            edges = store.list_edges()?;
        }
        let (node_map, adj) = build_index(&nodes, &edges, &filters);

        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, usize)> = VecDeque::new();
        visited.insert(id.to_string());
        queue.push_back((id.to_string(), 0));

        while let Some((current, d)) = queue.pop_front() {
            if d >= depth {
                continue;
            }
            if let Some(nei) = adj.get(&current) {
                for next in nei {
                    if visited.insert(next.clone()) {
                        queue.push_back((next.clone(), d + 1));
                    }
                }
            }
        }

        let mut out_nodes: Vec<GraphNode> = visited
            .iter()
            .filter_map(|nid| node_map.get(nid).cloned())
            .collect();
        out_nodes.sort_by(|a, b| a.id.cmp(&b.id));

        let mut out_edges: Vec<GraphEdge> = edges
            .into_iter()
            .filter(|e| visited.contains(&e.src) && visited.contains(&e.dst))
            .filter(|e| filter_edge(e, &filters))
            .collect();
        out_edges.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(Subgraph {
            nodes: out_nodes,
            edges: out_edges,
        })
    }

    pub fn stats(scope: GraphScope) -> Result<GraphStats> {
        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let edges = store.list_edges()?;
        let mut categories: BTreeMap<String, usize> = BTreeMap::new();
        for n in &nodes {
            let key = classify_kind(&n.kind);
            *categories.entry(key).or_insert(0) += 1;
        }
        Ok(GraphStats {
            scope: scope_label(&scope),
            backend: store.descriptor(),
            nodes: nodes.len(),
            edges: edges.len(),
            categories,
        })
    }

    pub fn export(scope: GraphScope, format: GraphExportFormat, out_path: &Path) -> Result<()> {
        let store = store_for_scope(&scope);
        let mut nodes = store.list_nodes()?;
        let mut edges = store.list_edges()?;
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        let (nodes, edges) = filter_snapshot_for_activation(&nodes, &edges);

        let file = File::create(out_path)?;
        let mut out = BufWriter::new(file);

        match format {
            GraphExportFormat::Dot => {
                writeln!(out, "digraph yai_graph {{")?;
                for n in &nodes {
                    writeln!(
                        out,
                        "  \"{}\" [label=\"{}\"];",
                        escape_dot(&n.id),
                        escape_dot(&n.kind)
                    )?;
                }
                for e in &edges {
                    writeln!(
                        out,
                        "  \"{}\" -> \"{}\" [label=\"{}:{:.3}\"];",
                        escape_dot(&e.src),
                        escape_dot(&e.dst),
                        escape_dot(&e.rel),
                        e.weight
                    )?;
                }
                writeln!(out, "}}")?;
            }
            GraphExportFormat::Jsonl => {
                for n in &nodes {
                    writeln!(
                        out,
                        "{}",
                        serde_json::json!({"kind":"node","id":n.id,"type":n.kind,"meta":n.meta})
                            .to_string()
                    )?;
                }
                for e in &edges {
                    writeln!(
                        out,
                        "{}",
                        serde_json::json!({"kind":"edge","id":e.id,"src":e.src,"dst":e.dst,"rel":e.rel,"weight":e.weight,"meta":e.meta}).to_string()
                    )?;
                }
            }
        }
        out.flush()?;
        Ok(())
    }

    pub fn activate(
        scope: GraphScope,
        seeds: &[(String, f32)],
        policy: ActivationPolicy,
    ) -> Result<ActivationResult> {
        if matches!(scope, GraphScope::Global) {
            bail!("ws_id required for activation");
        }
        let store = store_for_scope(&scope);
        let mut nodes = store.list_nodes()?;
        let mut edges = store.list_edges()?;
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        let (nodes, edges) = filter_snapshot_for_activation(&nodes, &edges);

        let ws_id = match &scope {
            GraphScope::Workspace(ws) => ws.clone(),
            GraphScope::Global => "global".to_string(),
        };
        let graph = SemanticSnapshotGraph::new(nodes.clone(), edges.clone(), ws_id);
        let seed = seeds
            .iter()
            .map(|(node, weight)| ActivationSeed {
                node: node.clone(),
                weight: *weight as f64,
            })
            .collect::<Vec<_>>();
        let seed = canonicalize_seeds(&seed)?;
        let params = policy_to_params(&policy);
        let act = run_activation(&graph, &seed, &params)?;

        let selected: HashSet<String> = act.hits.iter().map(|n| n.node.clone()).collect();
        let scores: BTreeMap<String, f32> = act
            .hits
            .iter()
            .map(|hit| (hit.node.clone(), hit.score as f32))
            .collect();
        let out_nodes: Vec<GraphNode> = nodes
            .into_iter()
            .filter(|n| selected.contains(&n.id))
            .collect();
        let out_edges: Vec<GraphEdge> = edges
            .into_iter()
            .filter(|e| selected.contains(&e.src) && selected.contains(&e.dst))
            .collect();

        Ok(ActivationResult {
            nodes: out_nodes,
            edges: out_edges,
            scores,
            metrics: Some(act.stats),
        })
    }

    pub fn activate_and_commit(
        scope: GraphScope,
        seeds: &[(String, f64)],
        params: ActivationParams,
    ) -> Result<ActivationCommit> {
        Self::activate_and_commit_with_trace(scope, seeds, params, true)
    }

    pub fn activate_and_commit_with_trace(
        scope: GraphScope,
        seeds: &[(String, f64)],
        params: ActivationParams,
        save_trace: bool,
    ) -> Result<ActivationCommit> {
        let ws_id = match &scope {
            GraphScope::Workspace(ws) => ws.clone(),
            GraphScope::Global => bail!("ws_id required for activation"),
        };
        let store = store_for_scope(&scope);
        let mut nodes = store.list_nodes()?;
        let mut edges = store.list_edges()?;
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        edges.sort_by(|a, b| a.id.cmp(&b.id));
        let (nodes, edges) = filter_snapshot_for_activation(&nodes, &edges);

        let graph = SemanticSnapshotGraph::new(nodes, edges, ws_id.clone());
        let graph_snapshot_id = graph.fingerprint()?;
        let seed = seeds
            .iter()
            .map(|(node, weight)| ActivationSeed {
                node: node.clone(),
                weight: *weight,
            })
            .collect::<Vec<_>>();
        let seed = canonicalize_seeds(&seed)?;
        let params_hash = hash_params(&params)?;
        let seed_hash = hash_seeds(&seed)?;

        let result = run_activation(&graph, &seed, &params)?;
        let proof = evaluate_activation_result(&result, &params)?;
        let result_hash = result.commit_hash.clone();
        let activation_id = result.run_id.clone();
        let trace = ActivationTrace {
            run_id: result.run_id.clone(),
            created_at_unix: now_epoch_secs() as i64,
            graph_fingerprint: graph_snapshot_id.clone(),
            params: params.clone(),
            seeds: seed.clone(),
            commit_hash: result.commit_hash.clone(),
            topk: result.hits.clone(),
            stats: result.stats.clone(),
        };
        let trace_hash = hash_json(&serde_json::json!({
            "run_id": trace.run_id,
            "graph_fingerprint": trace.graph_fingerprint,
            "params": trace.params,
            "seeds": trace.seeds,
            "commit_hash": trace.commit_hash,
            "topk": trace.topk,
            "stats": trace.stats,
        }))?;
        if save_trace {
            let trace_store = ActivationTraceStore::open(&ws_id)?;
            let meta = ActivationRunMeta {
                run_id: trace.run_id.clone(),
                ws_id: ws_id.clone(),
                created_at_unix: trace.created_at_unix,
                graph_fingerprint: trace.graph_fingerprint.clone(),
                params_hash: params_hash.clone(),
                seeds_hash: seed_hash.clone(),
                commit_hash: trace.commit_hash.clone(),
                params: trace.params.clone(),
                seeds: trace.seeds.clone(),
                stats: trace.stats.clone(),
            };
            let results = trace
                .topk
                .iter()
                .enumerate()
                .map(|(idx, hit)| ActivationResultRow {
                    node_id: hit.node.clone(),
                    rank: idx as i64 + 1,
                    score_q: hit.score_q,
                })
                .collect::<Vec<_>>();
            trace_store.record_run(&meta, &results, None)?;
        }

        Ok(ActivationCommit {
            activation_id,
            graph_snapshot_id,
            params_hash,
            seed_hash,
            result_hash,
            trace_hash,
            algo_id: algo_id(&params).to_string(),
            metrics: result.stats.clone(),
            proof,
            result,
            trace,
        })
    }
}

fn store_for_scope(scope: &GraphScope) -> Box<dyn GraphStore> {
    match scope {
        GraphScope::Workspace(ws) => Box::new(WorkspaceSqliteStore::new(ws.clone())),
        GraphScope::Global => Box::new(GlobalKnowledgeStore::new()),
    }
}

fn scope_label(scope: &GraphScope) -> String {
    match scope {
        GraphScope::Workspace(ws) => format!("workspace:{ws}"),
        GraphScope::Global => "global".to_string(),
    }
}

fn classify_kind(kind: &str) -> String {
    let k = kind.to_ascii_lowercase();
    if k.contains("episode") {
        "episodic".to_string()
    } else if k.contains("authority") || k.contains("policy") {
        "authority".to_string()
    } else {
        "semantic".to_string()
    }
}

fn build_index(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    filters: &NeighborFilters,
) -> (HashMap<String, GraphNode>, HashMap<String, Vec<String>>) {
    let node_map: HashMap<String, GraphNode> = nodes
        .iter()
        .filter(|n| filter_node(n, filters))
        .map(|n| (n.id.clone(), n.clone()))
        .collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for e in edges.iter().filter(|e| filter_edge(e, filters)) {
        if node_map.contains_key(&e.src) && node_map.contains_key(&e.dst) {
            adj.entry(e.src.clone()).or_default().push(e.dst.clone());
            if !filters.directed {
                adj.entry(e.dst.clone()).or_default().push(e.src.clone());
            }
        }
    }
    (node_map, adj)
}

fn filter_node(node: &GraphNode, filters: &NeighborFilters) -> bool {
    if let Some(kinds) = &filters.kinds {
        kinds.iter().any(|k| {
            node.kind == *k
                || node.kind.starts_with(k)
                || node
                    .kind
                    .to_ascii_lowercase()
                    .contains(&k.to_ascii_lowercase())
        })
    } else {
        true
    }
}

fn filter_edge(edge: &GraphEdge, filters: &NeighborFilters) -> bool {
    if let Some(rels) = &filters.rels {
        rels.contains(&edge.rel)
    } else {
        true
    }
}

fn escape_dot(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn policy_to_params(policy: &ActivationPolicy) -> ActivationParams {
    ActivationParams {
        top_k: policy.top_n.max(1),
        method: ActivationMethod::LocalPush,
        ..ActivationParams::default()
    }
}

fn filter_snapshot_for_activation(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    let kept_nodes: Vec<GraphNode> = nodes
        .iter()
        .filter(|n| {
            n.kind != "activation_event"
                && n.kind != "activation_trace"
                && !n.id.starts_with("node:activation:")
                && !n.id.starts_with("node:activation_trace:")
        })
        .cloned()
        .collect();
    let kept_ids: HashSet<String> = kept_nodes.iter().map(|n| n.id.clone()).collect();
    let kept_edges: Vec<GraphEdge> = edges
        .iter()
        .filter(|e| kept_ids.contains(&e.src) && kept_ids.contains(&e.dst))
        .cloned()
        .collect();
    (kept_nodes, kept_edges)
}

fn evaluate_activation_result(
    result: &EngineActivationResult,
    params: &ActivationParams,
) -> Result<ActivationProof> {
    let mut invariant_errors = Vec::new();
    if result.hits.is_empty() {
        invariant_errors.push("activation produced empty top_k".to_string());
    }
    let mut prev_score_q = i64::MAX;
    let mut prev_id = String::new();
    for node in &result.hits {
        if !node.score.is_finite() {
            invariant_errors.push(format!("activation score is not finite for {}", node.node));
        }
        if node.score_q > prev_score_q {
            invariant_errors.push("activation top_k is not sorted by score_q desc".to_string());
        }
        if node.score_q == prev_score_q && node.node < prev_id {
            invariant_errors.push("activation top_k ordering is not stable".to_string());
        }
        prev_score_q = node.score_q;
        prev_id = node.node.clone();
    }

    if !result.stats.residual_mass.is_finite() {
        invariant_errors.push("activation residual mass is not finite".to_string());
    }
    if result.stats.pushed > params.max_push {
        invariant_errors.push("activation pushes exceeded max_push".to_string());
    }
    if result.stats.visited > params.max_nodes {
        invariant_errors.push("activation visited exceeded max_nodes".to_string());
    }
    let converged =
        result.stats.residual_mass <= params.epsilon || result.stats.pushed < params.max_push;
    let proof = ActivationProof {
        residual_mass: result.stats.residual_mass,
        pushed: result.stats.pushed,
        visited: result.stats.visited,
        converged,
        invariants_passed: invariant_errors.is_empty(),
        invariant_errors: invariant_errors.clone(),
    };
    if !proof.invariants_passed {
        bail!(
            "activation invariants failed: {}",
            invariant_errors.join("; ")
        );
    }
    Ok(proof)
}

fn now_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn hash_json<T: serde::Serialize>(value: &T) -> Result<String> {
    let payload = serde_json::to_vec(value)?;
    Ok(blake3::hash(&payload).to_hex().to_string())
}

fn algo_id(params: &ActivationParams) -> &'static str {
    match params.method {
        ActivationMethod::LocalPush => "ppr_push_v1",
        ActivationMethod::PowerIteration => "ppr_power_v1",
    }
}

pub fn semantic_neighbors_out(scope: GraphScope, n: NodeId) -> Result<Vec<(NodeId, f64)>> {
    let store = store_for_scope(&scope);
    let mut out = store
        .list_edges()?
        .into_iter()
        .filter(|e| e.src == n)
        .map(|e| (e.dst, e.weight as f64))
        .collect::<Vec<_>>();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

pub fn semantic_out_norm(scope: GraphScope, n: NodeId) -> Result<f64> {
    let out = semantic_neighbors_out(scope, n)?;
    Ok(out.iter().map(|(_, w)| *w).sum())
}

pub fn graph_fingerprint(scope: GraphScope) -> Result<String> {
    let store = store_for_scope(&scope);
    let mut nodes = store.list_nodes()?;
    let mut edges = store.list_edges()?;
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    edges.sort_by(|a, b| {
        a.src
            .cmp(&b.src)
            .then_with(|| a.dst.cmp(&b.dst))
            .then_with(|| a.rel.cmp(&b.rel))
    });
    let ws_id = match &scope {
        GraphScope::Workspace(ws) => ws.as_str(),
        GraphScope::Global => "global",
    };
    Ok(hash_graph_snapshot(ws_id, &nodes, &edges))
}

struct SemanticSnapshotGraph {
    neighbors: BTreeMap<NodeId, Vec<(NodeId, f64)>>,
    norms: BTreeMap<NodeId, f64>,
    fingerprint: String,
}

impl SemanticSnapshotGraph {
    fn new(nodes: Vec<GraphNode>, edges: Vec<GraphEdge>, ws_id: String) -> Self {
        let mut neighbors: BTreeMap<NodeId, Vec<(NodeId, f64)>> = BTreeMap::new();
        for n in nodes {
            neighbors.entry(n.id).or_default();
        }
        let mut edges_for_fingerprint = Vec::new();
        for e in edges {
            edges_for_fingerprint.push(GraphEdge {
                id: e.id.clone(),
                src: e.src.clone(),
                dst: e.dst.clone(),
                rel: e.rel.clone(),
                weight: e.weight,
                meta: e.meta.clone(),
            });
            neighbors
                .entry(e.src)
                .or_default()
                .push((e.dst, e.weight as f64));
        }
        for vals in neighbors.values_mut() {
            vals.sort_by(|a, b| a.0.cmp(&b.0));
        }
        let norms = neighbors
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().map(|(_, w)| *w).sum()))
            .collect::<BTreeMap<_, _>>();
        edges_for_fingerprint.sort_by(|a, b| {
            a.src
                .cmp(&b.src)
                .then_with(|| a.dst.cmp(&b.dst))
                .then_with(|| a.rel.cmp(&b.rel))
        });
        let node_ids = neighbors.keys().cloned().collect::<Vec<_>>();
        let nodes_for_fingerprint = node_ids
            .into_iter()
            .map(|id| GraphNode {
                id,
                kind: String::new(),
                meta: serde_json::Value::Null,
                last_seen: 0,
            })
            .collect::<Vec<_>>();
        let fingerprint =
            hash_graph_snapshot(&ws_id, &nodes_for_fingerprint, &edges_for_fingerprint);
        Self {
            neighbors,
            norms,
            fingerprint,
        }
    }
}

fn hash_graph_snapshot(ws_id: &str, nodes: &[GraphNode], edges: &[GraphEdge]) -> String {
    let mut bytes = Vec::new();
    append_str(&mut bytes, ws_id);
    append_u64(&mut bytes, nodes.len() as u64);
    for node in nodes {
        append_str(&mut bytes, &node.id);
    }
    append_u64(&mut bytes, edges.len() as u64);
    for edge in edges {
        append_str(&mut bytes, &edge.src);
        append_str(&mut bytes, &edge.dst);
        append_str(&mut bytes, &edge.rel);
    }
    blake3::hash(&bytes).to_hex().to_string()
}

fn append_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn append_str(out: &mut Vec<u8>, value: &str) {
    append_u64(out, value.len() as u64);
    out.extend_from_slice(value.as_bytes());
}

impl ActivationGraph for SemanticSnapshotGraph {
    fn neighbors_out(&self, n: NodeId) -> Result<Vec<(NodeId, f64)>> {
        Ok(self.neighbors.get(&n).cloned().unwrap_or_default())
    }

    fn out_norm(&self, n: NodeId) -> Result<f64> {
        Ok(self.norms.get(&n).copied().unwrap_or(0.0))
    }

    fn fingerprint(&self) -> Result<String> {
        Ok(self.fingerprint.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn ws() -> String {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        format!("graph_facade_test_{ts}")
    }

    fn seed(scope: GraphScope) -> Result<()> {
        GraphFacade::put_node(
            scope.clone(),
            GraphNode {
                id: "node:test:a".to_string(),
                kind: "semantic".to_string(),
                meta: serde_json::json!({"name":"a"}),
                last_seen: 1,
            },
        )?;
        GraphFacade::put_node(
            scope.clone(),
            GraphNode {
                id: "node:test:b".to_string(),
                kind: "episodic".to_string(),
                meta: serde_json::json!({"name":"b"}),
                last_seen: 2,
            },
        )?;
        GraphFacade::put_node(
            scope.clone(),
            GraphNode {
                id: "node:test:c".to_string(),
                kind: "authority_policy".to_string(),
                meta: serde_json::json!({"name":"c"}),
                last_seen: 3,
            },
        )?;
        GraphFacade::put_edge(
            scope.clone(),
            GraphEdge {
                id: "edge:test:ab".to_string(),
                src: "node:test:a".to_string(),
                dst: "node:test:b".to_string(),
                rel: "related".to_string(),
                weight: 1.0,
                meta: Value::Null,
            },
        )?;
        GraphFacade::put_edge(
            scope,
            GraphEdge {
                id: "edge:test:bc".to_string(),
                src: "node:test:b".to_string(),
                dst: "node:test:c".to_string(),
                rel: "related".to_string(),
                weight: 0.5,
                meta: Value::Null,
            },
        )?;
        Ok(())
    }

    #[test]
    fn insert_and_get_node() -> Result<()> {
        let scope = GraphScope::Workspace(ws());
        seed(scope.clone())?;
        let node = GraphFacade::get_node(scope, "node:test:a")?;
        assert!(node.is_some());
        Ok(())
    }

    #[test]
    fn neighbors_depth_is_coherent() -> Result<()> {
        let scope = GraphScope::Workspace(ws());
        seed(scope.clone())?;
        let d1 =
            GraphFacade::neighbors(scope.clone(), "node:test:a", 1, NeighborFilters::default())?;
        let d2 = GraphFacade::neighbors(scope, "node:test:a", 2, NeighborFilters::default())?;
        assert!(d1.nodes.len() >= 2);
        assert!(d2.nodes.len() >= d1.nodes.len());
        Ok(())
    }

    #[test]
    fn export_and_stats_work() -> Result<()> {
        let scope = GraphScope::Workspace(ws());
        seed(scope.clone())?;
        let stats = GraphFacade::stats(scope.clone())?;
        assert!(stats.nodes >= 3);
        assert!(stats.edges >= 2);
        let out_dot = std::env::temp_dir().join("yai_graph_facade_test.dot");
        GraphFacade::export(scope.clone(), GraphExportFormat::Dot, &out_dot)?;
        assert!(fs::metadata(&out_dot)?.len() > 0);
        let out_jsonl = std::env::temp_dir().join("yai_graph_facade_test.jsonl");
        GraphFacade::export(scope, GraphExportFormat::Jsonl, &out_jsonl)?;
        assert!(fs::metadata(&out_jsonl)?.len() > 0);
        Ok(())
    }

    #[test]
    fn activation_commit_is_deterministic() -> Result<()> {
        let base = ws();
        let scope_a = GraphScope::Workspace(format!("{base}_a"));
        let scope_b = GraphScope::Workspace(format!("{base}_b"));
        seed(scope_a.clone())?;
        seed(scope_b.clone())?;
        let mut params = ActivationParams::default();
        params.top_k = 4;
        let seeds = vec![("node:test:a".to_string(), 1.0)];

        let first = GraphFacade::activate_and_commit(scope_a, &seeds, params.clone())?;
        let second = GraphFacade::activate_and_commit(scope_b, &seeds, params)?;

        assert_ne!(first.result_hash, second.result_hash);
        assert_ne!(first.trace_hash, second.trace_hash);
        assert_ne!(first.graph_snapshot_id, second.graph_snapshot_id);
        assert_eq!(first.params_hash, second.params_hash);
        assert_eq!(first.seed_hash, second.seed_hash);
        assert!(first.proof.invariants_passed);
        Ok(())
    }

    #[test]
    fn snapshot_order_independent_fingerprint() -> Result<()> {
        let ws_id = "order_independent_ws".to_string();
        let nodes_a = vec![
            GraphNode {
                id: "node:test:b".to_string(),
                kind: "semantic".to_string(),
                meta: Value::Null,
                last_seen: 2,
            },
            GraphNode {
                id: "node:test:a".to_string(),
                kind: "semantic".to_string(),
                meta: Value::Null,
                last_seen: 1,
            },
        ];
        let edges_a = vec![GraphEdge {
            id: "edge:test:ab".to_string(),
            src: "node:test:a".to_string(),
            dst: "node:test:b".to_string(),
            rel: "related".to_string(),
            weight: 1.0,
            meta: Value::Null,
        }];

        let nodes_b = vec![
            GraphNode {
                id: "node:test:a".to_string(),
                kind: "semantic".to_string(),
                meta: Value::Null,
                last_seen: 1,
            },
            GraphNode {
                id: "node:test:b".to_string(),
                kind: "semantic".to_string(),
                meta: Value::Null,
                last_seen: 2,
            },
        ];
        let edges_b = vec![GraphEdge {
            id: "edge:test:ab".to_string(),
            src: "node:test:a".to_string(),
            dst: "node:test:b".to_string(),
            rel: "related".to_string(),
            weight: 1.0,
            meta: Value::Null,
        }];

        let snap_a = SemanticSnapshotGraph::new(nodes_a, edges_a, ws_id.clone());
        let snap_b = SemanticSnapshotGraph::new(nodes_b, edges_b, ws_id);
        assert_eq!(snap_a.fingerprint()?, snap_b.fingerprint()?);
        Ok(())
    }
}
