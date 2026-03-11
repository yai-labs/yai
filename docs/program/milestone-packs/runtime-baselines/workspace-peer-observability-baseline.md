# Workspace Peer Observability Baseline (OP-3)

## Scenario

Workspace `comune-bologna-piao-2026` with three peers:
- peer A: performance/KPI scope
- peer B: planning/budget scope
- peer C: document attachments scope

## Goal

Read one workspace and understand:
- which peers are attached
- peer health/freshness
- backlog pressure per peer
- coverage and overlap signals

## Commands

### 1) Workspace source summary

```bash
yai workspace query source --ws comune-bologna-piao-2026
```

Check:
- `data.coordination.peer_count`
- `data.coordination.states`
- `data.coordination.backlog`
- `data.coordination.coverage`
- `data.coordination.scheduling_state`

### 2) Peer inspect rows

```bash
yai workspace query source.peer --ws comune-bologna-piao-2026
```

For each row verify:
- `effective_state` and `freshness`
- `peer_role` / `peer_scope`
- backlog counters
- `coverage_ref` and `overlap_state`

### 3) Coverage summary

```bash
yai workspace query source.coverage --ws comune-bologna-piao-2026
```

Use to assess:
- number of distinct scopes
- overlap pressure (`overlap_count`)
- gap risk (`gap_count`)

### 4) Graph workspace summary

```bash
yai workspace graph workspace --ws comune-bologna-piao-2026
```

Validate source graph growth and peer orchestration topology presence.

## Baseline Interpretation

- `scheduling_state=nominal`: no immediate pressure.
- `scheduling_state=backlog_pressure`: stale peers or backlog/retry are building.
- `scheduling_state=attention_required`: disconnected peers or failed backlog.

- `overlap_count > 0`: prepare OP-4 conflict/ordering checks.
- `gap_count > 0`: missing coverage in at least one expected scope.

## Notes

- This is owner-side observability baseline, not full conflict resolution.
- Coverage/overlap are operational signals and must be read with governance context.
