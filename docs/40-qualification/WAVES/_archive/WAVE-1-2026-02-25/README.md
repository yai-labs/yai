---
id: WAVE-1-2026-02-25
title: SC-102 Wave 1 Qualification Bundle (D1 + D8)
status: draft
owner: runtime
effective_date: 2026-02-25
revision: 1
catalog_scenario_ref: docs/30-catalog/scenarios/SC-102.md
---

# WAVE-1-2026-02-25

Publishable SC-102 Wave 1 bundle with deterministic evidence selection.

## What is demonstrated
- D1 digital egress containment (`baseline-deny`) across 3 RealTarget surfaces:
  - curl
  - OTEL export
  - S3 upload
- D8 scientific params-lock containment (`baseline-deny`) on docker artifact-store profile.

## Bundle contract
This wave includes the 3 mandatory outputs:
1. Entrypoint (`README.md`)
2. Evidence bundle (`evidence/` + `MANIFEST.json` + `INDEX.md`)
3. Replay/verify command (`verify/verify.sh`)

## Selected runs
Only selected runs are included (run-003 per trial) for external consumption.

## Path normalization policy
- Evidence may include host-specific absolute paths from raw runtime receipts.
- Consumers must use `*_ref` fields from `MANIFEST.json` as canonical references.

## Verify (replay)
```bash
cd docs/40-qualification/WAVES/WAVE-1-2026-02-25
./verify/verify.sh
```

Expected output:
- `PASS: Wave 1 bundle verified`
- regenerated `INDEX.md`.
