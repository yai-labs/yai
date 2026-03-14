---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-010
decision_id: ADR-010
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#phase-root-boot-baseline"
applies_to: 
effective_date: 2026-02-18
law_refs: 
phase: boot-baseline
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md
---
# ADR-010 - Boot as Canonical Machine Entry

## Context

Multiple startup paths historically allowed inconsistent runtime initialization and reduced confidence in machine-level governance.

## Decision

`yai` boot is the canonical runtime entrypoint.

Boot responsibilities:

- Preboot validation
- Runtime directory integrity checks
- Root socket initialization
- Ordered startup of governed planes

Direct ad-hoc startup of internal binaries is deprecated.

## Rationale

A single entrypoint improves reproducibility, policy enforcement, and incident diagnosis.

## Consequences

- Positive:
  - Consistent startup contract for operators and CI.
  - Better alignment with Root-first architecture.
- Negative:
  - Legacy scripts and habits need migration.

## Traceability

- Proposals:
  - `docs/program/rfc/rfc-003-workspace-lifecycle-isolation.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/workspaces-lifecycle/mp-workspace-000-workspaces-lifecycle-v0-1-4.md` *(planned)*

## Status

Accepted and active.
