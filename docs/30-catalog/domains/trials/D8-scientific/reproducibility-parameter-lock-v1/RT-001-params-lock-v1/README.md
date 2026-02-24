---
id: RT-001
title: D8 Reproducibility RealTarget Trial - params lock v1
status: draft
owner: catalog
effective_date: 2026-02-24
revision: 1
domain_pack_id: D8-scientific/reproducibility-parameter-lock-v1
---

# RT-001-params-lock-v1

## Purpose
Prove that scientific runs are fail-closed when parameter-lock requirements are not satisfied.

## Target
- Real target: local publish path (`artifacts/`)
- Real guard: params lock verification (`params.json` + `params.lock`)
- No external network dependency

## What it proves
1. Run start is blocked when lock is missing/mismatched/invalid.
2. No output artifact is persisted when denied.
3. Evidence includes baseline hash + params digests + reason code.

## Catalog -> Qualification mapping
- Catalog trial spec (this folder)
- Qualification runner: `docs/40-qualification/RT-0.1-001-D8-PARAMS-LOCK/`

## Produced evidence
- `baseline.json`
- `timeline.jsonl`
- `decision_records.jsonl`
- `containment_metrics.json`
- `system_state.txt`
- `EVIDENCE_INDEX.md`
