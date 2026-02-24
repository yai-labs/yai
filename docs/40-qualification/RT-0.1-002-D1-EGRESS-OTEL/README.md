---
id: RT-0.1-002-D1-EGRESS-OTEL
title: RealTarget Trial - D1 Egress OTEL export
type: realtarget-trial
status: draft
owner: runtime
effective_date: 2026-02-24
revision: 1
domain_pack_id: D1-digital/egress-v1
catalog_trial_ref: docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-002-otel-export-egress-v1
---

# RT-0.1-002-D1-EGRESS-OTEL

LIVE-only OTEL export-style egress trial.

```bash
cd docs/40-qualification/RT-0.1-002-D1-EGRESS-OTEL
BASELINE_ID=baseline-deny ./run/run-three.sh
```


Local controlled (L0):
```bash
TARGET_PROFILE=local BASELINE_ID=baseline-deny ./run/run-three.sh
```

Remote controlled (L1):
```bash
TARGET_PROFILE=remote TARGET_SCHEME=https TARGET_HOST=<subdomain> TARGET_PORT=443 BASELINE_ID=baseline-deny ./run/run-three.sh
```


Note:
- Default runs write to `evidence.local/` (git-ignored).
- To refresh tracked audit evidence under `evidence/`, run with `EVIDENCE_ROOT="$PWD/evidence"`.
