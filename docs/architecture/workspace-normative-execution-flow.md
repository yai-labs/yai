# Workspace Normative Execution Flow (WS-5)

This document defines the first real workspace-driven normative execution path.

## Flow

1. Set workspace binding (`yai.workspace.set` / `yai.workspace.switch`).
2. Declare context hint (`yai.workspace.domain_set --family economic --specialization payments`).
3. Execute action from workspace context (`yai.workspace.run payment.authorize ...`).
4. Runtime resolves through law stack:
   - classification
   - family/specialization discovery (declared hint is strong, non-dogmatic)
   - overlay attachment
   - authority/evidence aggregation
   - final effect mapping
5. Resolver snapshot is persisted into workspace runtime manifest.
6. Post-action inspection surfaces expose the result:
   - `yai.workspace.inspect`
   - `yai.workspace.policy_effective`
   - `yai.workspace.debug_resolution`

## Runtime handoff rules

- `declared` context is injected into runtime payload as `workspace_declared_family` and `workspace_declared_specialization` when not already present.
- Resolver still classifies input action/provider/resource; declared fields bias selection but do not bypass classification.
- Effective decision is emitted as normal runtime decision payload and mirrored into workspace summaries.

## Expected post-action outputs

- decision/effect (`allow | review_required | quarantine | deny`)
- authority summary (`stack.authority_profile`)
- evidence summary (`stack.evidence_profile`)
- effective overlays summary (`effective_overlays_ref`)
- trace reference (`last_resolution_trace_ref`)

## Primary pilot scenario in WS-5

- Control family: `economic`
- Specialization: `payments`
- Action: `payment.authorize`
- Expected behavior: workspace context drives a coherent economic/payments resolution and updates inspect/policy/debug surfaces after execution.
