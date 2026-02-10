# A-002 — Authority and Inference / Control Separation

This document defines authority as a foundational axiom of YAI and formalizes the strict separation between inference and control.

YAI rejects the assumption that authority can emerge implicitly from inference, optimization, intent, or confidence.
Authority MUST be explicit, stateful, and checkable.
Authority is conceptually prior to execution.

Any system in which inference alone is sufficient to cause execution is not a valid instance of YAI by definition.

## Authority as a Lawal Concept

In YAI, authority is the explicit constraint that determines which actions are permitted to be committed.

Authority is not emergent.
It is not inferred.
It is not optimized.
It is not assumed.

Authority exists as a first-class, system-level concept that constrains execution.

No component may possess authority by virtue of inference, confidence, or intelligence alone.

## Inference Versus Control

YAI draws a strict conceptual boundary between inference and control.

### Inference

Inference:

- interprets inputs
- evaluates conditions
- produces intent, recommendations, plans, or propositions

Inference is descriptive.
Inference may propose actions, but it cannot authorize them.

### Control

Control:

- determines whether an action is permitted to be committed
- applies constraints and policy
- authorizes or rejects execution

Control is normative and authoritative.
Control MUST be deterministic and auditable at the point of decision.

## Authority Boundaries

Inference and control MUST not share the same authority boundary.

The system MUST provide an explicit control boundary where authorization is decided and enforced.
Any design where control is implemented as an inference step is invalid by definition.

## Axiom Statement

In YAI:

- Inference and control are strictly separated.
- Authority MUST be explicit and checkable.
- If authority is not proven, execution is denied.
- No inference process may directly authorize execution.

Violating this separation invalidates the system conceptually.

## Authority, Execution, and State

Execution is always subject to authority.

Committed state transitions may only occur as a result of authorized execution.

This separation enforces:

- traceability
- responsibility
- governance
- prevention of unintended action

## Canonical Status

This document defines a canonical axiom of YAI.

Any conceptual model that allows inference to substitute authority is incompatible with YAI.

## Scope Notes

This document does not define:

- enforcement mechanisms
- policy engines
- governance processes
- runtime implementations
- tooling or configuration
