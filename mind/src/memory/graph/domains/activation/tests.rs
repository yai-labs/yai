use super::api::{
    quantize_score, run_activation, ActivationGraph, ActivationMethod, ActivationParams,
    ActivationSeed, NodeId, QUANTIZE_SCALE,
};
use anyhow::{bail, Result};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

    if r1.hits.is_empty() {
        bail!("empty hits");
    }
    assert_eq!(r1.commit_hash, r2.commit_hash);
    assert!(r1.hits.iter().any(|h| h.node == "a"));
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

    let err = run_activation(&g, &seed("a"), &params).expect_err("expected bounded failure");
    assert!(err.to_string().contains("max_push"));
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

    let err = run_activation(&g, &seed("a"), &params).expect_err("expected bounded failure");
    assert!(err.to_string().contains("max_nodes"));
}

#[test]
fn trace_store_roundtrip() -> Result<()> {
    let g = TinyGraph::with_edges(&[("a", "b", 1.0)]);
    let params = ActivationParams::default();
    let result = run_activation(&g, &seed("a"), &params)?;

    let trace = super::trace::ActivationTrace {
        run_id: result.run_id.clone(),
        created_at_unix: 1,
        graph_fingerprint: g.fingerprint()?,
        params,
        seeds: seed("a"),
        commit_hash: result.commit_hash.clone(),
        topk: result.hits.clone(),
        stats: result.stats.clone(),
    };

    let ws_id = format!(
        "activation_store_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );

    let store = super::store::ActivationTraceStore::open(&ws_id)?;

    let meta = super::store::ActivationRunMeta {
        run_id: trace.run_id.clone(),
        ws_id: ws_id.clone(),
        created_at_unix: trace.created_at_unix,
        graph_fingerprint: trace.graph_fingerprint.clone(),
        params_hash: super::api::hash_params(&trace.params)?,
        seeds_hash: super::api::hash_seeds(&trace.seeds)?,
        commit_hash: trace.commit_hash.clone(),
        params: trace.params.clone(),
        seeds: trace.seeds.clone(),
        stats: trace.stats.clone(),
    };

    let results = trace
        .topk
        .iter()
        .enumerate()
        .map(|(idx, hit)| super::store::ActivationResultRow {
            node_id: hit.node.clone(),
            rank: idx as i64 + 1,
            score_q: hit.score_q,
        })
        .collect::<Vec<_>>();

    store.record_run(&meta, &results, None)?;
    let loaded = store.get_run(&result.run_id)?;
    assert!(loaded.is_some());

    let list = store.list_runs(10, 0)?;
    assert!(!list.is_empty());

    Ok(())
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
fn activation_does_not_mutate_semantic_graph() -> Result<()> {
    use crate::memory::graph::facade::{GraphEdge, GraphFacade, GraphNode, GraphScope};
    use serde_json::Value;

    let ws_id = format!(
        "activation_semantic_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );
    let scope = GraphScope::Workspace(ws_id.clone());

    GraphFacade::put_node(
        scope.clone(),
        GraphNode {
            id: "node:test:a".to_string(),
            kind: "semantic".to_string(),
            meta: serde_json::json!({"name": "a"}),
            last_seen: 1,
        },
    )?;
    GraphFacade::put_node(
        scope.clone(),
        GraphNode {
            id: "node:test:b".to_string(),
            kind: "semantic".to_string(),
            meta: serde_json::json!({"name": "b"}),
            last_seen: 2,
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

    let before_fp = GraphFacade::graph_fingerprint(scope.clone())?;
    let before_stats = GraphFacade::stats(scope.clone())?;

    let mut params = ActivationParams::default();
    params.top_k = 4;
    let seeds = vec![("node:test:a".to_string(), 1.0)];

    let _commit = GraphFacade::activate_and_commit_with_trace(scope.clone(), &seeds, params, true)?;

    let after_fp = GraphFacade::graph_fingerprint(scope.clone())?;
    let after_stats = GraphFacade::stats(scope)?;

    assert_eq!(before_fp, after_fp);
    assert_eq!(before_stats.nodes, after_stats.nodes);
    assert_eq!(before_stats.edges, after_stats.edges);

    Ok(())
}

#[test]
fn activation_trace_is_prunable_without_semantic_loss() -> Result<()> {
    use crate::memory::graph::facade::{GraphEdge, GraphFacade, GraphNode, GraphScope};
    use serde_json::Value;

    let ws_id = format!(
        "activation_prune_{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );
    let scope = GraphScope::Workspace(ws_id.clone());

    GraphFacade::put_node(
        scope.clone(),
        GraphNode {
            id: "node:test:a".to_string(),
            kind: "semantic".to_string(),
            meta: serde_json::json!({"name": "a"}),
            last_seen: 1,
        },
    )?;
    GraphFacade::put_node(
        scope.clone(),
        GraphNode {
            id: "node:test:b".to_string(),
            kind: "semantic".to_string(),
            meta: serde_json::json!({"name": "b"}),
            last_seen: 2,
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

    let before_fp = GraphFacade::graph_fingerprint(scope.clone())?;

    let mut params = ActivationParams::default();
    params.top_k = 4;

    let seeds = vec![("node:test:a".to_string(), 1.0)];
    let _commit = GraphFacade::activate_and_commit_with_trace(scope.clone(), &seeds, params, true)?;

    let store = super::store::ActivationTraceStore::open(&ws_id)?;
    let removed = store.purge_keep_last(0)?;
    assert!(removed >= 1);

    let after_fp = GraphFacade::graph_fingerprint(scope)?;
    assert_eq!(before_fp, after_fp);

    Ok(())
}

#[test]
fn workspace_isolation_commit_differs() -> Result<()> {
    use crate::memory::graph::facade::{GraphEdge, GraphFacade, GraphNode, GraphScope};
    use serde_json::Value;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let scope_a = GraphScope::Workspace(format!("activation_ws_a_{now}"));
    let scope_b = GraphScope::Workspace(format!("activation_ws_b_{now}"));

    for scope in [scope_a.clone(), scope_b.clone()] {
        GraphFacade::put_node(
            scope.clone(),
            GraphNode {
                id: "node:test:a".to_string(),
                kind: "semantic".to_string(),
                meta: serde_json::json!({"name": "a"}),
                last_seen: 1,
            },
        )?;
        GraphFacade::put_node(
            scope.clone(),
            GraphNode {
                id: "node:test:b".to_string(),
                kind: "semantic".to_string(),
                meta: serde_json::json!({"name": "b"}),
                last_seen: 2,
            },
        )?;
        GraphFacade::put_edge(
            scope,
            GraphEdge {
                id: "edge:test:ab".to_string(),
                src: "node:test:a".to_string(),
                dst: "node:test:b".to_string(),
                rel: "related".to_string(),
                weight: 1.0,
                meta: Value::Null,
            },
        )?;
    }

    let params = ActivationParams::default();
    let seeds = vec![("node:test:a".to_string(), 1.0)];

    let a = GraphFacade::activate_and_commit(scope_a, &seeds, params.clone())?;
    let b = GraphFacade::activate_and_commit(scope_b, &seeds, params)?;

    assert_ne!(a.result.commit_hash, b.result.commit_hash);
    assert_eq!(
        a.result
            .hits
            .iter()
            .map(|h| (h.node.clone(), h.score_q))
            .collect::<Vec<_>>(),
        b.result
            .hits
            .iter()
            .map(|h| (h.node.clone(), h.score_q))
            .collect::<Vec<_>>()
    );

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
