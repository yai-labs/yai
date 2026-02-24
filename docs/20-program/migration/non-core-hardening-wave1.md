# Non-Core Hardening Wave 1 (Historical Note)

## Goal
Reduce non-core surface in `yai` while keeping CI/runtime behavior stable.

## Wave 1 Delivered
- Removed local Python mirror under `tools/python/yai_tools/`.
- Enforced hard delegation from `tools/bin/*` to canonical `yai-infra/tools/bin/*`.

## Safety Rationale
- Governance tooling is non-runtime surface and now externalized.
- Runtime domains (`boot/`, `root/`, `kernel/`, `engine/`, `mind/`, `runtime/`) were not changed by this wave.

## Current Alignment Status
- Canonical governance/tooling ownership is `yai-infra`.
- `yai` keeps wrapper entrypoints for operator compatibility.
- Program migration notes are retained only as historical traceability records.

## Follow-up Cards (Automation and CI/CD)
- Enable **Auto-assign new issues** in repository automation settings.
- Enable and validate **Run a continuous integration test** in automation settings/workflows.
