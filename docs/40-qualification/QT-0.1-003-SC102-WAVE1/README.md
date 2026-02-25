---
id: QT-0.1-003-SC102-WAVE1
title: SC-102 Wave 1 Aggregate Gate (D1 + D8)
status: draft
owner: runtime
effective_date: 2026-02-25
revision: 1
catalog_scenario_ref: docs/30-catalog/scenarios/SC-102.md
---

# QT-0.1-003-SC102-WAVE1

Aggregate qualification gate for SC-102 Wave 1.

Scope:
- D1 digital egress containment (`QT-0.1-001-SC102`, live mode)
- D8 scientific params-lock containment (`RT-0.1-001-D8-PARAMS-LOCK`, docker profile)

## Run

```bash
cd docs/40-qualification/QT-0.1-003-SC102-WAVE1
./run/run-wave1.sh
```

## Expected outcome

- D1 live: `3/3` pass
- D8 docker deny: `3/3` pass
- Both stages print explicit PASS markers.

## Notes

- This gate is an orchestration wrapper. Evidence remains in the child gate/trial folders.
- It is intentionally strict: any stage failure exits non-zero.
