# ADR-007 — Workspace Isolation Model

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

Workspace isolation is enforced at three levels:

1. Session lock (PID-based lockfile)
2. Memory isolation (per-ws storage paths)
3. RPC routing (Root-bound dispatch)

### Lockfile Policy

- Lockfile contains PID
- Stale lock detection via kill(pid, 0)
- Stale locks auto-recovered

### Status

Robust lock logic active.
Future: move from file-lock to runtime registry model.

---

## Status

Active
