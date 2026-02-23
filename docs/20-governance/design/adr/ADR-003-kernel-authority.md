---
id: ADR-003
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: 0.1.2
  anchor: "#phase-0-1-2-envelope-authority-gate"
law_refs:
  - deps/yai-specs/contracts/axioms/A-002-authority.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/specs/protocol/include/auth.h
  - deps/yai-specs/specs/protocol/include/session.h
---
# ADR-003 - Kernel as Authority Plane (L1)

## Context

Authority checks were historically mixed with execution paths. This weakened guarantees around role/arming/workspace enforcement.

## Decision

Kernel is the sole authority plane and validates:

- Handshake and protocol conformance
- Role and arming constraints
- Workspace binding and session ownership

No effectful execution is allowed before Kernel authorization.

## Rationale

Separating authority from execution preserves deterministic policy behavior and ensures that Engine cannot be used as an authorization surface.

## Consequences

- Positive:
  - Clear trust boundary between policy and execution.
  - Consistent reject semantics for unauthorized requests.
- Negative:
  - Additional coordination needed when evolving command lifecycle.

## Traceability

- Proposals:
  - `docs/design/proposals/PRP-001-runtime-topology-and-authority.md`
- Implemented by runbooks:
  - `docs/runbooks/root-hardening.md`
- Milestone packs:
  - `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

## Status

Accepted and active.
