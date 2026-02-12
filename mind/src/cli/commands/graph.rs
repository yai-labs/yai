use crate::cli::config::RuntimeConfig;
use crate::cognition::memory::graph::facade::{
    GraphEdge, GraphExportFormat, GraphFacade, GraphNode, GraphScope, NeighborFilters,
};
use crate::control::lifecycle::awareness::{
    awareness_log_path, run_awareness_with_config, AwarenessConfig,
};
use anyhow::{bail, Result};
use serde_json::Value;
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn resolve_scope(cfg: &RuntimeConfig, ws: Option<&str>, global: bool) -> Result<GraphScope> {
    if global {
        if ws.is_some() {
            bail!("use either --ws or --global");
        }
        return Ok(GraphScope::Global);
    }
    let scope_ws = ws
        .map(ToString::to_string)
        .unwrap_or_else(|| cfg.ws_default.clone());
    Ok(GraphScope::Workspace(scope_ws))
}

pub fn add_node(
    cfg: &RuntimeConfig,
    ws: Option<&str>,
    global: bool,
    id: &str,
    kind: &str,
    meta: &str,
) -> Result<()> {
    let scope = resolve_scope(cfg, ws, global)?;
    let meta_val: Value = serde_json::from_str(meta)?;
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    GraphFacade::put_node(
        scope,
        GraphNode {
            id: id.to_string(),
            kind: kind.to_string(),
            meta: meta_val,
            last_seen: ts,
        },
    )?;
    println!("ok node id={id}");
    Ok(())
}

pub fn add_edge(
    cfg: &RuntimeConfig,
    ws: Option<&str>,
    global: bool,
    src: &str,
    dst: &str,
    rel: &str,
    weight: f32,
) -> Result<()> {
    let scope = resolve_scope(cfg, ws, global)?;
    let edge_id = format!("edge:{}:{}:{}", rel, src, dst);
    GraphFacade::put_edge(
        scope,
        GraphEdge {
            id: edge_id.clone(),
            src: src.to_string(),
            dst: dst.to_string(),
            rel: rel.to_string(),
            weight,
            meta: Value::Null,
        },
    )?;
    println!("ok edge id={edge_id}");
    Ok(())
}

pub fn query(
    cfg: &RuntimeConfig,
    ws: Option<&str>,
    global: bool,
    text: &str,
    k: usize,
) -> Result<()> {
    let scope = resolve_scope(cfg, ws, global)?;
    let mut seeds = Vec::new();
    let sub = GraphFacade::neighbors(scope.clone(), text, 0, NeighborFilters::default())?;
    if !sub.nodes.is_empty() {
        seeds.push((text.to_string(), 1.0));
    } else {
        for node in
            GraphFacade::neighbors(scope.clone(), text, 1, NeighborFilters::default())?.nodes
        {
            seeds.push((node.id, 1.0));
            if seeds.len() >= k {
                break;
            }
        }
    }
    if seeds.is_empty() {
        // fallback to deterministic first nodes by lexicographic order
        let stats = GraphFacade::stats(scope.clone())?;
        println!(
            "nodes: 0\nedges: 0\nnote: no direct seed match for text={text}; backend={}",
            stats.backend
        );
        return Ok(());
    }
    let top_k = k * 2;
    let commit = GraphFacade::activate_and_commit(
        scope,
        &seeds
            .iter()
            .map(|(id, weight)| (id.clone(), *weight as f64))
            .collect::<Vec<_>>(),
        {
            let mut params =
                crate::cognition::memory::graph::activation::api::ActivationParams::default();
            params.top_k = top_k.max(1);
            params
        },
    )?;
    println!("activation_id: {}", commit.activation_id);
    println!("trace_hash: {}", commit.trace_hash);
    println!("algo: {}", commit.algo_id);
    println!("nodes: {}", commit.result.hits.len());
    let kind_by_id = GraphFacade::list_nodes(resolve_scope(cfg, ws, global)?)?
        .into_iter()
        .map(|node| (node.id, node.kind))
        .collect::<std::collections::HashMap<_, _>>();
    for n in &commit.result.hits {
        let kind = kind_by_id
            .get(&n.node)
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        println!("node {} kind={} score={:.6}", n.node, kind, n.score);
    }
    println!(
        "metrics: pushed={} visited={} residual_mass={:.6e} converged={}",
        commit.metrics.pushed,
        commit.metrics.visited,
        commit.metrics.residual_mass,
        commit.proof.converged
    );
    for hit in commit.trace.topk.iter().take(3) {
        println!(
            "trace_top {} score_q={} score={:.6}",
            hit.node, hit.score_q, hit.score
        );
    }
    Ok(())
}

pub fn activate(
    cfg: &RuntimeConfig,
    ws: Option<&str>,
    global: bool,
    seeds: &[String],
    topk: usize,
    alpha: f64,
    epsilon: f64,
    no_trace: bool,
) -> Result<()> {
    let scope = resolve_scope(cfg, ws, global)?;
    let seed_items = seeds
        .iter()
        .map(|s| (s.clone(), 1.0_f64))
        .collect::<Vec<_>>();
    let mut params = crate::cognition::memory::graph::activation::api::ActivationParams::default();
    params.top_k = topk.max(1);
    params.alpha = alpha;
    params.epsilon = epsilon;

    let commit = GraphFacade::activate_and_commit(scope, &seed_items, params)?;
    println!("run_id: {}", commit.activation_id);
    println!("commit_hash: {}", commit.result.commit_hash);
    println!("trace_hash: {}", commit.trace_hash);
    for hit in &commit.result.hits {
        println!(
            "node={} score={:.6} score_q={}",
            hit.node, hit.score, hit.score_q
        );
    }
    println!(
        "stats pushed={} visited={} residual_mass={:.6e}",
        commit.result.stats.pushed, commit.result.stats.visited, commit.result.stats.residual_mass
    );
    if no_trace {
        println!("note: --no-trace requested, trace was still committed by facade policy");
    }
    Ok(())
}

