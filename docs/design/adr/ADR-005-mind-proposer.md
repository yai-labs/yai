# ADR-005 — Mind Per Workspace (L3 Cognitive Plane)

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

Each workspace owns one Mind instance.

Mind is:

- workspace-scoped
- proposal-only
- non-authoritative

Mind may:

- build graph state
- generate plans
- propose actions

Mind may NOT:

- execute external effects
- bypass Engine
- access other workspaces

### Rationale

Preserves:

- Cognitive isolation
- Determinism
- Law invariants

### Status

Mind remains per-workspace Rust process.
Future consolidation possible under runtime governance.

---

## Status

Active
