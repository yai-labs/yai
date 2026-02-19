---
id: ADR-008
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/workspaces-lifecycle.md
  phase: 0.1.1
  anchor: "#phase-0-1-1-ws-create-guardrails"
law_refs:
  - deps/yai-specs/contracts/invariants/I-001-traceability.md
  - deps/yai-specs/contracts/invariants/I-002-determinism.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/specs/protocol/include/session.h
  - deps/yai-specs/specs/protocol/include/transport.h
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
  - `docs/design/proposals/PRP-003-workspace-lifecycle-and-isolation.md`
- Implemented by runbooks:
  - `docs/runbooks/workspaces-lifecycle.md`
  - `docs/runbooks/engine-attach.md`
- Milestone packs:
  - *(TBD for workspace/engine tracks)*

## Status

Accepted and active, with remaining evidence hardening tracked in runbook phases.
