# Specs Bridge (YAI ↔ yai-specs)

This repository carries human-readable architecture and operational delivery.
The **normative truth** (contracts/specs/formal) lives in:

- `deps/yai-specs/contracts/*`
- `deps/yai-specs/specs/*`
- `deps/yai-specs/formal/*`

This page explains how to cite specs correctly and how docs remain subordinate to them.

## What is normative
The following are considered normative:
- contracts: axioms, invariants, boundaries
- specs: protocol schemas, role/error taxonomies, formal interfaces
- formal: TLA+ specs and proof obligations (when present)

Everything in `docs/` is explanatory or operational, never normative.

## How to cite yai-specs in this repo

Use stable paths (and IDs when present) in every ADR/Runbook/MP that touches governance, protocol, roles, errors, or boundaries.

### Citation style (recommended)
- Prefer paths inside `deps/yai-specs/...`
- When a document has an ID (e.g. invariants I-001), include both:

Example:
- `deps/yai-specs/contracts/invariants/I-001-traceability.md`
- `deps/yai-specs/contracts/boundaries/L1-kernel.md`
- `deps/yai-specs/specs/protocol/include/transport.h`
- `deps/yai-specs/specs/protocol/include/errors.h`

## What changes where
- If you change the law (contracts/specs): it belongs in `yai-specs` (upstream).
- If you explain the law or implement it here: it belongs in `docs/*` + code in this repo.

If a change requires touching both:
- treat it as a “twin change” across repos and link evidence in the MP/PR body.

## Practical mapping (minimum)

- Protocol & authority semantics → `deps/yai-specs/specs/protocol/*`
- Boundaries (L0/L1/L2/L3) → `deps/yai-specs/contracts/boundaries/*`
- Determinism/traceability invariants → `deps/yai-specs/contracts/invariants/*`
- Formal model references → `deps/yai-specs/formal/*`
