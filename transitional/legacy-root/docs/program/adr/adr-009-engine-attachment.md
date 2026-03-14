---
role: support
status: draft
audience: governance
owner_domain: program-adr
id: ADR-009
decision_id: ADR-009
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
# ADR-009 - Engine Attachment Model

## Context

Current runtime integration still carries transitional wiring. A final attachment model is needed to eliminate per-workspace execution coupling.

## Decision

Engine is attached as a shared runtime plane under Root governance; workspace context is passed through dispatch metadata rather than process topology.

Target model:

- Root governs ingress and routing
- Kernel enforces authority
- Engine executes within authorized workspace context

## Rationale

Shared attachment improves operability and keeps execution behavior aligned with the single-runtime strategy.

## Consequences

- Positive:
  - Lower operational complexity.
  - Cleaner governance boundary from ingress to effects.
- Negative:
  - Requires careful migration of existing workspace-oriented assumptions.

## Traceability

- Proposals:
  - `docs/program/rfc/rfc-003-workspace-lifecycle-isolation.md`
- Implemented by runbooks:
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-workspaces-lifecycle.md`
  - `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-engine-attach.md`
- Milestone packs:
  - `docs/archive/legacy/program/milestone-packs/engine-attach/mp-orchestration-000-engine-attach-v0-1-5.md` *(planned)*

## Status

Draft; acceptance tied to engine-attach runbook completion.
