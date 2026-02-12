use crate::cli::paths;
use crate::cognition::memory::graph::activation::api::{
    ActivationGraph, ActivationParams, ActivationSeed, ActivationStats, NodeId,
};
use crate::cognition::memory::graph::activation::trace::ActivationTrace;
use anyhow::{anyhow, bail, Result};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub fn compute_local_push(
    graph: &dyn ActivationGraph,
    seeds: &[ActivationSeed],
    params: &ActivationParams,
) -> Result<(BTreeMap<NodeId, f64>, ActivationStats)> {
    let mut p: BTreeMap<NodeId, f64> = BTreeMap::new();
    let mut r: BTreeMap<NodeId, f64> = BTreeMap::new();
    let mut visited: BTreeSet<NodeId> = BTreeSet::new();

    for seed in seeds {
        *r.entry(seed.node.clone()).or_insert(0.0) += seed.weight;
        visited.insert(seed.node.clone());
    }

    let mut pushed = 0usize;
    loop {
        let candidate = pick_push_candidate(graph, &r, params.epsilon)?;
        let Some((u, ru, out_norm)) = candidate else {
            break;
        };
        if pushed >= params.max_push {
            bail!(
                "activation bounded: max_push exceeded ({})",
                params.max_push
            );
        }

        r.remove(&u);
        *p.entry(u.clone()).or_insert(0.0) += params.alpha * ru;

        let remain = (1.0 - params.alpha) * ru;
        let neighbors = graph.neighbors_out(u.clone())?;
        if neighbors.is_empty() || out_norm <= 0.0 {
            *p.entry(u).or_insert(0.0) += remain;
        } else {
            let mut sorted_neighbors = neighbors;
            sorted_neighbors.sort_by(|a, b| a.0.cmp(&b.0));
            for (v, w) in sorted_neighbors {
                if w <= 0.0 || !w.is_finite() {
                    continue;
                }
                let delta = remain * (w / out_norm);
                if delta <= 0.0 || !delta.is_finite() {
                    continue;
                }
                *r.entry(v.clone()).or_insert(0.0) += delta;
                visited.insert(v);
                if visited.len() > params.max_nodes {
                    bail!(
                        "activation bounded: max_nodes exceeded ({})",
                        params.max_nodes
                    );
                }
            }
        }

        pushed += 1;
    }

    let residual_mass: f64 = r.values().sum();
    Ok((
        p,
        ActivationStats {
            pushed,
            visited: visited.len(),
            residual_mass,
        },
    ))
}

pub fn compute_power_iteration(
    graph: &dyn ActivationGraph,
    seeds: &[ActivationSeed],
    params: &ActivationParams,
) -> Result<(BTreeMap<NodeId, f64>, ActivationStats)> {
    let seed_vec: BTreeMap<NodeId, f64> =
        seeds.iter().map(|s| (s.node.clone(), s.weight)).collect();
    let mut p = seed_vec.clone();

    let mut pushed = 0usize;
    let mut visited: BTreeSet<NodeId> = seed_vec.keys().cloned().collect();

    while pushed < params.max_push {
        let mut next: BTreeMap<NodeId, f64> = BTreeMap::new();
        for (node, w) in &seed_vec {
            *next.entry(node.clone()).or_insert(0.0) += params.alpha * *w;
        }

        for (u, mass) in &p {
            if *mass <= 0.0 {
                continue;
            }
            let out_norm = graph.out_norm(u.clone())?;
            let mut neighbors = graph.neighbors_out(u.clone())?;
            neighbors.sort_by(|a, b| a.0.cmp(&b.0));
            if neighbors.is_empty() || out_norm <= 0.0 {
                *next.entry(u.clone()).or_insert(0.0) += (1.0 - params.alpha) * *mass;
                continue;
            }

            for (v, w) in neighbors {
                if w <= 0.0 || !w.is_finite() {
                    continue;
                }
                let delta = (1.0 - params.alpha) * *mass * (w / out_norm);
                if delta <= 0.0 || !delta.is_finite() {
                    continue;
                }
                *next.entry(v.clone()).or_insert(0.0) += delta;
                visited.insert(v);
            }
        }

        if visited.len() > params.max_nodes {
            bail!(
                "activation bounded: max_nodes exceeded ({})",
                params.max_nodes
            );
        }

        let mut keys: BTreeSet<NodeId> = BTreeSet::new();
        keys.extend(p.keys().cloned());
        keys.extend(next.keys().cloned());

        let mut delta_max = 0.0;
        for k in keys {
            let a = p.get(&k).copied().unwrap_or(0.0);
            let b = next.get(&k).copied().unwrap_or(0.0);
            let d = (a - b).abs();
            if d > delta_max {
                delta_max = d;
            }
        }

        p = next;
        pushed += 1;
        if delta_max <= params.epsilon {
            break;
        }
    }

    let residual_mass = (1.0 - p.values().sum::<f64>()).abs();
    Ok((
        p,
        ActivationStats {
            pushed,
            visited: visited.len(),
            residual_mass,
        },
    ))
}

fn pick_push_candidate(
    graph: &dyn ActivationGraph,
    residual: &BTreeMap<NodeId, f64>,
    epsilon: f64,
) -> Result<Option<(NodeId, f64, f64)>> {
    let mut best: Option<(NodeId, f64, f64, f64)> = None;
    for (node, ru) in residual {
        if *ru <= 0.0 {
            continue;
        }
        let out_norm = graph.out_norm(node.clone())?;
        let denom = if out_norm > 0.0 { out_norm } else { 1.0 };
        let ratio = *ru / denom;
        if ratio <= epsilon {
            continue;
        }
        match &best {
            None => best = Some((node.clone(), *ru, out_norm, ratio)),
            Some((best_node, _, _, best_ratio)) => {
                if ratio > *best_ratio
                    || ((ratio - *best_ratio).abs() <= f64::EPSILON
                        && node.cmp(best_node) == Ordering::Less)
                {
                    best = Some((node.clone(), *ru, out_norm, ratio));
                }
            }
        }
    }
    Ok(best.map(|(n, ru, out_norm, _)| (n, ru, out_norm)))
}

pub fn save_trace(trace: &ActivationTrace) -> Result<()> {
    let path = trace_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let line = serde_json::to_string(trace)?;
    writeln!(file, "{line}")?;
    Ok(())
}

pub fn load_trace(run_id: &str) -> Result<Option<ActivationTrace>> {
    let path = trace_path();
    if !path.exists() {
        return Ok(None);
    }
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let mut out = None;
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let trace: ActivationTrace = serde_json::from_str(&line)?;
        if trace.run_id == run_id {
            out = Some(trace);
        }
    }
    Ok(out)
}

pub fn list_traces(limit: usize) -> Result<Vec<(String, i64, String)>> {
    if limit == 0 {
        return Ok(Vec::new());
    }
    let path = trace_path();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let mut items: Vec<(String, i64, String)> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let trace: ActivationTrace = serde_json::from_str(&line)
            .map_err(|e| anyhow!("invalid activation trace row: {e}"))?;
        items.push((trace.run_id, trace.created_at_unix, trace.commit_hash));
    }

    items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    items.truncate(limit);
    Ok(items)
}

fn trace_path() -> std::path::PathBuf {
    paths::run_dir().join("activation_traces.jsonl")
}
