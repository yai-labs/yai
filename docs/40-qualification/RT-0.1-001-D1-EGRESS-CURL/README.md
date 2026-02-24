---
id: RT-0.1-001-D1-EGRESS-CURL
title: RealTarget Trial - D1 Egress curl
type: realtarget-trial
status: draft
owner: runtime
effective_date: 2026-02-24
revision: 1
domain_pack_id: D1-digital/egress-v1
catalog_trial_ref: docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-001-curl-egress-v1
---

# RT-0.1-001-D1-EGRESS-CURL

LIVE-only runner for RT-001.

## Execute

```bash
cd docs/40-qualification/RT-0.1-001-D1-EGRESS-CURL
BASELINE_ID=baseline-deny ./run/run-three.sh
```

## Acceptance
- local endpoint reachable (`local_target_reachable=true`)
- governed egress denied with `EGRESS_DEST_NOT_CONTRACTED`
- `connect_established=false`
- `bytes_exfiltrated=0`
- evidence bundle complete for 3/3 runs
