# Final Governed Workspace Demo and Test Matrix (14/14)

This matrix converges workspace refoundation, containment, security envelope, execution hooks, hostile-path baseline, and scientific/digital verticals into runnable end-to-end scenarios.

## Canonical Chain

All scenarios are validated through:

`operator -> cli -> sdk -> yai`

No scenario should rely on CLI-side runtime policy logic duplication.

## Scenario Taxonomy

- `LIFECYCLE`: workspace create/set/switch/unset/clear/destroy and attach sanity.
- `POSITIVE`: governed execution paths with allow/review + evidence.
- `HOSTILE`: stale binding, invalid context, cross-scope misuse, degraded mode, denial paths.
- `SCI`: scientific vertical scenarios.
- `DIG`: digital vertical scenarios.
- `SURFACE`: inspect/status/policy/debug/render coherence and contract visibility.

## Final Matrix

| Id | Class | Scenario | Primary surfaces | Expected outcome |
| --- | --- | --- | --- | --- |
| M14-L1 | LIFECYCLE | Create workspace, set binding, inspect/status/current | `ws create/set/current/status/inspect` | Active binding + valid root + stable token |
| M14-L2 | LIFECYCLE | Switch workspace and preserve isolation | `ws switch/current/inspect` | Bound workspace changes, state remains scoped |
| M14-L3 | LIFECYCLE | Unset binding and verify empty state | `ws unset/current/status` | `no_active` + prompt token hidden |
| M14-L4 | LIFECYCLE | Clear runtime-local state while bound | `ws clear/inspect/policy` | Binding remains active, runtime-local summaries reset |
| M14-S1 | SCI+POSITIVE | Scientific parameter mutation without lock | `ws domain set` + `ws run` + `policy/debug` | `deny` with parameter lock evidence obligations |
| M14-S2 | SCI+POSITIVE | Scientific publication with full reproducibility context | `ws run` + `inspect/policy/debug` | `review_required` or controlled `allow` |
| M14-S3 | SCI+HOSTILE | Scientific publication missing authority/repro context | `ws run` | `deny` or `quarantine` with explicit rationale |
| M14-D1 | DIG+POSITIVE | Digital retrieve on trusted path | `ws domain set` + `ws run` + `policy/debug` | `allow/review_required` with retrieval attestation |
| M14-D2 | DIG+POSITIVE | Digital publication to trusted sink with contract | `ws run` + `inspect/policy/debug` | `review_required` (or controlled `allow`) |
| M14-D3 | DIG+HOSTILE | Digital publication without contract or untrusted sink | `ws run` + `policy/debug` | `deny` or `quarantine` with sink rationale |
| M14-D4 | DIG+HOSTILE | Artifact distribution missing sink/artifact refs | `ws run` | `deny` with manifest/scope rationale |
| M14-H1 | HOSTILE | Run without active workspace | `ws unset` + `ws run` | clear refusal (`workspace_not_active`) |
| M14-H2 | HOSTILE | Degraded execution mode visibility | `ws inspect/status/policy/debug` | requested/effective/degraded fields coherent |
| M14-H3 | HOSTILE | Cross-workspace boundary check baseline | hostile baseline script | no accidental cross-workspace leakage |
| M14-C1 | SURFACE | Inspect/status/policy/debug consistency | workspace surface scripts | declared/inferred/effective + evidence coherent |
| M14-C2 | SURFACE | Prompt token semantics | prompt token script | token reflects active binding only |

## Automated Packs

- Lifecycle and surfaces:
  - `tests/integration/workspace_lifecycle/workspace_session_binding_contract_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_inspect_surfaces_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_real_flow_v1.sh`
- Verticals:
  - `tests/integration/workspace_lifecycle/workspace_scientific_flow_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_digital_flow_v1.sh`
- Hostile/boundary:
  - `tests/integration/workspace_lifecycle/workspace_hostile_path_baseline_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_isolation_guards_v1.sh`
  - `tests/integration/workspace_lifecycle/workspace_negative_paths_v1.sh`

## Current Honest Limitations

This matrix demonstrates governed workspace execution and boundary-aware runtime behavior, but not full OS-level sandboxing, full distributed orchestration, or complete low-level resource/network isolation.
