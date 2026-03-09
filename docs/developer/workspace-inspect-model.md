# Workspace Inspect Model

## Goal

Define the minimal inspect/debug surface for a workspace before full CLI UX rollout.

This document is the developer contract for future commands such as:

- `yai ws current`
- `yai ws inspect`
- `yai ws status`
- `yai ws domain get`
- `yai ws policy effective`

## Canonical inspect payload

Source shape: `yai_workspace_inspect_v1_t` (`include/yai/core/workspace.h`).

### Identity

- `workspace_id`
- `workspace_alias`
- `workspace_root`

### State and binding

- `workspace_state`
- `session_binding`
- `runtime_attached`
- `runtime_endpoint`
- `control_plane_attached`

### Context sections

- Declared:
  - `declared_control_family`
  - `declared_specialization`
  - `declared_profile`
  - `declared_context_source`
- Inferred:
  - `last_inferred_family`
  - `last_inferred_specialization`
  - `last_inference_confidence`
- Effective:
  - `effective_stack_ref`
  - `effective_overlays_ref`
  - `last_effect_summary`
  - `last_authority_summary`
  - `last_evidence_summary`

### Runtime flags

- `isolation_mode`
- `debug_mode`
- `last_resolution_trace_ref`

### Summary

- `last_resolution_summary`

## Persistence vs runtime ownership

Persisted (manifest-driven):

- identity
- lifecycle timestamps/state
- declared/inferred/effective summaries
- runtime flags needed for inspection

Runtime-transient:

- in-memory session slot data
- process-local locks and ephemeral execution handles

## Manifest contract

Current workspace manifest path:

- `~/.yai/run/<workspace_id>/manifest.json`

Manifest now includes both compatibility root fields and canonical sections.

Compatibility root fields keep existing tooling stable:

- `ws_id`, `state`, `root_path`, `layout`, `created_at`, `updated_at`

Canonical sections support future inspect/status commands without schema breaks.

## Resolution pipeline alignment

Workspace inspect model aligns with runtime law flow:

1. classification
2. family selection
3. specialization selection
4. overlays attachment
5. authority/evidence composition
6. effective decision

Workspace stores declared/inferred/effective checkpoints so resolution output remains inspectable outside raw traces.
