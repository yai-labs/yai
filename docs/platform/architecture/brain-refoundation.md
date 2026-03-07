# Brain Refoundation

## Scope
This wave converges legacy `mind/` implementation paths into `lib/brain/` and makes brain a native runtime module consumed by `yai-core`.

## Brain Domain Definition
`brain` is the cognitive plane of the unified runtime:
- cognition and orchestration
- memory and retrieval
- cognitive provider bridge
- brain transport and lifecycle

`brain` is not:
- authority plane (`core`)
- execution/resource plane (`exec`)
- wire/runtime protocol foundation (`protocol`)
- OS abstraction (`platform`)
- shared primitives (`support`)

## Implemented Structure
- `lib/brain/cognition/` (agents/orchestration/reasoning + cognition facade)
- `lib/brain/memory/` (graph backends/domains, storage bridge, arena store)
- `lib/brain/bridge/` (provider registry/client bridge)
- `lib/brain/transport/` (brain transport + protocol + uds server)
- `lib/brain/lifecycle/` (brain lifecycle facade)
- `lib/brain/internal.h` + per-subdomain internal headers

## Public Header Convergence
Public brain surfaces now live in `include/yai/brain/`:
- `brain.h`
- `cognition.h`
- `memory.h`
- `providers.h`
- `transport.h`
- plus explicit split contracts:
  - `types.h`
  - `errors.h`

Legacy `mind/include/mind_*.h` are now transition wrappers to the new `include/yai/brain/*` contracts.

## Strong Decisions for Ambiguous Cases
- `mind/src/memory/arena.c` -> `lib/brain/memory/arena_store.c`: **keep brain-specific** for now.
- `mind/include/mind_types.h`: **split outcome**; public stable types moved to `include/yai/brain/types.h`, legacy path kept as wrapper.
- `mind/include/mind_error.h`: **split outcome**; brain error surface moved to `include/yai/brain/errors.h`, legacy path kept as wrapper.
- `mind/src/transport/protocol.c` -> `lib/brain/transport/brain_protocol.c`: **keep brain-specific protocol** (no merge into `lib/protocol` in this wave).
- `mind/src/main.c`: **keep-temporary** as compatibility daemon entry for `make -C mind`.
- `mind/src/mind.c` -> `lib/brain/lifecycle/brain_lifecycle.c`: **moved** as lifecycle facade.
- `mind/src/providers/providers.c` -> `lib/brain/bridge/providers.c`: **kept** as useful bridge facade (not removed this wave).

## Pruning Outcome
- Runtime mock modules (`mock_provider.c`, `embedder_mock.c`): **keep-temporary** in runtime path because baseline runtime currently depends on deterministic mock provider bootstrap.
- Agents `agent_historian.c`, `agent_system.c`, `agents_dispatch.c`: **keep-temporary** due active role routing and tests.
- RAG micro-modules (`rag_context_builder.c`, `rag_prompts.c`, `rag_sessions.c`): **keep** (stable boundaries used by pipeline).
- Graph micro-segmentation (`domain_*`, `graph_backend*`, `graph_facade.c`, `graph.c`, `ids.c`): **keep-internal** under `lib/brain/memory/graph/`; flattening deferred.

## Build and Runtime Integration
- Root Makefile now builds `libyai_brain.a` from `lib/brain/*` and links it into `build/bin/yai-core`.
- `cmd/yai-core/main.c` now exposes `--brain-check` and runs a minimal cognition probe via brain lifecycle.
- `mind/Makefile` now compiles from `lib/brain/*` and test files moved to `tests/unit/brain/mind_legacy_tests/`.

## Deferred Items
- Full pruning of mock provider path to test-only fixtures.
- Flatten graph micro-segmentation under `lib/brain/memory/`.
- Remove temporary `mind/src/main.c` compatibility entry.
- Converge brain tests fully into repo-wide `tests/unit/brain` naming conventions (drop `mind_legacy_tests` suffix).
