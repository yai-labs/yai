---
id: ARCH-README
status: historical
effective_date: 2026-02-19
revision: 1
owner: architecture
law_refs:
  - deps/law/foundation/boundaries/L1-kernel.md
  - deps/law/foundation/boundaries/L2-engine.md
---

# Architecture

> Historical architecture pack: retained for traceability.
> Current runtime truth is single-binary `yai` ingress (`~/.yai/run/control.sock`).

This directory is the canonical human-readable architecture model for `yai`.
It must reflect current implementation and remain aligned to ADR + L0 anchors.

## Canonical Topology

Canonical Topology: Root -> Kernel -> Engine with Mind as planned/external L3 plane.

## Contents

- `docs/10-platform/architecture/overview.md`
- `docs/10-platform/architecture/runtime-model.md`
- `docs/10-platform/architecture/law-bridge.md`
- `docs/10-platform/architecture/traceability.md`
- `docs/10-platform/architecture/components/*.md`
- `docs/_generated/architecture-alignment.v1.json`

## Reading order

1. `docs/10-platform/architecture/overview.md`
2. `docs/10-platform/architecture/runtime-model.md`
3. `docs/10-platform/architecture/law-bridge.md`
4. `docs/10-platform/architecture/traceability.md`
5. Component docs

## Contract

- Architecture docs are explanatory, never normative.
- Normative truth remains under `deps/law/...`.
- If architecture text disagrees with ADR/L0, ADR+L0 win.
