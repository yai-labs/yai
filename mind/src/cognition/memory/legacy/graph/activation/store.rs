use crate::cli::paths;
use crate::cognition::memory::graph::activation::api::{
    ActivationGraph, ActivationParams, ActivationSeed, ActivationStats, NodeId,
};
use crate::cognition::memory::graph::activation::trace::ActivationTrace;
use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivationRunMeta {
    pub run_id: String,
    pub ws_id: String,
    pub created_at_unix: i64,
    pub graph_fingerprint: String,
    pub params_hash: String,
    pub seeds_hash: String,
    pub commit_hash: String,
    pub params: ActivationParams,
    pub seeds: Vec<ActivationSeed>,
    pub stats: ActivationStats,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivationResultRow {
    pub node_id: NodeId,
    pub rank: i64,
    pub score_q: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivationExplanationRow {
    pub run_id: String,
    pub from_node_id: NodeId,
    pub to_node_id: NodeId,
    pub contrib_q: i64,
    pub reason: String,
}

pub struct ActivationTraceStore {
    ws_id: String,
    db_path: PathBuf,
}

impl ActivationTraceStore {
    pub fn open(ws_id: &str) -> Result<Self> {
        let db_path = activation_db_path(ws_id);
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let store = Self {
            ws_id: ws_id.to_string(),
            db_path,
        };
        store.init()?;
        Ok(store)
    }

    pub fn path(&self) -> &Path {
        &self.db_path
    }

    pub fn record_run(
        &self,
        meta: &ActivationRunMeta,
        results: &[ActivationResultRow],
        explanations: Option<&[ActivationExplanationRow]>,
    ) -> Result<()> {
        if meta.ws_id != self.ws_id {
            bail!("activation trace ws_id mismatch");
        }
        let mut conn = self.conn()?;
        let tx = conn.transaction()?;

        let existing = load_run_meta(&tx, &meta.run_id)?;
        if let Some(existing) = existing {
            ensure_meta_matches(&existing, meta)?;
            let existing_results = load_run_results(&tx, &meta.run_id)?;
            ensure_results_match(&existing_results, results)?;
            return Ok(());
        }

        let params_json = serde_json::to_string(&meta.params)?;
        let seeds_json = serde_json::to_string(&meta.seeds)?;
        let stats_json = serde_json::to_string(&meta.stats)?;

        tx.execute(
            "INSERT INTO activation_runs (
                run_id, ws_id, created_at, graph_fingerprint, params_hash, seeds_hash, commit_hash,
                params_json, seeds_json, stats_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                meta.run_id,
                meta.ws_id,
                meta.created_at_unix,
                meta.graph_fingerprint,
                meta.params_hash,
                meta.seeds_hash,
                meta.commit_hash,
                params_json,
                seeds_json,
                stats_json,
            ],
        )?;

        for row in results {
            tx.execute(
                "INSERT INTO activation_results (run_id, node_id, rank, score_q)
                 VALUES (?1, ?2, ?3, ?4)",
                params![meta.run_id, row.node_id, row.rank, row.score_q],
            )?;
        }

        if let Some(rows) = explanations {
            for row in rows {
                tx.execute(
                    "INSERT INTO activation_explanations (run_id, from_node_id, to_node_id, contrib_q, reason)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        row.run_id,
                        row.from_node_id,
                        row.to_node_id,
                        row.contrib_q,
                        row.reason
                    ],
                )?;
            }
        }

        tx.commit()?;
        Ok(())
    }

    pub fn get_run(&self, run_id: &str) -> Result<Option<ActivationTrace>> {
        let conn = self.conn()?;
        let meta = load_run_meta(&conn, run_id)?;
        let Some(meta) = meta else {
            return Ok(None);
        };
        let results = load_run_results(&conn, run_id)?;
        Ok(Some(build_trace(meta, results)?))
    }

    pub fn list_runs(&self, limit: usize, offset: usize) -> Result<Vec<(String, i64, String)>> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT run_id, created_at, commit_hash
             FROM activation_runs
             WHERE ws_id = ?1
             ORDER BY created_at DESC, run_id ASC
             LIMIT ?2 OFFSET ?3",
        )?;
        let rows = stmt.query_map(params![self.ws_id, limit as i64, offset as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn purge_keep_last(&self, keep_last: usize) -> Result<usize> {
        let mut conn = self.conn()?;
        let to_delete: Vec<String> = {
            let mut stmt = conn.prepare(
                "SELECT run_id
                 FROM activation_runs
                 WHERE ws_id = ?1
                 ORDER BY created_at DESC, run_id ASC
                 LIMIT -1 OFFSET ?2",
            )?;
            let rows = stmt.query_map(params![self.ws_id, keep_last as i64], |row| row.get(0))?;
            let mut out = Vec::new();
            for row in rows {
                out.push(row?);
            }
            out
        };
        if to_delete.is_empty() {
            return Ok(0);
        }

        let tx = conn.transaction()?;
        for run_id in &to_delete {
            tx.execute(
                "DELETE FROM activation_results WHERE run_id = ?1",
                params![run_id],
            )?;
            tx.execute(
                "DELETE FROM activation_explanations WHERE run_id = ?1",
                params![run_id],
            )?;
            tx.execute(
                "DELETE FROM activation_runs WHERE run_id = ?1",
                params![run_id],
            )?;
        }
        tx.commit()?;
        Ok(to_delete.len())
    }

    fn conn(&self) -> Result<Connection> {
        Connection::open(&self.db_path).map_err(|e| anyhow!("open activation db: {e}"))
    }

    fn init(&self) -> Result<()> {
        let conn = self.conn()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS activation_runs (
                run_id TEXT PRIMARY KEY,
                ws_id TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                graph_fingerprint TEXT NOT NULL,
                params_hash TEXT NOT NULL,
                seeds_hash TEXT NOT NULL,
                commit_hash TEXT NOT NULL,
                params_json TEXT NOT NULL,
                seeds_json TEXT NOT NULL,
                stats_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS activation_results (
                run_id TEXT NOT NULL,
                node_id TEXT NOT NULL,
                rank INTEGER NOT NULL,
                score_q INTEGER NOT NULL,
                PRIMARY KEY (run_id, node_id)
            );
            CREATE TABLE IF NOT EXISTS activation_explanations (
                run_id TEXT NOT NULL,
                from_node_id TEXT NOT NULL,
                to_node_id TEXT NOT NULL,
                contrib_q INTEGER NOT NULL,
                reason TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS activation_runs_ws_idx ON activation_runs(ws_id);
            CREATE INDEX IF NOT EXISTS activation_runs_created_idx ON activation_runs(created_at);
            CREATE INDEX IF NOT EXISTS activation_results_run_idx ON activation_results(run_id);",
        )?;
        Ok(())
    }
}

fn activation_db_path(ws_id: &str) -> PathBuf {
    paths::run_dir().join(ws_id).join("activation.sqlite")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ActivationRunMetaRow {
    run_id: String,
    ws_id: String,
    created_at_unix: i64,
    graph_fingerprint: String,
    params_hash: String,
    seeds_hash: String,
    commit_hash: String,
    params_json: String,
    seeds_json: String,
    stats_json: String,
}

fn load_run_meta(conn: &Connection, run_id: &str) -> Result<Option<ActivationRunMetaRow>> {
    let mut stmt = conn.prepare(
        "SELECT run_id, ws_id, created_at, graph_fingerprint, params_hash, seeds_hash, commit_hash,
                params_json, seeds_json, stats_json
         FROM activation_runs
         WHERE run_id = ?1
         LIMIT 1",
    )?;
    let mut rows = stmt.query(params![run_id])?;
    if let Some(row) = rows.next()? {
        return Ok(Some(ActivationRunMetaRow {
            run_id: row.get(0)?,
            ws_id: row.get(1)?,
            created_at_unix: row.get(2)?,
            graph_fingerprint: row.get(3)?,
            params_hash: row.get(4)?,
            seeds_hash: row.get(5)?,
            commit_hash: row.get(6)?,
            params_json: row.get(7)?,
            seeds_json: row.get(8)?,
            stats_json: row.get(9)?,
        }));
    }
    Ok(None)
}

fn load_run_results(conn: &Connection, run_id: &str) -> Result<Vec<ActivationResultRow>> {
    let mut stmt = conn.prepare(
        "SELECT node_id, rank, score_q
         FROM activation_results
         WHERE run_id = ?1
         ORDER BY rank ASC, node_id ASC",
    )?;
    let rows = stmt.query_map(params![run_id], |row| {
        Ok(ActivationResultRow {
            node_id: row.get(0)?,
            rank: row.get(1)?,
            score_q: row.get(2)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

fn ensure_meta_matches(
    existing: &ActivationRunMetaRow,
    incoming: &ActivationRunMeta,
) -> Result<()> {
    if existing.ws_id != incoming.ws_id
        || existing.graph_fingerprint != incoming.graph_fingerprint
        || existing.params_hash != incoming.params_hash
        || existing.seeds_hash != incoming.seeds_hash
        || existing.commit_hash != incoming.commit_hash
    {
        bail!(
            "activation run metadata mismatch for run_id={}",
            incoming.run_id
        );
    }

    let params_json = serde_json::to_string(&incoming.params)?;
    let seeds_json = serde_json::to_string(&incoming.seeds)?;
    let stats_json = serde_json::to_string(&incoming.stats)?;
    if existing.params_json != params_json
        || existing.seeds_json != seeds_json
        || existing.stats_json != stats_json
    {
        bail!(
            "activation run payload mismatch for run_id={}",
            incoming.run_id
        );
    }
    Ok(())
}

fn ensure_results_match(
    existing: &[ActivationResultRow],
    incoming: &[ActivationResultRow],
) -> Result<()> {
    if existing.len() != incoming.len() {
        bail!("activation run results length mismatch");
    }
    let mut incoming_sorted = incoming.to_vec();
    incoming_sorted.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.node_id.cmp(&b.node_id)));
    for (a, b) in existing.iter().zip(incoming_sorted.iter()) {
        if a.node_id != b.node_id || a.rank != b.rank || a.score_q != b.score_q {
            bail!("activation run results mismatch");
        }
    }
    Ok(())
}

fn build_trace(
    meta: ActivationRunMetaRow,
    results: Vec<ActivationResultRow>,
) -> Result<ActivationTrace> {
    let params: ActivationParams = serde_json::from_str(&meta.params_json)?;
    let seeds: Vec<ActivationSeed> = serde_json::from_str(&meta.seeds_json)?;
    let stats: ActivationStats = serde_json::from_str(&meta.stats_json)?;
    let scale = params.quantize_scale as f64;
    let hits = results
        .into_iter()
        .map(
            |row| crate::cognition::memory::graph::activation::api::ActivationHit {
                node: row.node_id,
                score_q: row.score_q,
                score: row.score_q as f64 / scale,
            },
        )
        .collect();
    Ok(ActivationTrace {
        run_id: meta.run_id,
        created_at_unix: meta.created_at_unix,
        graph_fingerprint: meta.graph_fingerprint,
        params,
        seeds,
        commit_hash: meta.commit_hash,
        topk: hits,
        stats,
    })
}
