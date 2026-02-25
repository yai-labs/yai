# DEMO-SC102-WAVE1

Operator demo entrypoint for SC102 Wave 1.

References:
- Scenario: `docs/30-catalog/scenarios/SC-102.md`
- QT orchestrator: `docs/40-qualification/QT-0.1-003-SC102-WAVE1/README.md`
- Latest bundle family: `docs/40-qualification/WAVES/`
- D1 pack/trials:
  - `docs/30-catalog/domains/packs/D1-digital/egress-v1/README.md`
  - `docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-001-curl-egress-v1/README.md`
  - `docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-002-otel-export-egress-v1/README.md`
  - `docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-003-s3-upload-egress-v1/README.md`
- D8 pack/trial:
  - `docs/30-catalog/domains/packs/D8-scientific/reproducibility-parameter-lock-v1/README.md`
  - `docs/30-catalog/domains/trials/D8-scientific/reproducibility-parameter-lock-v1/RT-001-params-lock-v1/README.md`

Run:
```bash
cd docs/40-qualification/DEMO/DEMO-SC102-WAVE1
./run/run-demo.sh
```

Verify demo capture:
```bash
./verify/verify-demo.sh
```
