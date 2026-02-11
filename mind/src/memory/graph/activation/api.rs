use crate::memory::graph::semantic::types::{SemanticEdge, SemanticNode};
use std::collections::{HashMap, HashSet};

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
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

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

    out_nodes.sort_by(|a, b| b.activation.partial_cmp(&a.activation).unwrap_or(std::cmp::Ordering::Equal));

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
