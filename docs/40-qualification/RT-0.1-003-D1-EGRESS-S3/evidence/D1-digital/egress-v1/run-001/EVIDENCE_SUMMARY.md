# Evidence Executive Summary

## Identity
- Trial: `RT-0.1-003-D1-EGRESS-S3`
- Run: `run-001`
- Domain pack: `D1-digital/egress-v1`
- Baseline ID: `baseline-deny`
- Baseline Hash: `4214c2e8d1c75dd97cfe3bdb961d186da8927c24d341de9fe864ce3a9df443fb`
- Contract Ref: `policy.network_egress.allowlist`

## Decision
- Outcome: `deny`
- Reason code: `EGRESS_DEST_NOT_CONTRACTED`
- Enforcement result: `blocked`

## Traceability
- ws_id: `ws-rt003-run-001`
- trace_id: `trace-rt003-run-001`
- principal.id: `principal-rt003`

## KPI Highlights
- local_target_reachable: `true`
- connect_established: `false`
- bytes_exfiltrated: `0`
- time_to_contain_ms: `0`
- forbidden_effect_block_rate: `1.0`
- containment_success_rate: `1.0`

## Artifact Set
- `baseline.json`
- `decision_records.jsonl`
- `decision_records.pretty.json`
- `timeline.jsonl`
- `timeline.pretty.json`
- `containment_metrics.json`
- `KPI_SUMMARY.md`
- `system_state.txt`
