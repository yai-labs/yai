# Digital Runtime Resolution Model

This document defines the digital runtime path used by workspace-driven execution in YAI.

## Scope

The digital family governs:
- network egress
- remote publication
- external commentary
- artifact distribution
- remote retrieval
- digital sink control

## Runtime Path

1. `cli` issues a workspace command.
2. `sdk` forwards control-call payload without duplicating runtime logic.
3. `yai` classifies action/provider/resource and reads workspace declared context.
4. digital specialization is selected.
5. resolver builds effective stack + overlays and computes effect.
6. runtime snapshot stores effect/authority/evidence summaries into workspace-owned state.
7. inspect/policy/debug surfaces expose digital summaries.

## Digital specialization routing

The resolver routes toward digital specializations using command and context signals:
- `external-commentary` when commentary intent is detected.
- `artifact-distribution` when distribution/delivery intent is detected.
- `remote-retrieval` when retrieval/fetch intent is detected.
- `digital-sink-control` when sink/destination governance intent is detected.
- `remote-publication` when publication/export intent is detected.
- `network-egress` as default digital outbound specialization.

## Effect semantics baseline

- `network-egress`
  - external sink without authority contract -> `deny`
  - external sink with authority context -> `review_required`
- `remote-publication`
  - missing authority contract -> `deny`
  - untrusted external sink -> `quarantine`
  - controlled sink with authority -> `review_required`
- `external-commentary`
  - external commentary without authority contract -> `deny`
  - otherwise -> `review_required`
- `artifact-distribution`
  - missing artifact or sink reference -> `deny`
  - missing authority or untrusted sink -> `quarantine`
  - complete governed path -> `review_required`
- `remote-retrieval`
  - trusted retrieval path -> `allow/review_required`
  - untrusted external retrieval path -> `review_required`
- `digital-sink-control`
  - missing sink reference -> `deny`
  - untrusted external sink -> `quarantine`
  - controlled sink -> `review_required`

## Digital summary surfaces

The workspace surfaces expose a dedicated `digital` object with:
- `outbound_context_summary`
- `sink_target_summary`
- `publication_control_summary`
- `retrieval_control_summary`
- `distribution_control_summary`

These are available in:
- `yai.workspace.inspect`
- `yai.workspace.policy_effective`
- `yai.workspace.debug_resolution`
- `yai.workspace.run` reply data
