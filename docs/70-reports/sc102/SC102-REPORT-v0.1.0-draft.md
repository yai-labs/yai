# SC102 REPORT v0.1.0 (Draft)

## Executive Summary

SC102 Wave 1 demonstrates governed runtime containment across two different domain semantics using the same runtime grammar (`contract -> decision -> enforcement -> evidence`).

## Release Identity

Source of truth:

- `docs/40-qualification/WAVES/<wave_id>/MANIFEST.json`

Minimum identity fields to report:

- `yai_git_sha`
- `yai_cli_git_sha`
- `specs_pin_sha`
- `version_label`


## Launch Reference (Fixed 60-Day ID)

- Launch ID: `SC102-WAVE1-LAUNCH`
- Frozen wave ID: `WAVE-1-2026-02-25-bf59d51`
- Manifest SHA-256: `7f6806492950858a1b78fe99bb02701443eba77a371dfea2095cd0df2cb5b100`
- Index SHA-256: `9b320ec5d5edcd7ccc5630a940bf9d5e8af36f6fc808350b8990eed92a2c4f6c`
- yai sha: `bf59d51941456042b71f268468d9a62dbf53d74d`
- yai-cli sha: `72e487ee55de2efaa7de71374427421a923aa5ed`
- specs pin sha: `20abef1874e56e4c3493df5a42697779cba00381`

## Wave Coverage

Wave entrypoint:

- `docs/40-qualification/QT-0.1-003-SC102-WAVE1/`

Covered packs:

- `D1-digital/egress-v1`
- `D8-scientific/reproducibility-parameter-lock-v1`

## Star Case Evidence Summary

Star Case: `AI Production Change Guard`

Verification chain:

1. Execute wave: `run/run-wave.sh`
2. Verify bundle: `verify/verify.sh`
3. Consume summary in `INDEX.md`

Expected key outcomes for customer-facing proof:

- D1 rows: `outcome=deny`, `reason=EGRESS_DEST_NOT_CONTRACTED`, `connect_established=false`, `bytes_exfiltrated=0`
- D8 deny row: `outcome=deny`, `reason=PARAMS_LOCK_INVALID`, `outputs_persisted=false`, `bytes_written=0`, `artifacts_delta=0`
- D8 allow row: `outcome=allow`, `reason=PARAMS_LOCK_VALID`, `outputs_persisted=true`, `bytes_written>0`, `artifacts_delta>0`

## Operator Demo

- reference: `docs/40-qualification/DEMO/DEMO-SC102-WAVE1/`

## Benchmark Snapshot

- reference: `docs/50-validation/benchmarks/sc102/`

## Commercial Readout Template

For each customer run include:

- Customer/profile identifier
- `wave_id`
- verification result
- KPI deltas versus baseline process
- go/no-go recommendation

## Appendix

Primary links:

- `docs/80-commercial/COMMERCIAL-PLAN-v1.0.md`
- `docs/30-catalog/scenarios/SC-102.md`
- `docs/40-qualification/WAVES/<wave_id>/INDEX.md`
- `docs/40-qualification/WAVES/<wave_id>/MANIFEST.json`
