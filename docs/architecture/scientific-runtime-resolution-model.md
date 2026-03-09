# Scientific Runtime Resolution Model

This document defines the scientific runtime path used by workspace-driven execution in YAI.

## Scope

The scientific family governs:
- experiment configuration
- parameter governance
- reproducibility controls
- dataset integrity
- result publication control

## Runtime Path

1. `cli` issues a workspace command.
2. `sdk` forwards control-call payload without duplicating runtime logic.
3. `yai` classifies action/provider/resource and reads workspace declared context.
4. scientific specialization is selected.
5. resolver builds effective stack + overlays and computes effect.
6. runtime snapshot stores effect/authority/evidence summaries into workspace-owned state.
7. inspect/policy/debug surfaces expose scientific summaries.

## Scientific specialization routing

The resolver routes toward scientific specializations using command and context signals:
- `black-box-evaluation` when black-box intent is detected.
- `result-publication-control` when publication/export intent is detected.
- `reproducibility-control` when lineage/proofpack/repro signals are present.
- `dataset-integrity` when dataset integrity operations are detected.
- `experiment-configuration` for explicit configuration/setup operations.
- `parameter-governance` as default governed specialization.

## Effect semantics baseline

- `parameter-governance`
  - missing parameter lock -> `deny`
  - lock present -> `review_required`
- `reproducibility-control`
  - missing reproducibility context -> `deny`
  - context present -> `review_required`
- `dataset-integrity`
  - missing dataset reference -> `deny`
  - integrity flow present -> `review_required`
- `result-publication-control`
  - missing authority contract -> `deny`
  - missing reproducibility/result evidence -> `quarantine`
  - complete context -> `review_required`

## Scientific summary surfaces

The workspace surfaces expose a dedicated `scientific` object with:
- `experiment_context_summary`
- `parameter_governance_summary`
- `reproducibility_summary`
- `dataset_integrity_summary`
- `publication_control_summary`

These are available in:
- `yai.workspace.inspect`
- `yai.workspace.policy_effective`
- `yai.workspace.debug_resolution`
- `yai.workspace.run` reply data
