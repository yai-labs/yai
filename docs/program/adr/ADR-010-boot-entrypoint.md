---
id: ADR-010
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/program/milestone-packs/runtime-baselines/root-hardening.md
  phase: boot-baseline
  anchor: "#phase-root-boot-baseline"
law_refs:
  - ../law/foundation/axioms/A-002-authority.md
  - ../law/foundation/invariants/I-003-governance.md
  - ../law/foundation/boundaries/L1-kernel.md
  - ../law/contracts/protocol/include/transport.h
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

## Traceability

- Proposals:
  - `docs/program/rfc/RFC-003-workspace-lifecycle-and-isolation.md`
- Implemented by runbooks:
  - `docs/program/milestone-packs/runtime-baselines/workspaces-lifecycle.md`
  - `docs/program/milestone-packs/runtime-baselines/engine-attach.md`
- Milestone packs:
  - `docs/program/milestone-packs/workspaces-lifecycle/MP-WORKSPACES-LIFECYCLE-0.1.0.md` *(planned)*

## Status

Accepted and active.
