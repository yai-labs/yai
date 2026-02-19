---
id: ADR-010
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: boot-baseline
  anchor: "#phase-root-boot-baseline"
law_refs:
  - deps/yai-specs/contracts/axioms/A-002-authority.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/specs/protocol/include/transport.h
---
# ADR-010 - Boot as Canonical Machine Entry

## Context

Multiple startup paths historically allowed inconsistent runtime initialization and reduced confidence in machine-level governance.

## Decision

`yai` boot is the canonical runtime entrypoint.

Boot responsibilities:

- Preboot validation
- Runtime directory integrity checks
- Root socket initialization
- Ordered startup of governed planes

Direct ad-hoc startup of internal binaries is deprecated.

## Rationale

A single entrypoint improves reproducibility, policy enforcement, and incident diagnosis.

## Consequences

- Positive:
  - Consistent startup contract for operators and CI.
  - Better alignment with Root-first architecture.
- Negative:
  - Legacy scripts and habits need migration.

## Status

Accepted and active.