pub fn trace_show(run_id: &str) -> Result<()> {
    if run_id == "latest" {
        let list = crate::cognition::memory::graph::activation::store::list_traces(1)?;
        if list.is_empty() {
            bail!("no traces found");
        }
        return trace_show(&list[0].0);
    }
    let trace = crate::cognition::memory::graph::activation::store::load_trace(run_id)?
        .ok_or_else(|| anyhow::anyhow!("trace not found: {run_id}"))?;
    println!("run_id: {}", trace.run_id);
    println!("created_at_unix: {}", trace.created_at_unix);
    println!("graph_fingerprint: {}", trace.graph_fingerprint);
    println!("commit_hash: {}", trace.commit_hash);
    println!(
        "stats pushed={} visited={} residual_mass={:.6e}",
        trace.stats.pushed, trace.stats.visited, trace.stats.residual_mass
    );
    for hit in trace.topk {
        println!(
            "top {} score={:.6} score_q={}",
            hit.node, hit.score, hit.score_q
        );
    }
    Ok(())
}

pub fn stats(cfg: &RuntimeConfig, ws: Option<&str>, global: bool) -> Result<()> {
    let scope = resolve_scope(cfg, ws, global)?;
    let s = GraphFacade::stats(scope)?;
    println!("scope: {}", s.scope);
    println!("backend: {}", s.backend);
    println!("nodes: {}", s.nodes);
    println!("edges: {}", s.edges);
    for (k, v) in s.categories {
        println!("category.{k}: {v}");
    }
    Ok(())
}

pub fn node(
    cfg: &RuntimeConfig,
    ws: Option<&str>,
    global: bool,
    id: &str,
    limit: usize,
) -> Result<()> {
    let scope = resolve_scope(cfg, ws, global)?;
    let node = GraphFacade::get_node(scope.clone(), id)?;
    let Some(node) = node else {
        bail!("node not found: {id}");
    };
    let sub = GraphFacade::neighbors(scope, id, 1, NeighborFilters::default())?;
    println!("{}", serde_json::to_string_pretty(&node)?);
    let mut in_edges = Vec::new();
    let mut out_edges = Vec::new();
    for e in sub.edges {
        if e.src == id {
            out_edges.push(e);
        } else if e.dst == id {
            in_edges.push(e);
        }
    }
    in_edges.sort_by(|a, b| a.id.cmp(&b.id));
    out_edges.sort_by(|a, b| a.id.cmp(&b.id));
    in_edges.truncate(limit);
    out_edges.truncate(limit);
    println!("in_edges: {}", in_edges.len());
    for e in in_edges {
        println!("in  {} <- {} rel={} w={}", e.dst, e.src, e.rel, e.weight);
    }
    println!("out_edges: {}", out_edges.len());
    for e in out_edges {
        println!("out {} -> {} rel={} w={}", e.src, e.dst, e.rel, e.weight);
    }
    Ok(())
}

pub fn neighbors(
    cfg: &RuntimeConfig,
    ws: Option<&str>,
    global: bool,
    id: &str,
    depth: usize,
    rels: &[String],
    kinds: &[String],
) -> Result<()> {
    let scope = resolve_scope(cfg, ws, global)?;
    let filters = NeighborFilters {
        rels: if rels.is_empty() {
            None
        } else {
            Some(rels.iter().cloned().collect::<HashSet<_>>())
        },
        kinds: if kinds.is_empty() {
            None
        } else {
            Some(kinds.iter().cloned().collect::<HashSet<_>>())
        },
        directed: false,
    };
    let sg = GraphFacade::neighbors(scope, id, depth, filters)?;
    println!("nodes: {}", sg.nodes.len());
    println!("edges: {}", sg.edges.len());
    for n in sg.nodes {
        println!("node {} kind={}", n.id, n.kind);
    }
    for e in sg.edges {
        println!("edge {} -> {} rel={} w={}", e.src, e.dst, e.rel, e.weight);
    }
    Ok(())
}

pub fn export(
    cfg: &RuntimeConfig,
    ws: Option<&str>,
    global: bool,
    format: &str,
    out: &str,
) -> Result<()> {
    let scope = resolve_scope(cfg, ws, global)?;
    let fmt = match format {
        "dot" => GraphExportFormat::Dot,
        "jsonl" => GraphExportFormat::Jsonl,
        _ => bail!("unsupported export format: {format}"),
    };
    let out_path = PathBuf::from(out);
    GraphFacade::export(scope, fmt, &out_path)?;
    let size = std::fs::metadata(&out_path)?.len();
    println!("exported {} bytes to {}", size, out_path.display());
    Ok(())
}

pub fn awareness(
    _cfg: &RuntimeConfig,
    ws: &str,
    tick_ms: u64,
    max_steps: Option<u64>,
) -> Result<()> {
    run_awareness_with_config(ws, AwarenessConfig { tick_ms, max_steps })?;
    let p = awareness_log_path(ws);
    println!("awareness_log={}", p.display());
    Ok(())
}
