---
id: QT-0.1-003-SC102-WAVE1
title: SC-102 Wave 1 Aggregate Gate (D1 + D8)
status: draft
owner: runtime
effective_date: 2026-02-25
revision: 3
catalog_scenario_ref: docs/30-catalog/scenarios/SC-102.md
---

# QT-0.1-003-SC102-WAVE1

Canonical SC102 Wave 1 orchestrator.

Scope:
- D1 digital egress containment (RT-001/002/003)
- D8 scientific params-lock containment (RT-001 deny + allow)

## One-button run

```bash
cd docs/40-qualification/QT-0.1-003-SC102-WAVE1
./run/run-wave.sh
```

## Runtime output (outside repo)

Evidence and logs are written to:

`~/.yai/qualifications/SC102/<wave_id>/<timestamp>/`

## Bundle outputs (inside repo)

Canonical build bundle:

`docs/40-qualification/WAVES/<wave_id>/`

Stable launch alias (public-facing):

`docs/40-qualification/WAVES/SC102-WAVE1-LAUNCH/`

Generated artifacts:
- `README.md`
- `MANIFEST.json`
- `INDEX.md`
- `POLICY/` (pinned deny/allow policy files + hashes)
- `RUNS/deny/`
- `RUNS/allow/`
- `verify/verify.sh`
- `verify/verify_wave.py`

## Configuration

Single source of truth:

`docs/40-qualification/QT-0.1-003-SC102-WAVE1/wave/wave.yaml`

To add a new pack/domain to the wave, edit only `wave/wave.yaml`.

## Commercial consumption

This QT is the execution entrypoint used by commercial collateral.

Upstream business context:
- `docs/80-commercial/COMMERCIAL-PLAN-v1.0.md`
- `docs/30-catalog/scenarios/SC-102.md`

Downstream proof package:
- `docs/40-qualification/WAVES/SC102-WAVE1-LAUNCH/MANIFEST.json`
- `docs/40-qualification/WAVES/SC102-WAVE1-LAUNCH/INDEX.md`
- `docs/40-qualification/WAVES/SC102-WAVE1-LAUNCH/verify/verify.sh`
