# Test, Verify, and Tooling Realignment

## Objective
Converge verification and operational tooling to the refounded repository topology.

## New Test Grammar
Authoritative test topology:
- `tests/unit/core`
- `tests/unit/exec`
- `tests/unit/brain`
- `tests/unit/protocol`
- `tests/unit/support`
- `tests/integration/runtime_handshake`
- `tests/integration/workspace_lifecycle`
- `tests/integration/core_exec`
- `tests/integration/core_brain`
- `tests/e2e`
- `tests/fixtures`
- `tests/shared`

`tests/domains/*` is deprecated and retained only as temporary compatibility marker.

## Concrete Reclassification Applied
- `engine/tests/cortex_harness.c` -> `tests/unit/exec/cortex_harness.c`
- `engine/tests/protocol_test.c` -> `tests/unit/protocol/protocol_test.c`
- `tests/integration/test_handshake.py` -> `tests/integration/runtime_handshake/test_handshake.py`
- `tests/integration/workspace_runtime_contract_v1.sh` -> `tests/integration/workspace_lifecycle/workspace_runtime_contract_v1.sh`
- `mind/tests_c/test_mind_daemon_smoke.c` -> `tests/integration/core_brain/test_mind_daemon_smoke.c`
- `mind/tests_c/test_runtime_primary.c` -> `tests/integration/core_brain/test_runtime_primary.c`
- remaining `mind/tests_c/*` -> `tests/unit/brain/mind_legacy_tests/*`

## Test Execution and Build Outputs
Root Makefile test entrypoints now expose:
- `make test-unit`
- `make test-integration`
- `make test-e2e`
- `make test`

Test artifacts converge to `build/test/*`:
- `build/test/unit_exec`
- `build/test/unit_protocol`
- `build/test/brain`

## Tooling Realignment
Updated tooling behavior:
- `tools/dev/resolve-yai-bin.sh` now resolves `build/bin/yai` and `build/bin/yai-core` as primary.
- legacy artifact fallbacks are explicit and marked compatibility-only.

Verified tooling wrappers remain aligned to `yai-infra` canonical tools:
- `tools/bin/yai-verify`
- `tools/bin/yai-suite`
- `tools/bin/yai-docs-trace-check`
- `tools/bin/yai-check-pins`

## Compatibility Temporary Items
- `tests/domains/*` directories still exist as deprecated placeholders.
- workspace lifecycle contract script still supports external `yai-cli` binary for command-surface compatibility.
- `mind_legacy_tests` suffix retained for current brain unit subset until final naming cleanup.

## Deferred
- remove `tests/domains/*` placeholders in final legacy removal wave
- complete renaming from `mind_legacy_tests` to pure `tests/unit/brain/*`
- migrate remaining shell/python integration scripts to direct `yai`/`yai-core` surfaces where command parity is complete
