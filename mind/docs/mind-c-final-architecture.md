# Mind C Final Architecture

## 1. Subsystem structure

- `include/`
  - `mind.h`: runtime lifecycle and runtime state contract.
  - `mind_cognition.h`: cognition request/response, role and scoring APIs.
  - `mind_memory.h`: graph/domain APIs, arena contract, memory query/result.
  - `mind_providers.h`: provider vtable, registry and client bridge APIs.
  - `mind_transport.h`: protocol and UDS transport entrypoints.
  - `mind_types.h`, `mind_error.h`: shared primitives and error model.
- `src/`
  - `cognition/`: agents, orchestration, reasoning.
  - `memory/`: graph backend/facade, domain storage, arena.
  - `providers/`: registry, mock provider and client bridge.
  - `transport/`: protocol parser/dispatcher and UDS one-shot server.
  - `mind.c`: subsystem lifecycle composition.
  - `main.c`: daemon entrypoint.
- `tests_c/`: C baseline tests for lifecycle, providers, memory, cognition, transport.

## 2. Layer responsibilities

### cognition
Handles cognitive request execution with a deterministic baseline pipeline:
- role selection,
- planning,
- RAG context/prompt build,
- agent dispatch,
- response assembly.

### memory
Provides in-memory graph backend and domain stores (activation, authority, episodic, semantic, vector), plus retrieval APIs used by cognition.

### providers
Exposes provider abstraction via vtable and registry. The default mock provider supports completion and embedding flows for deterministic tests.

### transport
Defines the text protocol boundary (`PING`, `COMPLETE`, `EMBED`, `QUERY`, `COGNITION`) and UDS runtime integration.

## 3. Primitive contracts

- `yai_mind_runtime_t`
- `yai_mind_session_t`
- `yai_mind_task_t`
- `yai_mind_plan_step_t`
- `yai_mind_cognition_request_t`
- `yai_mind_cognition_response_t`
- `yai_mind_provider_request_t`
- `yai_mind_provider_response_t`
- `yai_mind_memory_query_t`
- `yai_mind_memory_result_t`
- `yai_mind_node_id_t`, `yai_mind_edge_id_t`

## 4. Runtime lifecycle

`yai_mind_init()` initializes subsystems in strict order:
1. transport
2. providers
3. memory
4. cognition

`yai_mind_shutdown()` tears down in reverse order and clears runtime state flags.

## 5. Arena allocator usage

Arena allocation is session-scoped in RAG execution:
- acquire/reset session arena per cognition cycle,
- allocate temporary context/prompt buffers from arena,
- avoid heap churn on transient data,
- keep ownership explicit (arena-owned transient memory only).

## 6. Provider vtable model

Provider interface is intentionally small:
- `completion(...)`
- `embedding(...)`
- `destroy(...)`

Registry guards:
- bounded capacity,
- duplicate-name rejection,
- deterministic default provider selection.

## 7. Memory graph shape

Graph backend stores nodes/edges with stable integer IDs and supports:
- create/get node,
- create/get edge,
- stats,
- query baseline.

Domain stores are reset on memory shutdown/re-init through explicit graph state reset helpers.

## 8. Minimal cognitive flow

Transport request (`COGNITION ...`) path:
1. protocol parse
2. cognition request creation
3. plan generation
4. context retrieval via memory query
5. prompt assembly
6. role-scoped agent execution
7. formatted transport response

## 9. Build and test usage

From `mind/`:
- `make clean`
- `make -j4`
- `make test`
- `make run`

This is the canonical runtime/test path for Mind after Rust decommissioning.
