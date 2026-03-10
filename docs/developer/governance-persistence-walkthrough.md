# Governance Persistence Walkthrough (DP-5)

## Goal
Show governance custom as a live Data Plane entity (not only artifact files).

## Scenario
1. Workspace `digital/remote-publication`.
2. Attach approved governance object
   `enterprise.ecohmedia.digital-outbound.review-gate`.
3. Execute `digital.publish` to produce decision/evidence trail.
4. Inspect persisted governance domain.

## Persisted artifacts (workspace-scoped)
- `~/.yai/run/<ws>/governance/object-state.v1.ndjson`
- `~/.yai/run/<ws>/governance/lifecycle-state.v1.ndjson`
- `~/.yai/run/<ws>/governance/attachment-state.v1.ndjson`
- `~/.yai/run/<ws>/governance/index.v1.json`

## What to verify
- object/lifecycle/attachment records exist and are append-updated by runtime.
- `index.v1.json` exposes latest refs.
- attachment records include `event_ref`, `decision_ref`, `evidence_ref` after
  governed execution.
- `yai ws inspect`, `yai ws policy effective`, `yai ws debug resolution` expose
  `governance_persistence` refs.

## Why it matters
Governance is now persisted as operational state in Data Plane, while `law`
remains normative source of schemas/rules.
