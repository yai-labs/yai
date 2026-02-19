# PP-FOUNDATION-0001

Status: active
Date: 2026-02-19
Canonical source: `yai/docs/proof/PP-FOUNDATION-0001/`

## Purpose
Standardize one canonical, machine-readable foundation proof pack for TRL/documentation/demo preparation.

## Pin Set (explicit)
- `yai-specs`: `v0.1.0` line, tag `UNTAGGED`, commit `1c37e5f98e43c6a03dbc7da4000ad9d3e22c7467` (as pinned in `yai/deps/yai-specs`)
- `yai-cli`: version `0.1.7`, tag `UNTAGGED`, commit `11edfa68327fe6126cef04d95fc8e2b4c7805113` (from `yai/deps/yai-cli.ref`)
- `yai-mind`: version `0.1.0`, tag `UNTAGGED`, commit `05aa7254c419a1620452e995c286d0d3b9a456c3` (workspace-observed baseline)

## Evidence Split
### Existing Evidence
- Contract/formal baseline exists and runs (`yai-specs` checks + formal coverage).
- Core verify baseline passes (`yai/tools/bin/yai-verify core`).
- CLI verify CI profile passes (`yai-cli/tools/bin/yai-cli-verify --profile ci`).

### Missing Evidence
- End-to-end non-skip proof for L3-L7 gates.
- Contract/command parity proof (`commands.v1.json` vs runtime-exposed command surface).
- Stable, passing mind integration test evidence.

## Gate Classification
### Non-skip gates (count as proof)
- `specs_check`: pass
- `specs_formal_coverage`: pass
- `core_verify`: pass
- `cli_verify_ci`: pass

### Skip gates (do NOT count as proof)
- `l7_ws_gate`
- `l7_cortex_gate`
- `l7_events_gate`
- `l7_graph_gate`
- `l7_providers_gate`
- `l7_smoke_test`

## Machine-readable Manifest
- `pp-foundation-0001.manifest.v1.json`
