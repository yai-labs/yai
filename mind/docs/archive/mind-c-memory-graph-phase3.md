# Mind C Memory Graph — Phase 3

## Goal
Phase 3 ports the `memory/graph` core semantics from Rust to a native C subsystem with:
- graph facade
- explicit backend boundary
- stable IDs/handles
- domain boundaries (`activation`, `authority`, `episodic`, `semantic`, `vector`)
- minimum parity tests

Rust legacy remains untouched and still available as reference.

## Rust -> C mapping (graph core)

| Rust source | C target | Status |
|---|---|---|
| `src/memory/graph/backend.rs` | `src/memory/graph/graph_backend.c` | implemented (in-memory backend) |
| `src/memory/graph/backend_rpc.rs` | `src/memory/graph/graph_backend_rpc.c` | boundary implemented, logic stubbed (`NOT_IMPLEMENTED`) |
| `src/memory/graph/facade.rs` | `src/memory/graph/graph_facade.c` | implemented |
| `src/memory/graph/ids.rs` | `src/memory/graph/ids.c` + `include/mind_types.h` | implemented |
| `src/memory/graph/mod.rs` | `include/mind_memory.h` + `src/memory/memory.c` | implemented |

## Backend and facade design
- Backend operations are defined in internal interface `graph_backend.h`.
- Active backend selected via:
  - `yai_mind_graph_backend_use_inmemory()`
  - `yai_mind_graph_backend_use_rpc()`
- Public callers only use facade APIs in `mind_memory.h`:
  - node/edge create
  - node/edge get
  - stats
  - query

This keeps storage details out of call sites.

## Graph data model (minimum real implementation)
In-memory backend includes:
- dynamic node array
- dynamic edge array
- monotonic `node_id` / `edge_id` allocation
- id-based get operations
- simple substring query over node fields

Supported operations (minimum parity):
- create node
- create edge
- get node by id
- get edge by id
- query and stats

## Domain boundaries ported in C
Implemented domain boundaries:
- `domain_activation.c`
  - record activation
  - fetch last activation + trace metadata
- `domain_authority.c`
  - grant policy level by node
  - get policy record
- `domain_episodic.c`
  - append episodic record
  - get latest record
- `semantic_db.c` + `domain_semantic.c`
  - put/get semantic records
  - optional node creation when node id is not provided
- `vector_index.c`
  - vector upsert
  - nearest neighbor lookup (linear L2 baseline)

## Vector domain baseline
Vector domain intentionally uses a small linear index:
- bounded vector dimension (`YAI_MIND_VECTOR_MAX_DIM`)
- deterministic insert/update and nearest lookup
- no ANN complexity in this phase

## Tests / parity minimum
Added C tests under `tests_c/`:
- `test_memory_graph.c`
  - memory init/shutdown
  - node/edge create
  - node/edge lookup
  - query
  - graph stats
- `test_memory_domains.c`
  - activation record + trace
  - authority grant/get
  - episodic append/latest
  - semantic put/get
  - vector upsert/nearest

## Build integration
`Makefile` now compiles graph modules and tests:
- `all`
- `clean`
- `check`
- `test`

## Stubbed / deferred items
Still intentionally deferred:
- RPC backend real transport behavior
- domain-level persistence and compaction strategies
- full graph serialization protocol
- full parity against all Rust domain tests

## Residual risks and next steps
- cognition and transport remain loosely coupled from graph (expected in this phase).
- backend RPC contract needs concrete protocol integration in next phase.
- eventual Rust decommission requires broader parity and compatibility checks.
