---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-007
decision_id: ADR-007
supersedes: []
superseded_by: []
implements: [docs/program/rfc/rfc-003-workspace-lifecycle-isolation.md]
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#phase-0-1-0-workspace-layout"
applies_to: 
effective_date: 2026-02-18
law_refs: 
phase: 0.1.0
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md
---
# ADR-007 - Workspace Isolation Model

## Context

Workspace lifecycle and runtime ownership needed a clear isolation model to prevent session bleed and mixed authority paths.

## Decision

Isolation is enforced on three layers:

1. Session/lock ownership
2. Per-workspace storage/memory boundaries
3. Root-mediated routing for runtime commands

Stale lock recovery remains allowed, but only through deterministic validation.

## Rationale

The model keeps tenancy explicit and reduces accidental cross-workspace effects under concurrent runtime load.

## Consequences

- Positive:
  - Safer multi-workspace operation.
  - Better reproducibility under parallel sessions.
- Negative:
  - Operational tooling must preserve strict lock semantics.

## Traceability

- Proposals:
  - `docs/program/rfc/rfc-003-workspace-lifecycle-isolation.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/workspaces-lifecycle/mp-workspace-000-workspaces-lifecycle-v0-1-4.md` *(planned)*
  - `docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md` *(planned)*

## Status

Accepted and active.
