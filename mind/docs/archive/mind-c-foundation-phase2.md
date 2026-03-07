# Mind C Foundation — Phase 2

## Scope
Phase 2 builds the shared C foundation required by subsequent migrations. This phase intentionally does not translate all Rust cognition/memory domains.

Implemented in this phase:
- shared error model
- shared primitive types
- subsystem lifecycle with idempotent init/shutdown
- arena allocator
- graph/memory ID validity helpers
- provider abstraction via vtable
- minimal provider registry
- mock provider bootstrap
- daemon entrypoint wired to lifecycle
- Makefile `check` target

Not implemented yet:
- full `cognition/agents` translation
- full `cognition/orchestration` translation
- full `memory/graph/domains` translation
- full UDS transport server
- destructive Rust cleanup

## Lifecycle model
Public lifecycle API:
- `int yai_mind_init(const yai_mind_config_t *config);`
- `int yai_mind_shutdown(void);`
- `int yai_mind_is_initialized(void);`

Behavior:
- idempotent init and shutdown
- ordered subsystem startup: transport -> providers -> memory -> cognition
- reverse shutdown order with best-effort cleanup
- init rollback on failure

## Error model
`include/mind_error.h` defines a compact shared model:
- `YAI_MIND_OK = 0`
- negative errors for failure classes (invalid arg, no memory, not found, not implemented, provider, transport, state)

This avoids mixed bool/null/int semantics and standardizes return behavior across modules.

## Shared primitive contracts
`include/mind_types.h` now contains baseline structs for:
- `yai_mind_session_t`
- `yai_mind_task_t`
- `yai_mind_plan_step_t`
- `yai_mind_provider_request_t`
- `yai_mind_provider_response_t`
- `yai_mind_memory_query_t`
- `yai_mind_memory_result_t`

Also includes stable handle primitives:
- `yai_mind_node_id_t`
- `yai_mind_edge_id_t`
- invalid constants `YAI_MIND_NODE_ID_INVALID`, `YAI_MIND_EDGE_ID_INVALID`

## Arena allocator
`include/mind_memory.h` + `src/memory/arena.c` provide:
- `yai_mind_arena_init`
- `yai_mind_arena_alloc`
- `yai_mind_arena_reset`
- `yai_mind_arena_destroy`

Constraints implemented:
- power-of-two alignment validation
- bounded offset growth
- explicit ownership/lifetime via init/destroy

## Provider abstraction and registry
`include/mind_providers.h` defines:
- `yai_mind_provider_t`
- `yai_mind_provider_vtable_t` (`completion`, `embedding`, `destroy`)
- `yai_mind_provider_registry_t`

Implemented modules:
- `src/providers/provider_registry.c`
- `src/providers/mock_provider.c`
- `src/providers/providers.c`

Behavior:
- registry init/register/get/default/shutdown
- provider destruction delegated to vtable destroy callback
- subsystem bootstrap registers mock provider as default

## Daemon entrypoint
`src/main.c` is now a real entrypoint:
- builds a minimal config
- calls `yai_mind_init`
- supports `--check` for bootstrap validation
- calls `yai_mind_shutdown`
- returns coherent exit status

## Build status
`mind/Makefile` compiles all Phase 2 C modules and adds:
- `all`
- `clean`
- `check`
- `test` (alias to check)

## Intentional stubs left for Phase 3+
- graph backend placeholder in `src/memory/graph/graph.c`
- storage bridge placeholder in `src/memory/storage_bridge.c`
- cognition and transport internals remain minimal

These are intentional to keep the foundation stable before domain-level translation.

## Next phase focus
Phase 3 should target memory graph real modules:
- backend/backend_rpc/facade translation
- domain modules (`activation`, `authority`, `episodic`, `semantic`, `vector`)
- domain parity tests against Rust behavior
