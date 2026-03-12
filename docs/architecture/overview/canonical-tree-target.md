# Canonical Tree Target (A2)

This document defines the official repository topology target for the unified
YAI system.

## Root Macro-Topology

Canonical roots:

- `cmd/`
- `governance/`
- `foundation/`
- `formal/`
- `include/`
- `lib/`
- `docs/`
- `tests/`
- `tools/`
- `data/`

Non-canonical/transitional root:

- `archive/` (temporary migration residue only)

## Five Primary Masses

1. Runtime implementation: `cmd/`, `include/`, `lib/`
2. Governance content: `governance/`
3. Foundation + formal methods: `foundation/`, `formal/`
4. Documentation/reference: `docs/`
5. Validation/tooling/data: `tests/`, `tools/`, `data/`

## Canonical Role Lock

- `governance/` is the canonical destination for former standalone governance content.
- `foundation/` defines conceptual system basis.
- `formal/` defines formal verification artifacts and traceability models.
- `docs/` is the single central documentation space.
- migration evidence is kept in documentation/report spaces under `docs/`.

## A2 Outcome

A2 instantiates the target topology and placeholder spine so subsequent
refactor/migration slices can attach without another topological redesign.
