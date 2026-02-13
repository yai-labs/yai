use crate::cognition::memory::graph::activation::store;
use crate::cognition::memory::graph::semantic::types::{SemanticEdge, SemanticNode};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};

pub type NodeId = String;
pub const ACTIVATION_ALGO_VERSION: u8 = 1;
pub const QUANTIZE_SCALE: i64 = 1_000_000_000_000;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActivationSeed {
    pub node: NodeId,
    pub weight: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivationMethod {
    LocalPush,
    PowerIteration,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActivationParams {
    pub alpha: f64,
    pub epsilon: f64,
    pub max_push: usize,
    pub max_nodes: usize,
    pub top_k: usize,
    pub method: ActivationMethod,
    pub quantize_scale: i64,
}

impl Default for ActivationParams {
    fn default() -> Self {
        Self {
            alpha: 0.15,
            epsilon: 1e-6,
            max_push: 50_000,
            max_nodes: 100_000,
            top_k: 16,
            method: ActivationMethod::LocalPush,
            quantize_scale: QUANTIZE_SCALE,
        }
    }
}

impl ActivationParams {
    pub fn validate(&self) -> Result<()> {
        if !(0.0..1.0).contains(&self.alpha) {
            bail!("alpha must be in (0,1)");
        }
        if !(self.epsilon.is_finite() && self.epsilon > 0.0) {
            bail!("epsilon must be finite and > 0");
        }
        if self.max_push == 0 || self.max_nodes == 0 || self.top_k == 0 {
            bail!("max_push/max_nodes/top_k must be > 0");
        }
        if self.quantize_scale != QUANTIZE_SCALE {
            bail!("quantize_scale must be {QUANTIZE_SCALE}");
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActivationHit {
    pub node: NodeId,
    pub score_q: i64,
    pub score: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActivationStats {
    pub pushed: usize,
    pub visited: usize,
    pub residual_mass: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActivationResult {
    pub run_id: String,
    pub commit_hash: String,
    pub hits: Vec<ActivationHit>,
    pub stats: ActivationStats,
}

pub trait ActivationGraph {
    fn neighbors_out(&self, n: NodeId) -> Result<Vec<(NodeId, f64)>>;
    fn out_norm(&self, n: NodeId) -> Result<f64>;
    fn fingerprint(&self) -> Result<String>;
}

pub fn run_activation(
    graph: &dyn ActivationGraph,
    seeds: &[ActivationSeed],
    params: &ActivationParams,
) -> Result<ActivationResult> {
    params.validate()?;
    let seeds = canonicalize_seeds(seeds)?;
    let graph_fingerprint = graph.fingerprint()?;

    let (scores, stats) = match params.method {
        ActivationMethod::LocalPush => store::compute_local_push(graph, &seeds, params)?,
        ActivationMethod::PowerIteration => store::compute_power_iteration(graph, &seeds, params)?,
    };

    let mut ranked: Vec<(NodeId, f64, i64)> = Vec::new();
    for (node, score) in scores {
        if score <= 0.0 {
            continue;
        }
        let score_q = quantize_score(score, params.quantize_scale)?;
        ranked.push((node, score, score_q));
    }
    ranked.sort_by(|a, b| {
        b.2.cmp(&a.2)
            .then_with(|| a.0.cmp(&b.0))
            .then_with(|| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal))
    });
    ranked.truncate(params.top_k);

    let hits: Vec<ActivationHit> = ranked
        .into_iter()
        .map(|(node, score, score_q)| ActivationHit {
            node,
            score_q,
            score,
        })
        .collect();

    let commit_hash = commit_hash(&graph_fingerprint, &seeds, params, &hits)?;
    let run_id = run_id(&graph_fingerprint, &seeds, params)?;

    Ok(ActivationResult {
        run_id,
        commit_hash,
        hits,
        stats,
    })
}

pub fn canonicalize_seeds(seeds: &[ActivationSeed]) -> Result<Vec<ActivationSeed>> {
    if seeds.is_empty() {
        bail!("at least one seed is required");
    }
    let mut acc: BTreeMap<NodeId, f64> = BTreeMap::new();
    for seed in seeds {
        if seed.node.trim().is_empty() {
            bail!("seed node must not be empty");
        }
        if !seed.weight.is_finite() || seed.weight < 0.0 {
            bail!("seed weight must be finite and >= 0");
        }
        *acc.entry(seed.node.clone()).or_insert(0.0) += seed.weight;
    }

    let total: f64 = acc.values().sum();
    if total <= 0.0 || !total.is_finite() {
        bail!("seed weights must sum to > 0");
    }

    Ok(acc
        .into_iter()
        .map(|(node, weight)| ActivationSeed {
            node,
            weight: weight / total,
        })
        .collect())
}

pub fn quantize_score(score: f64, scale: i64) -> Result<i64> {
    if !score.is_finite() {
        bail!("activation score must be finite");
    }
    let scaled = score * scale as f64;
    if !scaled.is_finite() {
        bail!("activation score overflow");
    }
    let floored = scaled.floor();
    let max = i64::MAX as f64;
    let min = i64::MIN as f64;
    if floored >= max {
        return Ok(i64::MAX);
    }
    if floored <= min {
        return Ok(i64::MIN);
    }
    Ok(floored as i64)
}

pub fn run_id(
    graph_fingerprint: &str,
    seeds: &[ActivationSeed],
    params: &ActivationParams,
) -> Result<String> {
    let mut bytes = Vec::new();
    bytes.push(ACTIVATION_ALGO_VERSION);
    append_str(&mut bytes, graph_fingerprint);
    append_seeds(&mut bytes, seeds);
    append_params(&mut bytes, params);
    Ok(format!("act:{}", blake3::hash(&bytes).to_hex()))
}

pub fn commit_hash(
    graph_fingerprint: &str,
    seeds: &[ActivationSeed],
    params: &ActivationParams,
    hits: &[ActivationHit],
) -> Result<String> {
    let mut bytes = Vec::new();
    bytes.push(ACTIVATION_ALGO_VERSION);
    append_str(&mut bytes, graph_fingerprint);
    append_seeds(&mut bytes, seeds);
    append_params(&mut bytes, params);
    append_hits(&mut bytes, hits);
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

pub fn hash_params(params: &ActivationParams) -> Result<String> {
    let mut bytes = Vec::new();
    bytes.push(ACTIVATION_ALGO_VERSION);
    append_f64(&mut bytes, params.epsilon);
    append_u64(&mut bytes, params.max_push as u64);
    append_u64(&mut bytes, params.top_k as u64);
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

pub fn hash_seeds(seeds: &[ActivationSeed]) -> Result<String> {
    let mut bytes = Vec::new();
    append_seeds(&mut bytes, seeds);
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

// Legacy compatibility API retained for existing call sites.
#[derive(Debug, Clone)]
pub struct ActivatedNode {
    pub id: String,
    pub kind: String,
    pub activation: f32,
    pub last_seen: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ActivatedResult {
    pub nodes: Vec<ActivatedNode>,
    pub edges: Vec<SemanticEdge>,
}

pub fn activate(
    nodes: &[SemanticNode],
    edges: &[SemanticEdge],
    seeds: &[(String, f32)],
    hops: usize,
    decay: f32,
    threshold: f32,
    top_n: usize,
) -> ActivatedResult {
    let mut activation: HashMap<String, f32> = HashMap::new();
    let mut frontier: Vec<String> = Vec::new();

    for (id, _score) in seeds {
        activation.insert(id.clone(), 1.0);
        frontier.push(id.clone());
    }

    for _ in 0..hops {
        let mut next: HashSet<String> = HashSet::new();
        for n in &frontier {
            let act = *activation.get(n).unwrap_or(&0.0);
            for e in edges.iter().filter(|e| &e.src == n) {
                let inc = act * e.weight * decay;
                if inc == 0.0 {
                    continue;
                }
                let entry = activation.entry(e.dst.clone()).or_insert(0.0);
                *entry += inc;
                next.insert(e.dst.clone());
            }
        }
        frontier = next.into_iter().collect();
        if frontier.is_empty() {
            break;
        }
    }

    let mut scored: Vec<(String, f32)> = activation.iter().map(|(k, v)| (k.clone(), *v)).collect();
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });

    let mut selected: HashSet<String> = scored
        .iter()
        .filter(|(_, a)| *a >= threshold)
        .map(|(id, _)| id.clone())
        .collect();

    if selected.is_empty() {
        selected = scored
            .iter()
            .take(top_n.max(1))
            .map(|(id, _)| id.clone())
            .collect();
    }

    let mut out_nodes: Vec<ActivatedNode> = nodes
        .iter()
        .filter(|n| selected.contains(&n.id))
        .map(|n| ActivatedNode {
            id: n.id.clone(),
            kind: n.kind.clone(),
            activation: *activation.get(&n.id).unwrap_or(&0.0),
            last_seen: Some(n.last_seen),
        })
        .collect();

    out_nodes.sort_by(|a, b| {
        b.activation
            .partial_cmp(&a.activation)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.id.cmp(&b.id))
    });

    let out_edges: Vec<SemanticEdge> = edges
        .iter()
        .filter(|e| selected.contains(&e.src) && selected.contains(&e.dst))
        .cloned()
        .collect();

    ActivatedResult {
        nodes: out_nodes,
        edges: out_edges,
    }
}

fn append_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn append_i64(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn append_f64(out: &mut Vec<u8>, value: f64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn append_str(out: &mut Vec<u8>, value: &str) {
    append_u64(out, value.len() as u64);
    out.extend_from_slice(value.as_bytes());
}

fn append_seeds(out: &mut Vec<u8>, seeds: &[ActivationSeed]) {
    append_u64(out, seeds.len() as u64);
    for seed in seeds {
        append_str(out, &seed.node);
        append_f64(out, seed.weight);
    }
}

fn append_params(out: &mut Vec<u8>, params: &ActivationParams) {
    append_f64(out, params.alpha);
    append_f64(out, params.epsilon);
    append_u64(out, params.max_push as u64);
    append_u64(out, params.max_nodes as u64);
    append_u64(out, params.top_k as u64);
    out.push(match params.method {
        ActivationMethod::LocalPush => 0,
        ActivationMethod::PowerIteration => 1,
    });
    append_i64(out, params.quantize_scale);
}

fn append_hits(out: &mut Vec<u8>, hits: &[ActivationHit]) {
    append_u64(out, hits.len() as u64);
    for hit in hits {
        append_str(out, &hit.node);
        append_i64(out, hit.score_q);
    }
}
