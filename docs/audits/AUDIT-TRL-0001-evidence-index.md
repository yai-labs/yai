# AUDIT-TRL-0001 Evidence Index

Date: 2026-02-19

## Existing Evidence (by claim)

- Contract anchor exists and is versioned
  - `yai-specs/SPEC_MAP.md`
  - `yai-specs/REGISTRY.md`
  - `yai-specs/VERSIONING.md`
  - `yai-specs/COMPATIBILITY.md`

- Law/invariants are explicit and structured
  - `yai-specs/contracts/axioms/*`
  - `yai-specs/contracts/invariants/*`
  - `yai-specs/contracts/boundaries/*`

- Protocol and runtime include contracts are defined
  - `yai-specs/specs/protocol/include/protocol.h`
  - `yai-specs/specs/protocol/include/transport.h`
  - `yai-specs/specs/protocol/include/yai_protocol_ids.h`
  - `yai-specs/specs/protocol/include/errors.h`
  - `yai-specs/specs/protocol/include/auth.h`
  - `yai-specs/specs/protocol/include/roles.h`
  - `yai-specs/specs/protocol/include/session.h`
  - `yai-specs/specs/protocol/include/audit.h`
  - `yai-specs/specs/protocol/runtime/include/rpc_runtime.h`

- Formal traceability coverage exists and runs
  - `yai-specs/formal/traceability.v1.json`
  - `yai-specs/tools/formal/validate_traceability.py`
  - `yai-specs/.github/workflows/ci.yml`
  - Local run 2026-02-19: `make formal-coverage` passed

- Core baseline formal+build verification exists and runs
  - `yai/tools/ops/verify/core.sh`
  - `yai/tools/ops/verify/law-kernel.sh`
  - `yai/tools/bin/yai-verify`
  - Local run 2026-02-19: `tools/bin/yai-verify core` passed

- Release pin governance exists
  - `yai/tools/release/check_pins.sh`
  - `yai/.github/workflows/bundle.yml`
  - `yai/deps/yai-cli.ref`

- CLI repo verification exists and runs
  - `yai-cli/tools/bin/yai-cli-verify`
  - `yai-cli/tools/python/yai_cli_tools/verify/*`
  - `yai-cli/.github/workflows/ci.yml`
  - Local run 2026-02-19: `./tools/bin/yai-cli-verify --profile ci` passed

- Operational suite/gates exist
  - `yai/tools/ops/suite/levels/l0-l7.sh`
  - `yai/tools/ops/suite/ops/no-llm-360.sh`
  - `yai/tools/ops/gate/ws.sh`
  - `yai/tools/ops/gate/cortex.sh`
  - `yai/tools/ops/gate/events.sh`
  - `yai/tools/ops/gate/graph.sh`
  - `yai/tools/ops/gate/providers.sh`

- Runtime policy/governance docs exist
  - `yai/FOUNDATION.md`
  - `yai/GOVERNANCE.md`
  - `yai/SECURITY.md`
  - `yai/DATA_POLICY.md`

- ADR/Runbook/MP spine and traceability checks exist
  - `yai/docs/design/spine.md`
  - `yai/docs/design/traceability.md`
  - `yai/.github/workflows/validate-traceability.yml`
  - `yai/.github/workflows/validate-runbook-adr-links.yml`

## Key Runtime Findings (execution evidence)

- `yai` L0-L7 suite reported success but L3-L7 checks were skipped due unsupported CLI targets
  - Run: 2026-02-19 `yai/tools/ops/suite/levels/l0-l7.sh`
  - Relevant scripts: `yai/tools/ops/gate/*.sh`

- `yai-cli` command contract drift detected
  - Contract: `yai-cli/deps/yai-specs/specs/cli/schema/commands.v1.json`
  - Implementation dispatch: `yai-cli/src/cli/dispatch.c`
  - Runtime check 2026-02-19: `./dist/bin/yai-cli up --help` => unknown target

- `yai-mind` test/build instability detected
  - Run: 2026-02-19 `cargo test` in `yai-mind` failed
  - Test files: `yai-mind/tests/integration_test.rs`, `yai-mind/tests/memory_rag.rs`, `yai-mind/tests/providers.rs`

## Missing Evidence (required to raise TRL)

- Non-skip E2E gate artifacts (ws/cortex/events/graph/providers) for current shipped CLI/runtime.
- Deterministic negative reject matrix with stable code mapping from specs -> core -> CLI output.
- Core↔mind integration proof with passing automated tests and traceable run artifacts.
- Reproducibility evidence for dataset-driven flows (seed/replay with stable outcomes).
- Consolidated audit pack linking command runs, CI URLs/artifacts, log excerpts, trace_id correlations.
