# Event Classification

## Purpose

Classification normalizes runtime inputs before discovery and resolution.

## Components

- `event_classifier.c`: builds base context from request payload
- `action_classifier.c`: maps operation to canonical action classes
- `provider_classifier.c`: extracts provider hints
- `resource_classifier.c`: extracts resource hints
- `protocol_classifier.c`: derives protocol class
- `workspace_context.c`: derives workspace-mode and authority hints

## Wave-2 coverage additions

Economic action classes:
- `payment.authorize`
- `transfer.authorize`
- `settlement.finalize`

Overlay-sensitive hints:
- provider trust hints (`untrusted`, `unknown`)
- personal-data publication hints
- high-risk scientific hints

## Output contract

Classifier emits a deterministic context consumed by discovery/resolver.
Classifier remains side-effect free and does not apply policy decisions directly.

Runtime genericity note:
- classifier output feeds family ranking and specialization candidate selection.
- direct pilot-id jumps are intentionally reduced in favor of family/specialization routing.

## Event surface contract (workspace)

Classifier/discovery output is mapped into workspace event-surface fields:

- declared scenario specialization
- business specialization
- enforcement specialization
- flow stage

This mapping is consumed by:

- `yai.workspace.run` response
- `yai.workspace.policy.effective`
- `yai.workspace.debug.resolution`
- `yai.workspace.inspect`

Design goal: preserve business scenario semantics even when enforcement specialization is a generic technical gate (for example `network-egress`).
