---
id: ARCH-COMP-MIND
status: active
effective_date: 2026-02-19
revision: 1
owner: cognition
law_refs:
  - deps/yai-specs/contracts/boundaries/L3-mind.md
  - deps/yai-specs/contracts/invariants/I-004-cognitive-reconfiguration.md
---

# Mind Component

## Role

Workspace-scoped cognitive/proposal plane that may generate plans but must not execute authority-bound effects.

## Current Implementation Status

planned/external

## Interfaces and Entry Points

- Planned contracts and references: `docs/runbooks/mind-redis-stm.md`, `docs/design/adr/ADR-005-mind-proposer.md`
- Current local implementation status: no tracked source under `yai/mind` (build artifacts only).

## Authority and Boundary Rules

- Mind cannot be an authority enforcement surface.
- Mind must pass proposals through governed runtime boundaries.

## Traceability

- ADR refs: `docs/design/adr/ADR-005-mind-proposer.md`
- Runbook refs: `docs/runbooks/mind-redis-stm.md`
- MP refs: *(TBD)*
- L0 anchors: `deps/yai-specs/contracts/boundaries/L3-mind.md`, `deps/yai-specs/contracts/invariants/I-004-cognitive-reconfiguration.md`

## Known Drift / Gaps

- ADR and runbook define direction, but implementation is currently external/not tracked in this repo.

## Next Alignment Steps

- Reintroduce tracked implementation sources before promoting status beyond planned/external.
