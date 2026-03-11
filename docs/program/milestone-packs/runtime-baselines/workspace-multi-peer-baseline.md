---
id: RB-WORKSPACE-MULTI-PEER-BASELINE
status: active
owner: runtime
effective_date: 2026-03-11
revision: 1
---
# Workspace Multi-Peer Baseline

## Purpose

Provide a concrete operator baseline for one canonical workspace fed by
multiple peers.

## Example scenario: Comune di Bologna (baseline)

Workspace:

- `comune-bologna-piao-2026`

Peers:

- peer A: Ufficio Performance (`peer_role=performance`)
- peer B: Ufficio Programmazione (`peer_role=budget`)
- peer C: Segreteria Documentale (`peer_role=documental`)

Expected coordination shape:

- one workspace
- three workspace peer memberships
- each peer with its own bindings and coverage
- owner-side summary shows per-peer health/backlog/coverage signals

## Baseline procedure

1. Enroll each peer against owner.
2. Attach each peer to the same workspace with role/scope metadata.
3. Start peer status updates (health + backlog counters).
4. Verify source query summary includes:
- source node count
- binding count
- workspace peer membership count
5. Validate degraded/disconnected peer state does not erase workspace truth but
appears in peer orchestration surfaces.

## Operational checks

- at least one membership record per attached peer
- role/scope values are explicit, not inferred by naming hacks
- coverage refs are present
- overlap states are present (`distinct`, `overlap_possible`, ...)
- backlog counters are visible per peer update

## Notes

- This runbook validates OP-1 coordination semantics.
- It is not full scheduler/conflict-resolution validation.
- Later waves (OP-2/OP-4) extend operational enforcement.
