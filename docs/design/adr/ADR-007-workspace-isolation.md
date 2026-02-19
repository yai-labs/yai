---
id: ADR-007
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/workspaces-lifecycle.md
  phase: 0.1.0
  anchor: "#phase-0-1-0-workspace-layout"
law_refs:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/contracts/invariants/I-002-determinism.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/specs/protocol/include/session.h
  - deps/yai-specs/specs/protocol/include/transport.h
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
  - `docs/design/proposals/PRP-003-workspace-lifecycle-and-isolation.md`
- Implemented by runbooks:
  - `docs/runbooks/workspaces-lifecycle.md`
  - `docs/runbooks/engine-attach.md`
- Milestone packs:
  - *(TBD for workspace/engine tracks)*

## Status

Accepted and active.
