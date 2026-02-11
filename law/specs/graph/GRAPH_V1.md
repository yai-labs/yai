# Graph v1 (Law Spec)

## Scope
Defines canonical graph contracts for MindGraph v1 (layered, deterministic).

## Data Model (Canonical Layers)

### Semantic Layer (source of truth)
- **Node**
  - `id`: string (stable identifier, format: `node:<kind>:<slug>`)
  - `kind`: string (semantic type)
  - `meta`: JSON object (metadata, freeform)
  - `last_seen`: unix epoch seconds
- **Edge**
  - `id`: string (stable identifier, format: `edge:<rel>:<src>:<dst>`)
  - `src`: string (node id)
  - `dst`: string (node id)
  - `rel`: string (relationship)
  - `weight`: float

### Episodic Layer (derived)
- Ingests from `events.log` only.
- No direct CLI writes to episodic store.

### Vector Layer (derived)
- Rebuildable from semantic nodes.
- Stores embeddings only; no semantic metadata.

### Activation Layer (runtime only)
- No persistence.
- Uses semantic + vector as input.

### Authority Layer (read‑only)
- Derived from Law specs + trusted policy inputs.
- No runtime mutations.

## Persistence (Paths)
- **Semantic**: `~/.yai/run/<ws>/semantic.sqlite`
- **Episodic**: `~/.yai/run/<ws>/events.log` (append‑only NDJSON)
- **Vector**: `~/.yai/run/<ws>/vector.usearch` (rebuildable)
- **Activation**: no persistence
- **Authority**: `<workspace>/yai-core/law/specs/control/authority.json`

## Indexing
- Embeddings are stored in vector layer only.
- Vector index must be rebuildable from semantic nodes.

## Queries
`query_active_subgraph(query_text, k, hops=1)` returns:
- top‑k nodes by similarity (vector)
- plus hop expansion (activation)

## Constraints
- No LLM required
- Local only
- Deterministic output for same inputs
 - Semantic store is the single source of truth

## Authority Boundary (Non‑Negotiable)
- Graph is **read‑only with respect to L1** (Kernel/Vault).
- Graph may only **read** events and derived data.
- Graph must **never** mutate runtime state, vault layout, or kernel authority.
- Any policy/action proposals must be mediated by Kernel enforcement, not Graph writes.
