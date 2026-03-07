# Mind C Refactor Architecture (Phase 1 Audit + Bootstrap)

## Scope and status
This document defines Phase 1 of the `mind/` Rust-to-C refoundation. Phase 1 is intentionally non-destructive:
- Rust legacy remains present and buildable (`Cargo.toml`, `build.rs`, `src/**/*.rs`, `tests/**/*.rs`).
- A minimal C subsystem skeleton is introduced for incremental migration.
- No deep behavioral translation is completed in this phase.

## Observed current state
Current `mind/` is Rust-centric and organized around these macro-areas:
- `cognition/` (agents, orchestration, reasoning)
- `memory/graph/` (graph backend, domain modules)
- `providers/` (registry, client, embedders)
- `transport/` (protocol + uds_server)
- `types/` (graph and memory types)
- `workspace/` (workspace layout)
- `tests/` (integration and provider/memory tests)

## Macro-area migration plan

### Cognition
Rust source:
- `src/cognition/agents/*.rs`
- `src/cognition/orchestration/**`
- `src/cognition/reasoning/**`

C target:
- `src/cognition/agents/*.c` (phase 2+)
- `src/cognition/orchestration/*.c` (phase 2+)
- `src/cognition/reasoning/*.c` (phase 2+)
- bootstrap entrypoint now: `src/cognition/cognition.c`

### Memory
Rust source:
- `src/memory/graph/**`
- `src/types/graph.rs`
- `src/types/memory.rs`

C target:
- `src/memory/graph/*.c`
- `src/memory/storage_bridge.c`
- public contracts in `include/mind_memory.h`

### Providers
Rust source:
- `src/providers/client.rs`
- `src/providers/registry.rs`
- `src/providers/types.rs`
- `src/providers/embedders/**`

C target:
- `src/providers/provider_registry.c` (phase 2+)
- `src/providers/client_bridge.c` (phase 2+)
- bootstrap now: `src/providers/providers.c`
- public provider contract in `include/mind_providers.h`

### Transport
Rust source:
- `src/transport/protocol.rs`
- `src/transport/uds_server.rs`

C target:
- `src/transport/uds_server.c` (phase 2+)
- `src/transport/protocol.c` (phase 2+)
- bootstrap now: `src/transport/transport.c`
- public transport boundary in `include/mind_transport.h`

### Legacy modules requiring redesign
- `src/error.rs` -> unify with C status policy in `mind.h`.
- `src/lib.rs` -> split into C bootstrap + subsystem init composition.
- `src/workspace/**` -> likely merge into runtime workspace model and mind-session boundaries.
- `src/memory/graph/domains/**` -> convert domain by domain with C ABI contracts.
- `tests/*.rs` -> retain during migration; add C-side tests in later phases.

## File-by-file mapping baseline (Rust -> C)

| Rust module | C target (planned) | Migration shape |
|---|---|---|
| `src/main.rs` | `src/main.c` | translated as bootstrap now |
| `src/lib.rs` | `include/mind.h` + `src/main.c` | redesign |
| `src/error.rs` | `include/mind.h` status enum | redesign |
| `src/cognition/mod.rs` | `src/cognition/cognition.c` | bootstrap now |
| `src/cognition/agents/*.rs` | `src/cognition/agents/*.c` | near 1:1 by agent responsibility |
| `src/cognition/orchestration/**` | `src/cognition/orchestration/*.c` | redesign for explicit task/session contracts |
| `src/cognition/reasoning/*.rs` | `src/cognition/reasoning/*.c` | near 1:1 for role/scoring rules |
| `src/memory/mod.rs` | `src/memory/memory.c` | bootstrap now |
| `src/memory/graph/backend.rs` | `src/memory/graph/backend.c` | near 1:1 with C interface adaptation |
| `src/memory/graph/backend_rpc.rs` | `src/memory/graph/backend_rpc.c` | redesign for transport/client reuse |
| `src/memory/graph/facade.rs` | `src/memory/graph/facade.c` | near 1:1 |
| `src/memory/graph/ids.rs` | `include/mind_memory.h` + `src/memory/graph/ids.c` | split public/private |
| `src/memory/graph/domains/*` | `src/memory/graph/domains/*/*.c` | phased by domain |
| `src/types/graph.rs` | `include/mind_memory.h` | public primitive contracts |
| `src/types/memory.rs` | `include/mind_memory.h` | public primitive contracts |
| `src/providers/mod.rs` | `src/providers/providers.c` | bootstrap now |
| `src/providers/types.rs` | `include/mind_providers.h` | public contracts |
| `src/providers/registry.rs` | `src/providers/provider_registry.c` | phase 2+ |
| `src/providers/client.rs` | `src/providers/client_bridge.c` | phase 2+ |
| `src/providers/embedders/*` | `src/providers/embedders/*` | phase 3+ |
| `src/transport/mod.rs` | `src/transport/transport.c` | bootstrap now |
| `src/transport/protocol.rs` | `src/transport/protocol.c` | phase 2+ |
| `src/transport/uds_server.rs` | `src/transport/uds_server.c` | phase 2+ |
| `src/workspace/layout.rs` | `src/workspace/workspace_layout.c` or runtime integration | redesign |
| `tests/*.rs` | `tests/*.c` (future) | keep Rust tests during transition |

## Primitive contracts proposed for next phases
The following contracts are fixed as architectural primitives:
- `mind_session` -> `yai_mind_session_t` (transport/session boundary)
- `mind_task` -> `yai_mind_task_t` (cognition input unit)
- `mind_plan_step` -> `yai_mind_plan_step_t` (orchestration output unit)
- `mind_provider_request` -> `yai_mind_provider_request_t`
- `mind_provider_response` -> `yai_mind_provider_response_t`
- `mind_node_id` -> `yai_mind_node_id_t`
- `mind_edge_id` -> `yai_mind_edge_id_t`
- `mind_memory_query` -> `yai_mind_memory_query_t`
- `mind_memory_result` -> `yai_mind_memory_result_t`

## Dependencies and conceptual cleanup targets
Dependencies to remove over migration:
- Rust module coupling via `mod.rs` implicit visibility.
- mixed type ownership spread across `types/` and runtime modules.
- transport/provider contracts encoded in Rust-only types.
- workspace semantics hidden in Rust-specific layout modules.

## Sequencing proposal for full migration
1. **Phase 1 (this)**: audit + C bootstrap + public C contracts.
2. **Phase 2**: transport and provider registry/client translation.
3. **Phase 3**: memory graph core (`backend`, `facade`, ids`) + first domain (`semantic`).
4. **Phase 4**: cognition orchestration + reasoning base.
5. **Phase 5**: remaining graph domains and embedders.
6. **Phase 6**: parity testing, contract hardening, Rust decommissioning.

## Rust decommissioning plan (not executed in phase 1)
- Freeze new feature work in Rust modules once C equivalents are ready.
- Add compatibility matrix: Rust module -> C module parity status.
- Remove Rust entrypoints only after C test and runtime acceptance gates are green.
- Final cleanup in a dedicated destructive phase (Cargo/build.rs/src/tests removal), never mixed with bootstrap.

## Phase 1 outputs summary
Created C bootstrap artifacts:
- public headers: `include/mind*.h`
- C source skeleton: `src/main.c`, `src/cognition/cognition.c`, `src/memory/*`, `src/providers/providers.c`, `src/transport/transport.c`
- initial `Makefile` (`all`, `clean`) producing `dist/bin/yai-mind`

Legacy Rust remains present and unchanged by design.
