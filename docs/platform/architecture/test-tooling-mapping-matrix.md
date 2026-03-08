# Test and Tooling Mapping Matrix

| Legacy path/tool | Target path/behavior | Action | Responsibility | Rationale | Follow-up |
|---|---|---|---|---|---|
| `tests/domains/boot` | `tests/unit/core` + `tests/integration/workspace_lifecycle` | deprecate | core lifecycle verification | old package taxonomy no longer authoritative | remove placeholder in final decommission |
| `tests/domains/kernel` | `tests/unit/core` + `tests/integration/runtime_handshake` | deprecate | core/session/protocol verification | align tests with core/protocol split | remove placeholder in final decommission |
| `tests/domains/root` | `tests/unit/core` + `tests/integration/runtime_handshake` | deprecate | core dispatch/control verification | root no longer package identity | remove placeholder in final decommission |
| `tests/domains/engine` | `tests/unit/exec` + `tests/integration/core_exec` | deprecate | exec verification | engine no longer package identity | remove placeholder in final decommission |
| `mind/tests_c/test_mind_daemon_smoke.c` | `tests/integration/core_brain/test_mind_daemon_smoke.c` | move | core+brain integration | runtime composition-oriented classification | fold into unified integration runner |
| `mind/tests_c/test_runtime_primary.c` | `tests/integration/core_brain/test_runtime_primary.c` | move | runtime primary composition | validates `yai-core` composition semantics | keep until legacy mains removed |
| `engine/tests/cortex_harness.c` | `tests/unit/exec/cortex_harness.c` | move | exec unit | pure exec runtime model unit check | none |
| `engine/tests/protocol_test.c` | `tests/unit/protocol/protocol_test.c` | move | protocol unit | protocol contract shape check | expand malformed envelope assertions |
| `tests/integration/test_handshake.py` | `tests/integration/runtime_handshake/test_handshake.py` | move | runtime handshake integration | explicit integration domain | migrate to `yai-core` socket discovery helper |
| `tests/integration/workspace_runtime_contract_v1.sh` | `tests/integration/workspace_lifecycle/workspace_runtime_contract_v1.sh` | move | workspace lifecycle integration | explicit integration domain | remove cli dependency once parity complete |
| `tools/dev/resolve-yai-bin.sh` | prefer `build/bin/{yai,yai-core}` | merge/update | tool runtime resolution | new binary topology authoritative | drop legacy artifact fallback later |
| `tools/bin/yai-verify` | infra wrapper with `YAI_CORE_ROOT` | keep | verify orchestration | already topology-agnostic | none |
| `tools/bin/yai-docs-trace-check` | infra wrapper with `YAI_CORE_ROOT` | keep | docs trace verification | already topology-agnostic | none |
| `tools/bin/yai-check-pins` | infra wrapper with `YAI_CORE_ROOT` | keep | release pin verification | already topology-agnostic | none |
