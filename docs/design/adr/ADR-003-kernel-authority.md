# ADR-003 — Kernel as Authority Plane (L1)

## Context

# YAI Architecture Decisions (Law-Aligned, 2026 Revision)

This document captures the **machine-level architecture commitments**
of YAI as of the current runtime refactor phase.

It is grounded in `deps/yai-specs/contracts/` invariants and reflects the
post-envelope, post-authority enforcement state.

The architecture is stratified across:

- L0 — Vault (immutable identity & ABI boundary)
- L1 — Kernel (authority, sessions, isolation)
- L2 — Engine (execution gates)
- L3 — Mind (proposal-only cognition per workspace)
- Root — Machine Control Plane (runtime governor)

---

### Decision

Kernel is the **authority enforcement layer**.

It validates:

- protocol version
- handshake
- role
- arming flag
- ws_id
- session ownership

It is the only layer that may authorize effectful execution.

### Enforcement Rules

- `arming=true` requires role ≥ operator
- No execution without handshake
- No execution before workspace attach
- No cross-workspace session mixing

### Non-Goals

- Kernel does not perform business logic
- Kernel does not execute provider/storage logic
- Kernel does not own cognition

### Status

Authority enforcement active.
Session locking stabilized (robust PID validation).

---

## Status

Active
