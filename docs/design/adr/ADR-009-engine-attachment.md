---
id: ADR-009
status: draft
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/engine-attach.md
  phase: v4
  anchor: "#phase-engine-attach-v4"
law_refs:
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/contracts/boundaries/L2-engine.md
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
  - `docs/design/proposals/PRP-003-workspace-lifecycle-and-isolation.md`
- Implemented by runbooks:
  - `docs/runbooks/workspaces-lifecycle.md`
  - `docs/runbooks/engine-attach.md`
- Milestone packs:
  - *(TBD for workspace/engine tracks)*

## Status

Draft; acceptance tied to engine-attach runbook completion.
