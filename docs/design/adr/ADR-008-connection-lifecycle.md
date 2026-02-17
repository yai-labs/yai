# ADR-008 — Connection Lifecycle Semantics

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

Connections are one of:

- root session
- workspace-attached session

Handshake establishes protocol validity.
Attach establishes workspace context.

### Rules

- No execution before attach
- Attach must be explicit
- Reconnect must re-handshake

### Status

One-shot CLI stable.
Persistent cockpit session model pending.

---

## Status

Active
