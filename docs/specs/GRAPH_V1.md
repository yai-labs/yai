# Graph v1 (Operational Spec)

## Purpose
Defines the runtime graph layout and contracts for MindGraph v1.

## Canonical Layers

### Semantic (source of truth)
- Nodes + edges with deterministic IDs.
- Persisted in sqlite.
- Node id format: `node:<kind>:<slug>`
- Edge id format: `edge:<rel>:<src>:<dst>`

### Episodic (derived)
- Ingested only from `events.log`.
- No direct CLI writes.

### Vector (derived)
- Embeddings only.
- Rebuildable from semantic.

### Activation (runtime)
- No persistence.
- Consumes semantic + vector.

### Authority (read‑only)
- Loaded from Law specs.
- No runtime mutations.

## Paths
- Semantic: `~/.yai/run/<ws>/semantic.sqlite`
- Episodic: `~/.yai/run/<ws>/events.log`
- Vector: `~/.yai/run/<ws>/vector.usearch`
- Activation: runtime only
- Authority: `<workspace>/yai-core/law/specs/control/authority.json`

## Query Contract
`graph query` must:
1. embed text
2. top‑k via vector
3. activation (1–2 hops)
4. return active subgraph (nodes + edges)

## Constraints
- Deterministic output for same inputs.
- Semantic is the only writable source of truth.
- Activation never writes to disk.

## Authority Boundary (Non‑Negotiable)
- Graph is **read‑only with respect to L1** (Kernel/Vault).
- Graph may only **read** events and derived data.
- Graph must **never** mutate runtime state, vault layout, or kernel authority.
- Any policy/action proposals must be mediated by Kernel enforcement, not Graph writes.
