---
id: ARCH-COMP-MIND-BOUNDARIES
status: historical
effective_date: 2026-02-23
revision: 1
owner: mind
law_refs:
  - ../law/foundation/boundaries/L3-mind.md
  - ../law/foundation/invariants/I-004-cognitive-reconfiguration.md
---

# Boundaries — Mind (L3)

> Historical record: this page documents pre-unified-runtime topology and is
> not an active architecture source for current runtime ownership.

## Role

This document defines boundary constraints that Mind must obey as a proposer-only plane.

## Current Implementation Status

partial

## Interfaces and Entry Points

- `mind/src/cognition/orchestration/`
- `mind/src/memory/graph/`
- `mind/src/providers/`
- `mind/src/transport/`

## Authority and Boundary Rules

- Any external effect must be routed through governed planes (Root/Kernel/Engine).
- Mind cannot bypass authority contracts or directly enforce side effects.
- Boundary decisions must remain auditable via traces/tests.

## Traceability

- ADR refs: `docs/program/adr/ADR-005-mind-proposer.md`, `docs/program/adr/ADR-003-kernel-authority.md`
- Runbook refs: `docs/program/milestone-packs/runtime-baselines/mind-redis-stm.md`, `docs/program/milestone-packs/runtime-baselines/root-hardening.md`
- MP refs: `docs/program/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`
- L0 anchors: `../law/foundation/boundaries/L3-mind.md`, `../law/foundation/invariants/I-004-cognitive-reconfiguration.md`

## Known Drift / Gaps

- Coverage of boundary evidence across all provider modes is incomplete.
- Some drift notes are still tracked outside consolidated closure artifacts.

## Next Alignment Steps

- Complete boundary-focused evidence for provider and memory domains.
- Re-run architecture alignment generation and keep claims synchronized.
