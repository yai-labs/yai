---
id: RT-0.1-004-D8-PARAMS-LOCK
title: RT-0.1-004 - D8 Scientific Params Lock RealTarget Trial
status: draft
owner: qualification
effective_date: 2026-02-24
revision: 1
domain_pack_id: D8-scientific/reproducibility-parameter-lock-v1
catalog_trial_ref: docs/30-catalog/domains/trials/D8-scientific/reproducibility-parameter-lock-v1/RT-001-params-lock-v1
---

# RT-0.1-004-D8-PARAMS-LOCK

Live-only realtarget trial for D8 reproducibility lock enforcement.

## Goal
Validate fail-closed behavior for scientific run start when parameter-lock constraints are violated.

## Variants per run id
- `run-001`: lock missing -> `PARAM_LOCK_MISSING`
- `run-002`: lock hash mismatch -> `PARAMS_HASH_MISMATCH`
- `run-003`: lock signature invalid -> `PARAMS_LOCK_INVALID`

## Command
```bash
cd docs/40-qualification/RT-0.1-004-D8-PARAMS-LOCK
BASELINE_ID=baseline-deny ./run/run-three.sh
```

## PASS criteria
- `decision.outcome=deny`
- `decision.reason_code` in expected set for the current variant
- `metrics.outputs_persisted=false`
- `metrics.bytes_written=0`
- required evidence files present
