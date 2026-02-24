---
id: RT-0.1-001-D8-PARAMS-LOCK
title: RT-0.1-001 - D8 Scientific Params Lock RealTarget Trial
status: draft
owner: qualification
effective_date: 2026-02-24
revision: 2
domain_pack_id: D8-scientific/reproducibility-parameter-lock-v1
catalog_trial_ref: docs/30-catalog/domains/trials/D8-scientific/reproducibility-parameter-lock-v1/RT-001-params-lock-v1
---

# RT-0.1-001-D8-PARAMS-LOCK

RealTarget trial for D8 reproducibility parameter lock using black-box artifact publish checks.

## Profiles
- `TARGET_PROFILE=local`: local store path under this RT folder (L0).
- `TARGET_PROFILE=docker`: dockerized isolated store/runtime/workload (L1).

## Variants (deny baseline)
- `run-001`: lock missing -> `PARAM_LOCK_MISSING`
- `run-002`: lock mismatch -> `PARAMS_LOCK_MISMATCH`
- `run-003`: lock invalid -> `PARAMS_LOCK_INVALID`

## Run
```bash
cd docs/40-qualification/RT-0.1-001-D8-PARAMS-LOCK
BASELINE_ID=baseline-deny TARGET_PROFILE=local ./run/run-three.sh
```

Docker profile:
```bash
docker compose -f docs/40-qualification/_shared/d8-artifact-store/docker-compose.yml up -d
cd docs/40-qualification/RT-0.1-001-D8-PARAMS-LOCK
BASELINE_ID=baseline-deny TARGET_PROFILE=docker ./run/run-three.sh
```

## Black-box proof
```bash
find docs/40-qualification/_shared/d8-artifact-store/_artifact_store -type f | wc -l
BASELINE_ID=baseline-deny TARGET_PROFILE=docker ./run/run-three.sh
find docs/40-qualification/_shared/d8-artifact-store/_artifact_store -type f | wc -l
```

Expected under `baseline-deny`: unchanged count, `outputs_persisted=false`, `bytes_written=0`, `artifacts_delta=0`.
