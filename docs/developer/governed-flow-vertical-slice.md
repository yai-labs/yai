# Governed Flow Vertical Slice (Consegna 7)

Closeout anchor:

- `docs/developer/pre-pilot-governed-workspace-baseline.md`

## Scenario

Canonical slice: **Digital outbound/publication governed flow**.

Goal: show one repeatable end-to-end path where explicit governance attachment, workspace context, runtime resolution, and inspectability are visible together.

## Actors and chain

- Operator: issues commands from CLI surface.
- SDK: mediation layer for command shaping and runtime call contracts.
- YAI runtime: sovereign resolution/enforcement and workspace state.
- LAW artifacts: source of governable objects and normative stack.

Canonical chain remains: `cli -> sdk -> yai`.

## Governable object used

This slice uses explicit workspace attachment:

- `customer.default.org-workspace-contextual-review`

The workspace is not treated as policy-default-only; attachment is explicit and verifiable in inspect/policy/debug operational state.

## End-to-end flow

1. Start runtime.
2. Create and bind workspace.
3. Attach explicit governance object.
4. Declare digital context (`remote-publication`).
5. Run event A (untrusted sink) to trigger deny/quarantine path.
6. Run event B (trusted sink + contract) to trigger review/allow path.
7. Inspect:
   - `workspace.inspect`
   - `workspace.policy.effective`
   - `workspace.debug_resolution`
8. Verify event surface + operational state coherence.

## What to verify in output

- Attachment is visible and active in operational state.
- Declared scenario remains visible (`remote-publication`).
- Business specialization is preserved even if enforcement gate is generic.
- Effective stack, effect, authority/evidence, trace refs are all exposed.
- Review state and operational summary are readable.

## Runnable script

Primary script:

- `tests/integration/workspace/workspace_governed_vertical_slice.sh`

Quick run:

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
make -j4 yai
make test-vertical-slice
```

## CLI-facing walkthrough (operator shorthand)

When running from `cli` repo, equivalent operator-level sequence is:

1. `yai up`
2. `yai ws create <ws>`
3. `yai ws set <ws>`
4. `yai ws policy attach customer.default.org-workspace-contextual-review`
5. `yai ws domain set --family digital --specialization remote-publication`
6. `yai ws run digital.publish sink=external_untrusted artifact=bundle-v1`
7. `yai ws run digital.publish sink=internal_trusted contract=approved destination=ops_portal artifact=bundle-v1`
8. `yai ws inspect`
9. `yai ws policy effective`
10. `yai ws debug resolution`

## What this proves

- Explicit governance object attachment is operational.
- Workspace-scoped governed execution is real and repeatable.
- Event semantics remain readable (declared/business/enforcement split).
- Operational state is inspectable (last event, stack, effect, review, trace).

## What this does not prove

- Full pilot workflow lifecycle.
- Full enterprise review queue/approval engine.
- DB-backed dataplane or graph-backed lineage.
- Full multi-tenant production governance operations.

## Residual gaps tracked

- Attachability constraints remain partially descriptive until full runtime attach-policy enforcement matures.
- Conflict engine (`conflicts_with`) remains baseline-level, not advanced semantic arbitration.
- Full CLI/SDK UX convergence for all operational-state fields needs final refinement in their repos.
