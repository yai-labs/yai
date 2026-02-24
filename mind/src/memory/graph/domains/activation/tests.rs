use super::api::{
    quantize_score, run_activation, ActivationGraph, ActivationMethod, ActivationParams,
    ActivationSeed, NodeId, QUANTIZE_SCALE,
};
use anyhow::Result;
use std::collections::BTreeMap;

#[derive(Clone, Default)]
struct TinyGraph {
    adj: BTreeMap<NodeId, Vec<(NodeId, f64)>>,
    fingerprint: String,
}

impl TinyGraph {
    fn with_edges(edges: &[(&str, &str, f64)]) -> Self {
        let mut adj: BTreeMap<NodeId, Vec<(NodeId, f64)>> = BTreeMap::new();
        for (s, d, w) in edges {
            adj.entry((*s).to_string())
                .or_default()
                .push(((*d).to_string(), *w));
            adj.entry((*d).to_string()).or_default();
        }
        for vals in adj.values_mut() {
            vals.sort_by(|a, b| a.0.cmp(&b.0));
        }
        let fingerprint = {
            let payload = serde_json::to_vec(&adj).unwrap_or_default();
            format!("tiny:{}", blake3::hash(&payload).to_hex())
        };
        Self { adj, fingerprint }
    }
}

impl ActivationGraph for TinyGraph {
    fn neighbors_out(&self, n: NodeId) -> Result<Vec<(NodeId, f64)>> {
        Ok(self.adj.get(&n).cloned().unwrap_or_default())
    }

    fn out_norm(&self, n: NodeId) -> Result<f64> {
        Ok(self
            .adj
            .get(&n)
            .map(|v| v.iter().map(|(_, w)| *w).sum())
            .unwrap_or(0.0))
    }

    fn fingerprint(&self) -> Result<String> {
        Ok(self.fingerprint.clone())
    }
}

fn seed(node: &str) -> Vec<ActivationSeed> {
    vec![ActivationSeed {
        node: node.to_string(),
        weight: 1.0,
    }]
}

#[test]
fn tiny_graph_sanity_stable_hash() -> Result<()> {
    let g = TinyGraph::with_edges(&[("a", "b", 1.0), ("b", "c", 1.0), ("a", "d", 0.2)]);
    let params = ActivationParams::default();
    let r1 = run_activation(&g, &seed("a"), &params)?;
    let r2 = run_activation(&g, &seed("a"), &params)?;

    assert_eq!(r1.commit_hash, r2.commit_hash);
    assert_eq!(r1.run_id, r2.run_id);
    Ok(())
}

#[test]
fn local_push_vs_power_iteration_small_graph() -> Result<()> {
    let g = TinyGraph::with_edges(&[
        ("a", "b", 1.0),
        ("a", "c", 1.0),
        ("b", "d", 1.0),
        ("c", "d", 1.0),
    ]);

    let mut p_push = ActivationParams::default();
    p_push.top_k = 10;
    p_push.epsilon = 1e-9;
    p_push.max_push = 200_000;
    p_push.method = ActivationMethod::LocalPush;

    let mut p_power = p_push.clone();
    p_power.method = ActivationMethod::PowerIteration;

    let r_push = run_activation(&g, &seed("a"), &p_push)?;
    let r_power = run_activation(&g, &seed("a"), &p_power)?;

    let mut map_push = BTreeMap::new();
    for h in &r_push.hits {
        map_push.insert(h.node.clone(), h.score);
    }
    let mut map_power = BTreeMap::new();
    for h in &r_power.hits {
        map_power.insert(h.node.clone(), h.score);
    }

    let mut l1 = 0.0;
    let mut keys = map_push.keys().cloned().collect::<Vec<_>>();
    for k in map_power.keys() {
        if !keys.contains(k) {
            keys.push(k.clone());
        }
    }
    keys.sort();

    for k in keys {
        let a = map_push.get(&k).copied().unwrap_or(0.0);
        let b = map_power.get(&k).copied().unwrap_or(0.0);
        l1 += (a - b).abs();
    }

    assert!(l1 < 0.1, "L1 too high: {l1}");
    Ok(())
}

#[test]
fn determinism_five_runs_same_commit() -> Result<()> {
    let g = TinyGraph::with_edges(&[("a", "b", 1.0), ("b", "c", 1.0), ("c", "a", 0.3)]);
    let params = ActivationParams::default();

    let mut baseline: Option<(String, String, Vec<(String, i64)>)> = None;

    for _ in 0..5 {
        let r = run_activation(&g, &seed("a"), &params)?;
        let hits = r
            .hits
            .iter()
            .map(|h| (h.node.clone(), h.score_q))
            .collect::<Vec<_>>();

        if let Some((hash, run_id, base_hits)) = &baseline {
            assert_eq!(hash, &r.commit_hash);
            assert_eq!(run_id, &r.run_id);
            assert_eq!(base_hits, &hits);
        } else {
            baseline = Some((r.commit_hash.clone(), r.run_id.clone(), hits));
        }
    }

    Ok(())
}

#[test]
fn bounds_max_push_fail() {
    let g = TinyGraph::with_edges(&[("a", "b", 1.0), ("b", "a", 1.0)]);
    let mut params = ActivationParams::default();
    params.max_push = 1;
    params.epsilon = 1e-18;

    let r = run_activation(&g, &seed("a"), &params).expect("activation run should succeed deterministically");
    assert!(r.hits.is_empty());
}

#[test]
fn bounds_max_nodes_fail() {
    let g = TinyGraph::with_edges(&[
        ("a", "b", 1.0),
        ("a", "c", 1.0),
        ("a", "d", 1.0),
        ("a", "e", 1.0),
    ]);

    let mut params = ActivationParams::default();
    params.max_nodes = 2;
    params.epsilon = 1e-18;

    let r = run_activation(&g, &seed("a"), &params).expect("activation run should succeed deterministically");
    assert!(r.hits.is_empty());
}

#[test]
fn activation_run_id_is_deterministic() -> Result<()> {
    let g = TinyGraph::with_edges(&[("a", "b", 1.0), ("b", "c", 1.0)]);
    let params = ActivationParams::default();
    let r1 = run_activation(&g, &seed("a"), &params)?;
    let r2 = run_activation(&g, &seed("a"), &params)?;
    assert_eq!(r1.run_id, r2.run_id);
    assert_eq!(r1.commit_hash, r2.commit_hash);
    Ok(())
}

#[test]
fn quantization_floor_and_bounds() -> Result<()> {
    assert_eq!(quantize_score(0.0, QUANTIZE_SCALE)?, 0);
    assert_eq!(quantize_score(1e-13, QUANTIZE_SCALE)?, 0);
    assert_eq!(quantize_score(1.0, QUANTIZE_SCALE)?, QUANTIZE_SCALE);
    assert!(quantize_score(f64::NAN, QUANTIZE_SCALE).is_err());
    assert!(quantize_score(f64::INFINITY, QUANTIZE_SCALE).is_err());
    assert!(quantize_score(f64::NEG_INFINITY, QUANTIZE_SCALE).is_err());
    Ok(())
}
