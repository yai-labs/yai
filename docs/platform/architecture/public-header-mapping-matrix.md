# Public Header Mapping Matrix (Initial)

Action vocabulary:
- `keep`
- `move`
- `rename`
- `merge`
- `split`
- `remove`
- `keep-internal`

This matrix is intentionally initial and domain-level; file-level refinement will continue in subsequent waves.

| Old header path | Target header path | Action | Rationale | Notes |
|---|---|---|---|---|
| `boot/include/bootstrap.h` | `include/yai/core/lifecycle.h` | `merge` | bootstrap/preboot become lifecycle surface | thin lifecycle contract in this wave |
| `boot/include/preboot.h` | `include/yai/core/lifecycle.h` | `merge` | same lifecycle ownership | preboot-specific details stay implementation-side |
| `root/include/control_transport.h` | `include/yai/core/dispatch.h` | `merge` | control plane dispatch belongs to core | keeps rpc envelope dependency explicit |
| `root/include/root_command_dispatch.h` | `include/yai/core/dispatch.h` | `merge` | root command dispatch is core dispatch concern | legacy symbol kept temporarily |
| `root/include/ws_id.h` | `include/yai/core/workspace.h` | `move` | workspace identity validation is core workspace concern | support split may follow later |
| `kernel/include/yai_session.h` | `include/yai/core/session.h` | `rename` | session surface re-homed under core | transitional wrapper in this wave |
| `kernel/include/yai_session_internal.h` | module-local internal header | `keep-internal` | parsing/runtime helpers are not public contracts | do not export via `include/yai` |
| `kernel/include/yai_vault.h` | `include/yai/core/vault.h` | `rename` | vault/sovereignty model belongs to core | transitional wrapper in this wave |
| `kernel/include/yai_events.h` | `include/yai/core/events.h` | `rename` | runtime event taxonomy belongs to core surface | copied stable taxonomy |
| `kernel/include/transport.h` | `include/yai/protocol/transport_contract.h` | `split` | transport contract is protocol concern; runtime ops remain internal | protocol wrapper added |
| `kernel/include/kernel.h` | `include/yai/core/{authority,enforcement,events}.h` | `split` | mixed gate + error + event concerns must be separated | gradual extraction planned |
| `engine/include/engine_bridge.h` | `include/yai/exec/engine_bridge.h` | `move` | execution bridge belongs to exec domain | transitional wrapper |
| `engine/include/network_gate.h` | `include/yai/exec/network_gate.h` | `move` | network gate belongs to exec surface | placeholder contract pending implementation |
| `engine/include/provider_gate.h` | `include/yai/exec/provider_gate.h` | `move` | provider gate is execution-plane boundary | transitional wrapper |
| `engine/include/storage_gate.h` | `include/yai/exec/storage_gate.h` | `move` | storage gate belongs to exec | transitional wrapper |
| `engine/include/resource_gate.h` | `include/yai/exec/resource_gate.h` | `move` | resource gate belongs to exec | placeholder contract pending implementation |
| `engine/include/transport_client.h` | `include/yai/exec/transport_client.h` | `move` | execution transport client is exec concern | transitional wrapper |
| `engine/include/config_enforcer.h` | `include/yai/core/enforcement.h` | `merge` | enforcement semantics belong to core sovereignty plane | narrowed public shape |
| `engine/include/shared_constants.h` | `include/yai/api/config.h` | `split` | truly global constants go to api config; others remain domain-local | avoid dumping all constants in api |
| `engine/include/ingest_monitor.h` | internal or tests | `keep-internal` | monitoring detail is not yet a stable public surface | classify later in exec wave |
| `engine/include/agent_contract.h` | internal or future `include/yai/exec/*` | `keep-internal` | contract maturity unclear in this wave | explicit follow-up required |
| `engine/include/rpc_router.h` | `include/yai/exec/*` or `include/yai/protocol/*` | `split` | mixes routing semantics and protocol types | resolve ownership in exec/protocol waves |
| `mind/include/mind.h` | `include/yai/brain/brain.h` | `rename` | mind runtime contract becomes brain contract | transitional wrapper |
| `mind/include/mind_cognition.h` | `include/yai/brain/cognition.h` | `rename` | cognition belongs to brain | transitional wrapper |
| `mind/include/mind_memory.h` | `include/yai/brain/memory.h` | `rename` | memory belongs to brain | transitional wrapper |
| `mind/include/mind_providers.h` | `include/yai/brain/providers.h` | `rename` | cognitive provider contracts belong to brain | transitional wrapper |
| `mind/include/mind_transport.h` | `include/yai/brain/transport.h` | `rename` | brain transport belongs to brain domain | transitional wrapper |
| `mind/include/mind_error.h` | `include/yai/support/errors.h` | `merge` | cross-cutting error taxonomy belongs to support | transitional include bridge used |
| `mind/include/mind_types.h` | `include/yai/brain/*` + `include/yai/support/*` | `split` | mixed public types require domain ownership separation | gradual extraction |
| `runtime-protocol/include/*` | `include/yai/protocol/*` | `move`/`merge` | protocol layer is foundation owner | currently bridged to yai-law protocol headers |
