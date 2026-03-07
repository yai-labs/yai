# Exec Refoundation

## Scope
This wave converges legacy `engine/` implementation into:

- `lib/exec/`
- `include/yai/exec/`

`exec` is the execution plane of the unified runtime and is not an authority plane.

## Exec Domain Definition
`exec` includes:

- execution runtime concerns
- runtime config loading for execution
- execution bridges/adapters
- transport client for execution plane calls
- provider/network/storage/resource gates
- execution-side agent enforcement helpers

`exec` excludes:

- authority and workspace sovereignty (`core`)
- cognition and memory (`brain`)
- rpc codec/wire foundations (`protocol`)
- os/fs/uds/clock adapters (`platform`)
- neutral primitives (`support`)

## Implemented Structure

- `lib/exec/runtime/`
- `lib/exec/gates/`
- `lib/exec/bridge/`
- `lib/exec/agents/`
- `lib/exec/internal.h`

## Effective Module Migration

- `engine/src/config/config_loader.c` -> `lib/exec/runtime/config_loader.c`
- `engine/src/cortex/engine_cortex.c` -> `lib/exec/runtime/runtime_model.c`
- `engine/src/gates/provider_gate.c` -> `lib/exec/gates/provider_gate.c`
- `engine/src/gates/network_gate.c` -> `lib/exec/gates/network_gate.c`
- `engine/src/gates/storage_gate.c` -> `lib/exec/gates/storage_gate.c`
- `engine/src/gates/resource_gate.c` -> `lib/exec/gates/resource_gate.c`
- `engine/src/bridge/transport_client.c` -> `lib/exec/bridge/transport_client.c`
- `engine/src/core/commands/rpc_router.c` -> `lib/exec/bridge/rpc_router.c`
- `engine/src/agent_enforcement.c` -> `lib/exec/agents/agent_enforcement.c`
- `engine/src/bridge/bridge.c` -> `lib/exec/bridge/engine_bridge.c`

## Public Exec Surface

- `include/yai/exec/runtime.h`
- `include/yai/exec/engine_bridge.h`
- `include/yai/exec/transport_client.h`
- `include/yai/exec/provider_gate.h`
- `include/yai/exec/network_gate.h`
- `include/yai/exec/storage_gate.h`
- `include/yai/exec/resource_gate.h`

Legacy headers in `engine/include` are now compatibility wrappers where appropriate.

## Ambiguous Cases (classification)

- `engine_cortex.c`: `rename` to `runtime_model.c`; semantics preserved, name aligned with exec runtime modeling.
- `bridge.c`: `rename` to `engine_bridge.c`; generic name removed.
- `yai_engine.h`: `keep-temporary` compatibility header; reduced to wrapper over `exec` runtime/bridge surfaces.
- `config_enforcer.h`: `keep-internal` (engine compatibility); functionality now exposed as `yai_exec_config_*` in `include/yai/exec/runtime.h`.
- `ingest_monitor.h`: `keep-internal` and deferred; no stable exec public contract in this wave.
- `engine/src/main.c`: `keep-temporary` legacy daemon entrypoint; target absorption into `cmd/yai-core` during runtime entry convergence.

## Build Impact

- Root `Makefile` now builds `build/lib/libyai_exec.a` from `lib/exec/*`.
- `cmd/yai-core/main.c` now consumes exec runtime with `--exec-probe`.
- `engine/Makefile` now compiles moved implementation from `lib/exec/*` while keeping `engine/src/main.c` as temporary entrypoint.

## Intentional Limits

- No full replacement of legacy `yai-engine` entrypoint in this wave.
- No `brain` refoundation in this wave.
- No destructive removal of top-level `engine/` directory in this wave.
