---
id: ADR-010
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: boot-baseline
  anchor: "#phase-root-boot-baseline"
---
# ADR-010 — Boot as Canonical Machine Entry

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

`yai` (boot) is the only official runtime entrypoint.

It performs:

- preboot validation
- directory integrity
- root socket creation
- runtime initialization

Direct launching of workspace kernel binaries is deprecated.

### Status

Migration in progress.
Boot canonicalization required before engine integration.

---

## Status

Active
