---
id: ADR-009
status: draft
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/engine-attach.md
  phase: v4
  anchor: "#phase-engine-attach-v4"
---
# ADR-009 — Engine Attachment Model (Next Phase)

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

Engine will be attached to Root runtime,
not directly spawned per workspace.

Workspace context will be passed through dispatch layer.

### Future Model

Root
 ├── Kernel (authority)
 ├── Engine (shared execution plane)
 └── Workspace contexts (logical isolation)

### Status

Planned.
Not yet fully integrated.

---

## Status

Active
