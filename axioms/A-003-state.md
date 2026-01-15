# State as a Derived and Inspectable Artifact

This document defines the role of state in ICE.

In ICE, state is not a primitive, a cause, or a driver of behavior.
State is the observable result of authorized execution.

Any system in which state implicitly causes execution,
or exists independently of traceable actions,
is not a valid instance of ICE.

---

## Definition of State in ICE

In ICE, **state** is defined as:

- A derived artifact of execution
- The result of authorized control acting on the system
- An observable and inspectable representation of what has occurred

State does not exist prior to execution.
State does not initiate execution.
State does not grant authority.

---

## State and Execution

Execution precedes state.

Every state transition in ICE must be attributable to a specific execution event
that was explicitly authorized.

There is no valid state change without execution.
There is no execution without authority.
There is no authority inferred from state.

---

## State Versus Memory

State is not memory.

- Memory stores information.
- State represents the current condition resulting from execution.

Memory may contribute to inference.
Memory may inform decisions.
Memory does not define state.

State reflects what has happened, not what is remembered.

---

## State Versus Configuration

State is not configuration.

- Configuration defines potential behavior.
- State reflects realized behavior.

Configuration exists before execution.
State exists after execution.

Treating configuration as state, or state as configuration,
introduces ambiguity and violates ICE axioms.

---

## Inspectability Requirement

All state in ICE must be inspectable.

Inspectability means:

- State transitions can be observed
- State origins can be traced
- State changes can be attributed to execution and authority

Uninspectable state is invalid state.

If a state cannot be inspected, it cannot be governed,
and therefore cannot exist in ICE.

---

## State and Responsibility

Because state is derived from authorized execution,
state is inherently tied to responsibility.

Every state implies:

- An execution occurred
- Authority permitted it
- Accountability exists

State without responsibility is not permitted.

---

## Relationship to Other Axioms

This axiom derives from and reinforces:

- **Execution as a First Principle**  
  State cannot exist without execution.

- **Explicit Authority and Inference / Control Separation**  
  State changes must be authorized, not inferred.

State acts as the observable surface of execution under authority.

---

## Invalid Patterns

The following patterns are explicitly invalid in ICE:

- State driving execution
- Implicit state-based control
- Hidden or uninspectable state
- State changes without traceable execution
- State treated as memory or configuration

---

## Canonical Status

This document is canonical.

All ICE components must be able to trace their state model back to this axiom.

Any deviation invalidates the system as a coherent instance of ICE.
