# Binding — Control Plane (Core Authority Surface)

## Scope

Authority and control-plane semantics for the sovereign runtime plane (`core`).

## Canonical sources

Law:
- `foundation/axioms/A-002-authority.md`
- `foundation/invariants/I-003-governance.md`
- `foundation/invariants/I-006-external-effect-boundary.md`
- `foundation/invariants/I-007-compliance-context-required.md`

Schemas:
- `lib/protocol/contracts/schema/control/control_plane.v1.json`
- `lib/protocol/contracts/schema/control/control_call.v1.json`
- `lib/protocol/contracts/schema/control/exec_reply.v1.json`
- `lib/protocol/contracts/schema/control/authority.v1.json`

## Obligations

- Authority decisions are `core` decisions.
- Effectful operations require explicit governance and compliance context.
- Replies must be deterministic for same input and declared context.

## Compatibility

Legacy plane aliases (`ingress`, `root`, `kernel`, `engine`, `mind`) remain accepted where schema declares them, but are secondary to `core/exec/brain` ontology.
