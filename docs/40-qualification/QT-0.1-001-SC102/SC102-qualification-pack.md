---
id: SC102-QUALIFICATION-PACK
title: SC102 Qualification Pack - D1 Egress Containment
status: draft
owner: runtime
effective_date: 2026-02-24
revision: 1
---

# SC102 Qualification Pack - D1 Egress Containment

## Intent
Demonstrate governed containment for D1 egress under pinned baseline hash and reproducible evidence.

## Invariants
- `decision.outcome=deny` for non-contracted destination under `baseline-deny`
- `reason_code=EGRESS_DEST_NOT_CONTRACTED`
- `connect_established=false`
- `bytes_exfiltrated=0`
- `target_reachable=true`

## Coverage
- RT-0.1-001-D1-EGRESS-CURL
- RT-0.1-002-D1-EGRESS-OTEL
- RT-0.1-003-D1-EGRESS-S3

## Run Commands (L0)
```bash
cd docs/40-qualification/RT-0.1-001-D1-EGRESS-CURL && BASELINE_ID=baseline-deny TARGET_PROFILE=local ./run/run-three.sh
cd docs/40-qualification/RT-0.1-002-D1-EGRESS-OTEL && BASELINE_ID=baseline-deny TARGET_PROFILE=local ./run/run-three.sh
cd docs/40-qualification/RT-0.1-003-D1-EGRESS-S3 && BASELINE_ID=baseline-deny TARGET_PROFILE=local ./run/run-three.sh
```

## L1 Bridge (Remote controlled)
```bash
TARGET_PROFILE=remote TARGET_SCHEME=https TARGET_HOST=curl.<your-domain> TARGET_PORT=443 BASELINE_ID=baseline-deny ./run/run-three.sh
```
Apply analogous host/path mapping for OTEL and S3 trials.


## Workload completeness note
Full workload-completeness validation is controlled in dedicated workload validation tracks outside this RT harness.
SC102 RT focus remains governance containment + evidence integrity.
