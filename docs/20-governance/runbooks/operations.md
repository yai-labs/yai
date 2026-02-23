---
id: RB-OPERATIONS
title: Operations
status: draft
owner: runtime
effective_date: 2026-02-19
revision: 1
supersedes: []
depends_on:
  - RB-ROOT-HARDENING
  - RB-WORKSPACES-LIFECYCLE
adr_refs:
  - docs/20-governance/design/adr/ADR-003-kernel-authority.md
  - docs/20-governance/design/adr/ADR-006-unified-rpc.md
decisions:
  - docs/20-governance/design/adr/ADR-003-kernel-authority.md
  - docs/20-governance/design/adr/ADR-006-unified-rpc.md
related:
  adr: []
  specs: []
  test_plans:
    - docs/50-qualification/test-plans/hardfail.md
  tools:
    - tools/bin/yai-gate
    - tools/bin/yai-suite
    - tools/bin/yai-verify
tags:
  - runtime
  - ops
---

# RB-OPERATIONS — Operations

## 1) Purpose
Define the operational control loop for running, validating, and recovering the YAI runtime stack in deterministic fashion.

## 2) Preconditions
- [ ] Required binaries are built and available.
- [ ] Workspace/runtime directories are writable.
- [ ] Core verify/gate scripts are executable.

## 3) Inputs
- Runtime commands: `yai up/down/status`
- Gate commands: `tools/bin/yai-gate`, `tools/bin/yai-suite`, `tools/bin/yai-verify`
- Test plans: `docs/50-qualification/test-plans/hardfail.md`

## 4) Procedure
1. Start from clean runtime state.
2. Boot baseline services and verify core health.
3. Execute gate/suite levels required by target phase.
4. Capture logs/evidence and close only on deterministic pass.

## 5) Verification
- Core health checks pass (`up/status/ping`).
- Required verify/gate commands exit `0`.
- Evidence artifacts are attached for audit/review.

## 6) Failure Modes
- Symptom: runtime processes remain in partial/dirty state.
  - Fix: force cleanup, validate sockets/pids, restart baseline.
- Symptom: gate pass without required evidence.
  - Fix: mark run failed and require explicit artifact capture.

## 7) Rollback
- Stop runtime processes and clear active run state for the affected workspace.
- Revert active operational change set to last known good baseline.
- Re-run core checks before resuming.

## 8) References
- Test plans: `docs/50-qualification/test-plans/hardfail.md`
- Ops scripts: `tools/ops/README.md`

## Traceability
- ADR refs (ops-only runbook): none mandatory.
- MPs (filled as phases ship): `docs/20-governance/milestone-packs/...`
