---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-005
decision_id: ADR-005
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#phase-mind-proposer"
applies_to: 
effective_date: 2026-02-18
law_refs: 
phase: v5.3
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-mind-redis-stm.md
---
# ADR-005 - Mind as Workspace-Scoped Proposer (L3)

## Context

Cognitive features need to evolve without becoming a hidden execution authority or a cross-workspace leakage vector.

## Decision

Each workspace owns its Mind context. Mind is proposal-oriented and non-authoritative.

Mind may:

- Build internal graph/cognitive state
- Generate plans and proposals

Mind may not:

- Execute external effects directly
- Bypass Engine/Kernel governance
- Access other workspace state by default

## Rationale

This model preserves cognitive isolation while keeping effectful operations under governed L1/L2 control.

## Consequences

- Positive:
  - Safe evolution of cognition features.
  - Clear boundary between reasoning and execution.
- Negative:
  - Additional orchestration needed for proposal-to-execution flows.

## Traceability

- Proposals:
  - `docs/program/rfc/rfc-001-runtime-topology-authority.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-root-hardening.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/root-hardening/mp-runtime-000-root-hardening-v0-1-5.md`

## Status

Accepted and active.
