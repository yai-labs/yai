---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-008
decision_id: ADR-008
supersedes: []
superseded_by: []
implements: [docs/program/rfc/rfc-003-workspace-lifecycle-isolation.md]
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#phase-0-1-1-ws-create-guardrails"
applies_to: 
effective_date: 2026-02-18
law_refs: 
phase: 0.1.1
runbook: docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md
---
# ADR-008 - Connection Lifecycle Semantics

## Context

Connection semantics are foundational for deterministic control-plane behavior and must stay consistent across Root and workspace-attached sessions.

## Decision

Two connection states are supported:

- Root session
- Workspace-attached session

Lifecycle rules:

- Handshake is mandatory before execution
- Workspace attach is mandatory for runtime-bound operations
- Reconnect requires fresh handshake
- Reject paths must remain deterministic and traceable

## Rationale

A strict lifecycle avoids hidden state transitions and improves forensic clarity for failures.

## Consequences

- Positive:
  - Cleaner protocol guarantees and stable observability.
  - Better fit for non-skip proof requirements.
- Negative:
  - Partial implementations cannot be presented as full readiness.

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

Accepted and active, with remaining evidence hardening tracked in runbook phases.
