---
id: FINDING-001
status: active
owner: governance
effective_date: 2026-02-24
related:
  - docs/30-catalog/scenarios/SC-102.md
  - docs/40-qualification/QT-0.1-001-SC102/README.md
  - docs/30-catalog/domains/packs/D1-digital/egress-v1/pack.meta.json
  - docs/50-validation/audits/claims/infra-grammar.v0.1.json
---

# FINDING-001 - SC-102 D1 Harness 3-Run Evidence

## Summary
`QT-0.1-001-SC102` simulation harness for `D1-digital/egress-v1` executed 3 coherent runs on `baseline-deny`.

## Command
```bash
cd docs/40-qualification/QT-0.1-001-SC102
DOMAIN_PACK_ID=D1-digital/egress-v1 BASELINE_ID=baseline-deny ./run/run-three.sh
```

## Result
- 3/3 PASS
- forbidden effect success rate = 0.0
- decision outcome/reason stable across runs
- evidence artifacts generated per run

## Evidence paths
- `docs/40-qualification/QT-0.1-001-SC102/evidence/D1-digital/egress-v1/run-001/`
- `docs/40-qualification/QT-0.1-001-SC102/evidence/D1-digital/egress-v1/run-002/`
- `docs/40-qualification/QT-0.1-001-SC102/evidence/D1-digital/egress-v1/run-003/`

## Claim impact
This finding supports Gate A progress for:
- `C-DOMAIN-COVERAGE-NETWORK`
- `C-EVIDENCE-PACK-REPRODUCIBLE`

Status note:
- Kept as supporting evidence for current baseline advancement.
- Final claim status transitions remain constrained by full gate closure policy.
