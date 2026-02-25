# Evidence Executive Summary

## Identity
- Trial: `RT-0.1-004-D1-GITHUB-ISSUE-COMMENT`
- Run: `run-001`
- Domain pack: `D1-digital/egress-v1`
- Baseline ID: `baseline-allow`
- Baseline Hash: `79c578d85000c82e7481bed0252e24043a5aa9e9b17fe3847a8321692dfd0308`
- Contract Ref: `policy.network_egress.allowlist`

## Decision
- Outcome: `allow`
- Reason code: `EGRESS_DEST_CONTRACTED`
- Enforcement result: `allowed`

## Traceability
- ws_id: `wrt4-run-001`
- trace_id: `trt4-run-001`
- principal.id: `principal-rt004`

## KPI Highlights
- target_reachable: `true`
- connect_established: `true`
- bytes_exfiltrated: `101`
- time_to_contain_ms: `2132`
- forbidden_effect_block_rate: `0.0`
- containment_success_rate: `0.0`

## Artifact Set
- `baseline.json`
- `decision_records.jsonl`
- `decision_records.pretty.json`
- `timeline.jsonl`
- `timeline.pretty.json`
- `containment_metrics.json`
- `KPI_SUMMARY.md`
- `system_state.txt`
