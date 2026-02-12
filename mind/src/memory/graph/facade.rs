use crate::memory::graph::activation;
use crate::memory::graph::semantic::types::{SemanticEdge, SemanticNode};
use crate::memory::graph::store::global_knowledge_store::GlobalKnowledgeStore;
use crate::memory::graph::store::workspace_sqlite_store::WorkspaceSqliteStore;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

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

        let file = File::create(out_path)?;
        let mut out = BufWriter::new(file);

        match format {
            GraphExportFormat::Dot => {
                writeln!(out, "digraph yai_graph {{")?;
                for n in &nodes {
                    writeln!(out, "  \"{}\" [label=\"{}\"];", escape_dot(&n.id), escape_dot(&n.kind))?;
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
                        serde_json::json!({"kind":"node","id":n.id,"type":n.kind,"meta":n.meta}).to_string()
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
        let store = store_for_scope(&scope);
        let nodes = store.list_nodes()?;
        let edges = store.list_edges()?;
        let semantic_nodes: Vec<SemanticNode> = nodes
            .iter()
            .map(|n| SemanticNode {
                id: n.id.clone(),
                kind: n.kind.clone(),
                meta: n.meta.clone(),
                last_seen: n.last_seen,
                // Placeholder until GraphNode has a dedicated created_ts field.
                created_ts: n.last_seen,
                expires_at: None,
                retention_policy_id: "none".to_string(),
                tombstone: false,
                compliance: None,
            })
            .collect();
        let semantic_edges: Vec<SemanticEdge> = edges
            .iter()
            .map(|e| SemanticEdge {
                id: e.id.clone(),
                src: e.src.clone(),
                dst: e.dst.clone(),
                rel: e.rel.clone(),
                weight: e.weight,
            })
            .collect();

        let act = activation::api::activate(
            &semantic_nodes,
            &semantic_edges,
            seeds,
            policy.hops,
            policy.decay,
            policy.threshold,
            policy.top_n,
        );

        let node_lookup: HashMap<String, GraphNode> =
            nodes.into_iter().map(|n| (n.id.clone(), n)).collect();
        let edge_lookup: HashMap<String, GraphEdge> =
            edges.into_iter().map(|e| (e.id.clone(), e)).collect();

        let mut scores = BTreeMap::new();
        let mut out_nodes = Vec::new();
        for n in act.nodes {
            scores.insert(n.id.clone(), n.activation);
            let original = node_lookup.get(&n.id);
            out_nodes.push(GraphNode {
                id: n.id,
                kind: n.kind,
                meta: original
                    .map(|v| v.meta.clone())
                    .unwrap_or(Value::Null),
                last_seen: original
                    .map(|v| v.last_seen)
                    .unwrap_or_else(|| n.last_seen.unwrap_or(0)),
            });
        }
        let out_edges = act
            .edges
            .into_iter()
            .map(|e| GraphEdge {
                id: e.id.clone(),
                src: e.src.clone(),
                dst: e.dst.clone(),
                rel: e.rel.clone(),
                weight: e.weight,
                meta: edge_lookup
                    .get(&e.id)
                    .map(|v| v.meta.clone())
                    .unwrap_or(Value::Null),
            })
            .collect();
        Ok(ActivationResult {
            nodes: out_nodes,
            edges: out_edges,
            scores,
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
                || node.kind.to_ascii_lowercase().contains(&k.to_ascii_lowercase())
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
        let d1 = GraphFacade::neighbors(scope.clone(), "node:test:a", 1, NeighborFilters::default())?;
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
}
