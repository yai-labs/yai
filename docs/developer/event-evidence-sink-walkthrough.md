# Event/Evidence Sink Walkthrough (DP-4)

This walkthrough shows the first canonical append baseline for governed runtime
actions in `yai`.

## Scenario
- Workspace family: `digital`
- Specialization: `remote-publication`
- Action: `digital.publish`
- Expected governance outcome: `review_required` (or stronger)

## Steps

1. Start runtime and bind workspace.
2. Execute a governed action (`yai ws run ...`).
3. Verify persisted records under:
   - `~/.yai/run/<ws>/events/runtime-events.v1.ndjson`
   - `~/.yai/run/<ws>/events/decision-records.v1.ndjson`
   - `~/.yai/run/<ws>/events/evidence-records.v1.ndjson`
   - `~/.yai/run/<ws>/events/index.v1.json`
4. Verify `yai ws policy effective` or `yai ws debug resolution` exposes
   `event_evidence_sink` refs.

## What is persisted

- Runtime event (`yai.runtime_event.v1`)
  with event/decision/evidence refs.
- Decision record (`yai.decision_record.v1`)
  from `decision_to_audit`.
- Evidence record (`yai.evidence_record.v1`)
  from `decision_to_evidence`.

## What this proves

- Event/evidence are no longer only terminal output.
- Workspace inspect/debug/effective surfaces can point to persisted sink refs.
- Runtime owns writes; CLI remains a reader through runtime contracts.
