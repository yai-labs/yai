# Workspace Peer Conflict Baseline (OP-4)

## Goal

Operate one workspace with multiple peers when replay, overlap, or ordering
ambiguity appears.

## Main Operator Surface

```bash
yai workspace query source --ws <workspace>
yai workspace query source.conflicts --ws <workspace>
yai workspace query source.peer --ws <workspace>
```

## What to Read

### 1) `source` coordination

Focus on:

- `coordination.integrity.replay_detected`
- `coordination.integrity.overlap_detected`
- `coordination.integrity.conflict_detected`
- `coordination.integrity.ordering_late`
- `coordination.integrity.review_required`

### 2) `source.conflicts` rows

Check per-outcome:

- `classification`
- `handling_action`
- `ordering_status`
- `replay_status`
- `overlap_status`
- `review_required`

### 3) `source.peer` rows

Check per-peer pressure and latest integrity markers:

- backlog counters
- integrity counters
- last classification/handling

## Operational Interpretation

- `duplicate_replay` + `accept_with_flag`:
  expected retry/replay behavior, monitor growth trend.
- `ordering_late`:
  stale event arrival, verify peer freshness and backlog.
- `overlap_ambiguous`:
  two peers covering close/overlapping asset zone.
- `conflict_requires_review` + `review_required=true`:
  keep ingest, escalate for explicit review path.

## Backlog Flush Scenario

If one peer reconnects and emits a burst:

- replay status may show `backlog_flush`
- scheduling can remain `backlog_pressure`
- prioritize draining and scope verification before policy escalation

## Guardrails

- replay is not automatically a failure
- overlap is not automatically reject
- structural invalid payloads remain reject path
- OP-4 is baseline discipline, not final semantic conflict resolver
