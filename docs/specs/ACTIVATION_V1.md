# ACTIVATION_V1

Canonical sources:

- `law/specs/graph/GRAPH_V1.md`
- `law/specs/graph/graph.v1.json`

## Objective
Activation computes memory emergence as graph diffusion (PPR/RWR), with deterministic commit and persisted activation traces.

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

## Deterministic Commit
Commit output is normalized and hashable:
- sort hits by `(score desc, node_id asc)`
- quantize score with fixed scale (`score_q`)
- hash payload with `blake3`:
  - graph fingerprint
  - canonical seeds
  - canonical params subset
  - `(node_id, score_q)` top-k

`run_id` is derived from canonical `(graph_fingerprint, seeds, params)`.

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
- `save_trace(trace)`
- `load_trace(run_id)`
- `list_traces(limit)`

Persistence backend: append-only JSONL (`activation_traces.jsonl`) under run directory.

## Facade/CLI Wiring
- `GraphFacade` exposes semantic graph adapter helpers for activation.
- CLI:
  - `yai graph activate --seed ... --topk ... --alpha ... --epsilon ...`
  - `yai graph trace-show <RUN_ID>`

## Metrics and Bounds
Always returned:
- `pushed`
- `visited`
- `residual_mass`

Safety bounds:
- `max_push`
- `max_nodes`

Bound violations fail fast with explicit error.
