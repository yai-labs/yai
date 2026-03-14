---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-004
decision_id: ADR-004
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#phase-engine-attach-v4"
applies_to: 
effective_date: 2026-02-18
law_refs: 
phase: v4
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md
---
# ADR-004 - Engine as Execution Plane (L2)

## Context

Execution responsibilities must stay isolated from authority checks to keep runtime behavior auditable and composable.

## Decision

Engine is the execution plane for gates and workloads (storage/provider/network/resource/cortex) and operates only after Kernel authorization.

Engine must not:

- Perform authority validation
- Choose workspace ownership
- Open alternative policy channels

## Rationale

A strict L1-to-L2 handoff keeps effect boundaries explicit and simplifies traceability from command to governed execution.

## Consequences

- Positive:
  - Cleaner separation of concerns.
  - Better gate-level observability and testing.
- Negative:
  - Integration requires stable dispatch contracts from Root/Kernel.

## Traceability

- Proposals:
  - `docs/program/rfc/rfc-001-runtime-topology-authority.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`

## Status

Accepted and active.
