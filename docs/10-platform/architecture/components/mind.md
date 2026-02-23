---
id: ARCH-COMP-MIND
status: active
effective_date: 2026-02-23
revision: 1
owner: mind
law_refs:
  - deps/yai-specs/contracts/boundaries/L3-mind.md
  - deps/yai-specs/contracts/invariants/I-004-cognitive-reconfiguration.md
---

# Mind Component

## Role

Cognitive proposer plane (L3) that can suggest plans/actions but cannot directly enforce external effects.

## Current Implementation Status

partial

## Interfaces and Entry Points

- `mind/src/main.rs`
- `mind/src/cognition/mod.rs`
- `mind/src/cognition/orchestration/mod.rs`
- `mind/src/transport/mod.rs`

## Authority and Boundary Rules

- Mind is proposer-only and must not become an authority or enforcement surface.
- All effectful decisions remain governed by Root/Kernel/Engine contracts.
- Proposal context must preserve workspace and trace identifiers end-to-end.

## Traceability

- ADR refs: `docs/20-governance/design/adr/ADR-005-mind-proposer.md`, `docs/20-governance/design/adr/ADR-003-kernel-authority.md`
- Runbook refs: `docs/20-governance/runbooks/mind-redis-stm.md`, `docs/20-governance/runbooks/root-hardening.md`
- MP refs: `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`
- L0 anchors: `deps/yai-specs/contracts/boundaries/L3-mind.md`, `deps/yai-specs/contracts/invariants/I-004-cognitive-reconfiguration.md`

## Known Drift / Gaps

- End-to-end evidence for proposer path through Kernel enforcement is still partial.
- Redis STM integration and governance checks are not fully closed at milestone-pack level.

## Next Alignment Steps

- Close `RB-MIND-REDIS-STM` phase evidence with mandatory verify/suite/proof outputs.
- Keep architecture alignment and claims registry synchronized as mind milestones close.
