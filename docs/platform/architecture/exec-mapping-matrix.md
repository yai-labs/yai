# Exec Mapping Matrix

| Legacy Path | Target Path | Action | Semantic Responsibility | Dependency Impact | Follow-up |
|---|---|---|---|---|---|
| `engine/src/config/config_loader.c` | `lib/exec/runtime/config_loader.c` | `move` | Execution runtime config load | Public API now via `include/yai/exec/runtime.h` | Merge with core policy model later |
| `engine/src/cortex/engine_cortex.c` | `lib/exec/runtime/runtime_model.c` | `move+rename` | Runtime scaling/cortex model | Removed legacy naming from execution model | Decide if further split needed |
| `engine/src/gates/provider_gate.c` | `lib/exec/gates/provider_gate.c` | `move` | Provider gate dispatch | Uses public `exec` gate header | Provider auth hardening later |
| `engine/src/gates/network_gate.c` | `lib/exec/gates/network_gate.c` | `move` | Network gate boundary | Public readiness stub added | Implement concrete checks later |
| `engine/src/gates/storage_gate.c` | `lib/exec/gates/storage_gate.c` | `move` | Storage gate RPC path | Bound to `exec` bridge/vault | Separate sqlite adapter later |
| `engine/src/gates/resource_gate.c` | `lib/exec/gates/resource_gate.c` | `move` | Resource gate boundary | Public readiness stub added | Implement concrete checks later |
| `engine/src/bridge/transport_client.c` | `lib/exec/bridge/transport_client.c` | `move` | Transport client adapter | Public `exec` transport contract | Align socket path authority later |
| `engine/src/core/commands/rpc_router.c` | `lib/exec/bridge/rpc_router.c` | `move+rename` | Exec-side RPC routing | Kept internal header dependency for router contract | Promote/replace router contract later |
| `engine/src/bridge/bridge.c` | `lib/exec/bridge/engine_bridge.c` | `move+rename` | SHM bridge adapter | Generic bridge naming removed | Integrate with unified runtime context |
| `engine/src/agent_enforcement.c` | `lib/exec/agents/agent_enforcement.c` | `move` | Exec-side agent action checks | Uses exec bridge API | Reconcile with core enforcement rules |
| `engine/src/main.c` | `cmd/yai-core/main.c` | `keep-temporary` | Legacy engine daemon entry | No longer canonical runtime entry | Absorb into unified runtime flow |
| `engine/include/engine_bridge.h` | `include/yai/exec/engine_bridge.h` | `merge` | Public exec bridge API | Legacy header now wrapper | Remove wrapper in legacy removal wave |
| `engine/include/transport_client.h` | `include/yai/exec/transport_client.h` | `merge` | Public exec transport client API | Legacy header now wrapper | Remove wrapper in legacy removal wave |
| `engine/include/provider_gate.h` | `include/yai/exec/provider_gate.h` | `merge` | Public provider gate API | Legacy header now wrapper | Remove wrapper in legacy removal wave |
| `engine/include/network_gate.h` | `include/yai/exec/network_gate.h` | `merge` | Public network gate API | Legacy header now wrapper | Expand beyond readiness probe |
| `engine/include/storage_gate.h` | `include/yai/exec/storage_gate.h` | `merge` | Public storage gate API | Legacy header now wrapper | Storage backend abstraction follow-up |
| `engine/include/resource_gate.h` | `include/yai/exec/resource_gate.h` | `merge` | Public resource gate API | Legacy header now wrapper | Expand beyond readiness probe |
| `engine/include/yai_engine.h` | `engine/include/yai_engine.h` | `keep-temporary` | Legacy umbrella header | Reduced to compatibility role | Remove with engine package decommission |
| `engine/include/config_enforcer.h` | `lib/exec/runtime/*` | `keep-internal` | Legacy config limit model | API replaced with `yai_exec_config_*` | Merge with core enforcement policy later |
| `engine/include/ingest_monitor.h` | `engine/include/ingest_monitor.h` | `keep-internal` | Legacy ingest telemetry | No public exec dependency | Reclassify in later hardening wave |
| `engine/include/shared_constants.h` | `include/yai/exec/runtime.h` | `split` | Runtime constants | `YAI_RPC_BUFFER_MAX` now alias to `YAI_EXEC_RPC_BUFFER_MAX` | Remove alias in legacy removal wave |
