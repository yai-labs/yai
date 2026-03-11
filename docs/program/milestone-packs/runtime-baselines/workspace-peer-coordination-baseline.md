---
id: RB-WORKSPACE-PEER-COORDINATION-BASELINE
status: active
owner: runtime
effective_date: 2026-03-11
revision: 1
---
# Workspace Peer Coordination Baseline

## Purpose

Operational baseline for reading owner-side peer coordination state in a
multi-peer workspace.

## Procedure

1. Ensure peers are enrolled and attached to the same workspace.
2. Collect source-plane status updates from each peer.
3. Query workspace source summary (`yai.workspace.query source`).
4. Read `coordination` block:
- `peer_count`
- `states` (`ready`, `degraded`, `disconnected`, `stale`)
- `backlog`
- `scheduling_state`
- peer rows

## Interpretation baseline

- `nominal`: peers active and no critical backlog/failure pressure.
- `backlog_pressure`: retry/stale pressure is building.
- `attention_required`: disconnected peers or failed backlog signals present.

## Bologna-style scenario baseline

Workspace `comune-bologna-piao-2026` with peers:

- Performance office peer
- Programmazione office peer
- Documentale office peer

Operator checks:

- each peer appears in `coordination.peers`
- role/scope/coverage values are explicit
- stale/disconnected peers are visible
- backlog pressure is visible without reading raw records

## Limits

- No full scheduler control actions in OP-2.
- No full conflict resolution in OP-2.
- Registry is owner-side baseline for OP-3 and OP-4.
