---
id: ADR-004
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/engine-attach.md
  phase: v4
  anchor: "#phase-engine-attach-v4"
---
# ADR-004 — Engine as Execution Plane (L2)

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

Engine is the **execution gate layer**.

It provides:

- storage_gate
- provider_gate
- network_gate
- resource_gate
- cortex execution

Engine executes only after Kernel authorization.

### Rules

- Engine never validates authority
- Engine never selects workspace
- Engine never bypasses Kernel

### Relationship

Kernel → Engine (downward call)
Engine → Kernel (never)

### Status

Engine routing stub integrated.
Full integration with Root pending.

---

## Status

Active
