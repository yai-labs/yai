---
id: ADR-004
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md
  phase: v4
  anchor: "#phase-engine-attach-v4"
law_refs:
  - ../governance/foundation/invariants/I-006-external-effect-boundary.md
  - ../governance/foundation/boundaries/L2-engine.md
  - ../governance/contracts/protocol/include/protocol.h
  - ../governance/contracts/protocol/include/transport.h
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
  - `docs/program/rfc/rfc-runtime-001-runtime-topology-and-authority.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`

## Status

Accepted and active.
