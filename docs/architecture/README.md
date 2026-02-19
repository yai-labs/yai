---
id: ARCH-README
status: active
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/contracts/boundaries/L2-engine.md
---

# Architecture

This directory is the canonical human-readable architecture model for `yai`.
It must reflect current implementation and remain aligned to ADR + L0 anchors.

## Canonical Topology

Canonical Topology: Root -> Kernel -> Engine with Mind as planned/external L3 plane.

## Contents

- `docs/architecture/overview.md`
- `docs/architecture/runtime-model.md`
- `docs/architecture/specs-bridge.md`
- `docs/architecture/traceability.md`
- `docs/architecture/components/*.md`
- `docs/_generated/architecture-alignment.v1.json`

## Reading order

1. `docs/architecture/overview.md`
2. `docs/architecture/runtime-model.md`
3. `docs/architecture/specs-bridge.md`
4. `docs/architecture/traceability.md`
5. Component docs

## Contract

- Architecture docs are explanatory, never normative.
- Normative truth remains under `deps/yai-specs/...`.
- If architecture text disagrees with ADR/L0, ADR+L0 win.
