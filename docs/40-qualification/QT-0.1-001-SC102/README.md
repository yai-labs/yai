---
id: QT-0.1-001-SC102
title: Qualification Gate - SC-102 Core Contain -> Evidence (Cross-Domain)
status: active
owner: runtime
effective_date: 2026-02-23
revision: 3
sc_ref: SC-102
---

# QT-0.1-001-SC102 - Qualification Gate (Core-only, Cross-Domain)

## 1) Goal
Execute SC-102 as a repeatable pass/fail gate for Core qualification.
The harness is parameterized by `domain_pack_id`; runtime grammar stays invariant.

## 2) Required inputs
Each run declares:
- `DOMAIN_PACK_ID` (default: `D1-digital/egress-v1`)
- `BASELINE_ID` (default: `baseline-deny`)
- `WORKLOAD_ID` (default: `wrk-d1-egress-sim-v1`)
- `ATTACK_PROFILE_ID` (default: `safe-egress-attempt-001`)
- `RUN_ID` (`run-001|run-002|run-003`)

## 3) Harness structure
- `baseline/` baseline materials for gate context.
- `workload/` workload descriptors.
- `attacker/` safe stimulus descriptors.
- `run/` executable flow:
  - `01-start-runtime.sh`
  - `02-create-workspace.sh`
  - `03-start-workload.sh`
  - `04-attack.sh`
  - `05-collect-evidence.sh`
  - `06-assert-passfail.sh`
  - `run-once.sh`
  - `run-three.sh`
- `metrics/parse_containment.py` metrics parser.
- `evidence/<domain_pack_id>/run-00X/` evidence output per run.

## 4) Execution
Single run:

```bash
cd docs/40-qualification/QT-0.1-001-SC102
DOMAIN_PACK_ID=D1-digital/egress-v1 \
BASELINE_ID=baseline-deny \
RUN_ID=run-001 \
./run/run-once.sh
```

Three coherent runs:

```bash
cd docs/40-qualification/QT-0.1-001-SC102
DOMAIN_PACK_ID=D1-digital/egress-v1 \
BASELINE_ID=baseline-deny \
./run/run-three.sh
```

## 5) Pass criteria
A run passes when:
- forbidden effect is blocked (deny/quarantine baseline expectation)
- decision record includes outcome + reason_code + baseline_hash
- evidence files are complete and indexed
- containment metrics confirm `connect_established=false` and `bytes_exfiltrated=0`

Gate passes when:
- 3 consecutive runs pass for the same `domain_pack_id` + baseline.

## 6) Evidence layout (per run)
`evidence/<domain_pack_id>/run-00X/`
- `baseline.json`
- `timeline.jsonl`
- `decision_records.jsonl`
- `containment_metrics.json`
- `system_state.txt`
- `EVIDENCE_INDEX.md`

## 7) Safety notes
- Simulation-only harness for v0.1 core gate.
- No mind/cockpit dependency.
- No real external effects.


## 8) SC102 Qualification Pack
- `SC102-qualification-pack.md`
- `SC102-evidence-index.md`
- `SC102-deployment-notes-l1.md`

## 9) L0/L1 bridge
- L0 local controlled: `TARGET_PROFILE=local`
- L1 remote controlled: `TARGET_PROFILE=remote TARGET_SCHEME=https TARGET_HOST=<subdomain> TARGET_PORT=443`
