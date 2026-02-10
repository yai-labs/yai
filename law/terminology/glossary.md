# YAI Law — Glossary

This glossary defines the **controlled terminology** of the YAI Law.

Its purpose is to prevent semantic drift, ambiguity, and reinterpretation
of foundational concepts across the YAI ecosystem.

All terms defined here are **authoritative** within the scope of the Law.
Downstream projects may extend vocabulary, but must not redefine these terms.

---

## Purpose

The glossary exists to:

- ensure consistent meaning across repositories and projects
- prevent implicit redefinition of foundational concepts
- provide a shared semantic reference for reasoning and documentation
- separate *terminology* from *implementation language*

The Law governs meaning.
Implementations consume meaning.

---

## Scope

This glossary includes:

- core conceptual terms used in axioms, invariants, and boundaries
- terms that define authority, execution, state, and governance
- epistemic distinctions critical to YAI correctness

It intentionally excludes:

- implementation-specific terms
- technology or framework names
- runtime, tooling, or API vocabulary
- domain-specific extensions

---

## Canonical Terms

### Axiom

A foundational assumption taken as true by definition and not derived from
lower-level mechanisms.

Axioms constrain what YAI is allowed to mean.
They are non-configurable, non-derivable, and authoritative.

---

### Structural Invariant

A non-negotiable constraint that must always hold for a system to be considered
a valid instance of YAI.

Structural invariants derive authority from axioms and make them enforceable
over time.

---

### Law

The conceptual layer of YAI that defines axioms, structural invariants,
epistemic boundaries, and authoritative meaning.

The Law defines meaning and authority, not behavior.

---

### Execution

The act of causing state transitions within the system.

Execution implies consequence and accountability.

---

### Runtime

The domain responsible for executing actions, managing lifecycle, and enforcing
constraints during operation.

The Runtime implements behavior within limits defined by the Law.

---

### Intent

A proposed, declarative request for execution (command candidates).

Intent is not authority and cannot cause execution without control.

---

### Authority

The property by which actions, decisions, or state transitions are considered
valid and enforceable within YAI.

Authority must be explicit, traceable, and never inferred implicitly.

---

### Inference

The process of deriving conclusions, intentions, or proposals from information.

Inference may inform decisions but never grants authority to act.

---

### Control

The enforcement of authority over execution.

Control determines what is allowed to happen, independent of inference quality.

---

### State

A derived, inspectable artifact representing the outcome of execution.

State is not a cause, configuration, or source of authority.

---

### Vault

A bounded, shared-memory execution surface used for inter-component coordination.

The Vault is the canonical L0 contract surface.

---

### Internal Transition

A valid transition whose effects remain confined to YAI-controlled state.

---

### External-Effect Transition

A valid transition that produces effects outside YAI-controlled state that are irreversible or not reliably reversible.

---

### External Effect Boundary

The predicate separating internal transitions from transitions that produce irreversible or non-YAI-controlled effects.

---

### Abstract Cost Accountability

The requirement that every valid transition is attachable to abstract cost attributes within a declared metric space.

---

### Cognitive Adaptability

The foundational capability by which a YAI system explicitly suspends execution
and reorganizes its cognitive configuration when observed reality invalidates current assumptions.

Cognitive adaptability does not imply learning, optimization, or autonomy.

---

### Cognitive Configuration

The explicit set of assumptions, constraints, priorities, goals, and contextual validity
that constitutes the cognitive basis under which execution is allowed to proceed.

Cognitive configuration is not “State” as defined in this glossary.

---

### Cognitive Validity / Invalidation

Cognitive validity is the condition under which a cognitive configuration remains consistent
with observed reality and therefore may support execution.

Invalidation is the explicit detection that the cognitive configuration no longer supports valid execution.

---

### Cognitive Reconfiguration

An explicit, authority-bound transition from one cognitive configuration to another
performed in response to invalidation.

Reconfiguration constrains whether execution may continue; it does not define how adaptation is implemented.

---

### Reconfiguration Record

The canonical inspectable artifact representing a cognitive reconfiguration transition.

A Reconfiguration Record must be traceable and authority-referenced, and must not be conflated with “State”.

---

### Traceability

The property by which actions, decisions, and state transitions can be
attributed, reconstructed, and reasoned about after execution.

Traceability is a structural invariant.

---

### Determinism

The property by which system behavior, given the same conditions and constraints,
produces equivalent outcomes within a defined scope.

Determinism enables reconstruction, not prediction.

---

### Reproducibility

The ability to re-execute or reconstruct system behavior and obtain
behaviorally equivalent results.

Reproducibility is required for governance and accountability.

---

### Governance

The invariant by which authority, responsibility, and control remain enforceable
over time, across system evolution.

Governance is structural, not procedural.

---

### Mind

The domain responsible for intent formation, routing, and orchestration under L3 constraints.

Mind proposes; it does not execute.

---

### Consciousness

The domain responsible for long-term memory, historical continuity,
and retrospective reasoning.

Consciousness may inform inference but does not grant authority.

---

### Boundary

An explicit conceptual separation defining what the Law does and does not
govern with respect to other domains.

Boundaries prevent semantic overreach.

---

## Terminology Governance

- Terms defined here may not be redefined downstream
- New foundational terms require explicit inclusion
- Ambiguous or overloaded terms must be clarified or rejected

Terminology changes are **conceptual changes**.

---

## Final Note

Language shapes architecture.

A stable system requires stable meaning.
This glossary exists to ensure YAI remains coherent as it evolves.
