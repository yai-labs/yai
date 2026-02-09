# Authority and Inference / Control Separation

This document defines authority as a foundational axiom of ICE and formalizes the strict separation between inference and control.

ICE rejects the assumption that authority can emerge implicitly from inference, optimization, or intent.
Authority must be explicit, constrained, and conceptually prior to execution.

Any system in which inference alone is sufficient to cause execution is not a valid instance of ICE by definition.

---

## Authority as a Foundational Concept

In ICE, **authority** is the explicit conceptual constraint that determines which actions are permitted to occur.

Authority is not emergent.
It is not inferred.
It is not optimized.
It is not assumed.

Authority exists as a first-class, system-level concept that constrains execution.

No component may possess authority by virtue of inference, confidence, or intelligence alone.

---

## Inference Versus Control

ICE draws a strict conceptual boundary between **inference** and **control**.

### Inference

Inference:

- interprets inputs
- evaluates conditions
- produces intent, recommendations, or propositions

Inference is descriptive.
Inference does not authorize action.

---

### Control

Control:

- determines whether an action is permitted
- enforces constraints
- authorizes or rejects execution

Control is normative and authoritative.

Only control may permit execution.

---

## Axiom Statement

In ICE:

> Inference and control are strictly separated.  
> Authority must be explicit.  
> No inference process may directly authorize execution.

Violating this separation invalidates the system conceptually.

---

## Authority, Execution, and State

Execution is always subject to authority.

State transitions may only occur as a result of authorized execution.

This ensures:

- traceability
- responsibility
- governance
- prevention of unintended action

---

## Canonical Status

This document defines a canonical axiom of ICE.

Any conceptual model that allows inference to substitute authority is incompatible with ICE.

---

## Scope Notes

This document does not define:

- enforcement mechanisms
- policy engines
- governance processes
- runtime implementations
- tooling or configuration
