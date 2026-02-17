# ACTIVATION_V1

Canonical sources:

- `deps/yai-specs/graph/GRAPH_V1.md`
- `deps/yai-specs/graph/graph.v1.json`

## Objective
Activation computes memory emergence as graph diffusion (PPR/RWR), with deterministic commit and persisted activation traces.

## Separation Rule (Normative)
Activation MUST NOT modify semantic nodes/edges. Activation traces are derived operational artifacts stored in a separate activation trace store.

## Determinism Contract (Normative)
Activation MUST satisfy:
- Pure function of `(ws_id, snapshot_bytes, seeds, params, algo_version)`.
- No external state reads (no daemon files, provider lifecycle, global mutable state).
- No timestamps or RNG in compute.
- Identical inputs yield identical `commit_hash` across restarts.
- Execution is independent of insertion order (canonical ordering).

## Public API (L3/Mind)
`mind/src/cognition/memory/graph/activation/api.rs`

- `ActivationSeed { node, weight }`
- `ActivationMethod { LocalPush, PowerIteration }`
- `ActivationParams { alpha, epsilon, max_push, max_nodes, top_k, method, quantize_scale }`
- `ActivationHit { node, score_q, score }`
- `ActivationStats { pushed, visited, residual_mass }`
- `ActivationResult { run_id, commit_hash, hits, stats }`
- `ActivationGraph` trait:
  - `neighbors_out(node)`
  - `out_norm(node)`
  - `fingerprint()`
- `run_activation(graph, seeds, params)`

## Algorithms
### LocalPush (default)
Residual-based PPR approximation:
- maintain `p` and `r`
- initialize `r` with normalized seeds
- push while `r[u]/out_norm(u) > epsilon`
- update:
  - `p[u] += alpha * r[u]`
  - `remain = (1-alpha) * r[u]`
  - distribute `remain` on out-neighbors by normalized edge weights
  - `r[u] = 0`

Deterministic push selection and deterministic neighbor ordering are mandatory.

### PowerIteration (baseline)
- iterate `p_next = alpha*s + (1-alpha)*(P^T * p)`
- stop on `delta <= epsilon` or bound
- used as baseline on small graphs/tests.

## Quantization (Normative)
`score_q = floor(score * 10^12)`

Rules:
- input `score` MUST be finite (reject NaN and +/-inf).
- rounding mode is `floor` (no round-to-even, no casts).
- overflow saturation: clamp to `i64::MAX` / `i64::MIN`.

## Deterministic Commit
Commit output is normalized and hashable:
- sort hits by `(score_q desc, node_id asc)`
- quantize score with fixed scale (`score_q`)
- hash payload with `blake3` over canonical binary encoding:
  - `algo_version`
  - `graph_fingerprint`
  - canonical seeds
  - canonical params
  - `(node_id, score_q)` top-k

`run_id` is derived from canonical `(graph_fingerprint, seeds, params, algo_version)`.

## Fingerprint (Per-Workspace)
`graph_fingerprint = blake3(ws_id || canonical_nodes_bytes || canonical_edges_bytes)`.

Canonical ordering:
- nodes sorted by `node_id` (byte order)
- edges sorted by `(src, dst, rel)` (byte order)

## Params Hash (Normative)
`params_hash = blake3(epsilon || max_push || top_k || algo_version)` using fixed-width little-endian binary encoding.

## Activation Store Model v1
Activation traces are stored in a dedicated activation trace store (SQLite) under:
- `~/.yai/run/<ws_id>/activation.sqlite`

Tables:
- `activation_runs`
  - `run_id` TEXT PRIMARY KEY
  - `ws_id` TEXT NOT NULL
  - `created_at` INTEGER NOT NULL
  - `graph_fingerprint` TEXT NOT NULL
  - `params_hash` TEXT NOT NULL
  - `seeds_hash` TEXT NOT NULL
  - `commit_hash` TEXT NOT NULL
  - `params_json` TEXT NOT NULL
  - `seeds_json` TEXT NOT NULL
  - `stats_json` TEXT NOT NULL
- `activation_results`
  - `run_id` TEXT NOT NULL
  - `node_id` TEXT NOT NULL
  - `rank` INTEGER NOT NULL
  - `score_q` INTEGER NOT NULL
  - PRIMARY KEY (`run_id`, `node_id`)
- `activation_explanations` (optional)
  - `run_id` TEXT NOT NULL
  - `from_node_id` TEXT NOT NULL
  - `to_node_id` TEXT NOT NULL
  - `contrib_q` INTEGER NOT NULL
  - `reason` TEXT NOT NULL

## Retention / Purge
Activation traces are prunable without impacting semantic graph state. Purge operations MUST only affect activation trace tables.

## Activation Traces
`mind/src/cognition/memory/graph/activation/trace.rs`

`ActivationTrace` fields:
- `run_id`
- `created_at_unix`
- `graph_fingerprint`
- `params`
- `seeds`
- `commit_hash`
- `topk`
- `stats`

Store API in `mind/src/cognition/memory/graph/activation/store.rs`:
- `ActivationTraceStore::record_run`
- `ActivationTraceStore::get_run`
- `ActivationTraceStore::list_runs`
- `ActivationTraceStore::purge_keep_last`

## Facade/CLI Wiring
- `GraphFacade` exposes semantic graph adapter helpers for activation.
- CLI:
  - `yai graph activate --seed ... --topk ... --alpha ... --epsilon ...`
  - `yai graph activation show --run <RUN_ID>`
  - `yai graph activation list --limit 20`
  - `yai graph activation purge --keep-last 20`

## Metrics and Bounds
Always returned:
- `pushed`
- `visited`
- `residual_mass`

Safety bounds:
- `max_push`
- `max_nodes`

Bound violations fail fast with explicit error.
