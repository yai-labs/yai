---
id: RT-001
title: D1 Egress RealTarget Trial - curl egress v1
status: draft
owner: catalog
effective_date: 2026-02-24
revision: 1
domain_pack_id: D1-digital/egress-v1
---

# RT-001-curl-egress-v1

## Purpose
Prove that D1 egress enforcement blocks a real egress attempt path even when the destination is locally reachable.

## Target
- Real tool: `curl`
- Local endpoint: `127.0.0.1:8443`
- No external internet dependency.

## What it proves
1. Target endpoint is reachable locally (precheck).
2. Governed runtime path denies egress with `EGRESS_DEST_NOT_CONTRACTED`.
3. Evidence bundle is complete and repeatable.

## Catalog -> Qualification mapping
- Catalog trial spec (this folder)
- Qualification runner: `docs/40-qualification/RT-0.1-001-D1-EGRESS-CURL/`

## Produced evidence
- `baseline.json`
- `timeline.jsonl`
- `decision_records.jsonl`
- `containment_metrics.json`
- `system_state.txt`
- `EVIDENCE_INDEX.md`
