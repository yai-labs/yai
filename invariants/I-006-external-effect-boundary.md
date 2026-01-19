
# I-006 — External Effect Boundary as a Structural Invariant

## Purpose

This document defines the **external effect boundary** as a **structural invariant**
in ICE.

ICE must distinguish between transitions whose effects are confined to ICE-controlled
state and transitions that produce **external or irreversible effects**.

The purpose of this boundary is to ensure that the points of highest risk and
highest consequence are subject to **stronger authority and evidence constraints**
and remain defensible under governance.

Without an explicit external effect boundary, ICE cannot remain governable
where it matters most.

---

## Definition

In ICE:

- An **internal transition** is a valid state transition whose effects remain confined
  to ICE-controlled state and are, in principle, isolatable or reversible within the
  execution model.
- An **external-effect transition** is a valid state transition that produces effects
  outside ICE-controlled state that are irreversible or not reliably reversible.

The **external effect boundary** is the canonical predicate that separates these
two classes of transitions.

ICE must be able to determine, at the conceptual level, whether a transition
crosses this boundary.

---

## Invariant Status

The external effect boundary is a **structural invariant** in ICE.

As such:

- It is **not optional**
- It is **not configurable**
- It is **not context-dependent**
- It applies uniformly across all ICE components

Any ICE system that cannot distinguish external-effect transitions, or that treats
external effects as internal, is **not a valid instance of ICE**.

---

## Boundary Consequences

If a transition crosses the external effect boundary, ICE requires:

- **Strengthened authority** (appropriate to scope and impact)
- **Augmented semantic evidence** (impact, scope, justification, target, consequence)
- **Abstract cost accountability** including risk attribution (I-005)
- **Non-bypassability**: no component may execute external effects outside the boundary

Internal transitions remain subject to all other ICE invariants, but do not require
the strengthened conditions above.

---

## Relationship to ICE Axioms

This invariant derives from ICE axioms:

- Execution causes consequences.
- Authority defines what may happen.
- State must remain inspectable as the result of authorized execution.

External effects are the highest-consequence form of execution and must be
explicitly constrained.

---

## Relationship to Other Invariants

### Boundary and Traceability (I-001)

Crossing the boundary requires augmented semantic evidence and attribution.
Without traceability, external effects cannot be governed or audited.

---

### Boundary and Governance (I-003)

Governance must apply differentiated constraints based on boundary classification.
If governance cannot treat external effects differently, governance collapses.

---

### Boundary and Abstract Cost Accountability (I-005)

External-effect transitions must be cost-accountable, including risk as an abstract
cost attribute. External effects without cost accountability are non-defensible.

---

## External Effect Boundary vs Other Concepts

### Boundary vs API Classification

This boundary is not a list of APIs or tools.
It is a semantic classification by consequence.

---

### Boundary vs Security Policies

Security policies define specific rules.
This invariant defines the conceptual requirement that external effects are treated
as requiring stronger constraints. Policies belong downstream.

---

## Scope Clarifications

This invariant defines **what must be true**, not **how it is implemented**.

This document does **not** define:

- mechanisms to detect external effects
- provider-specific effect classification
- policy engines, confirmation workflows, or UI patterns
- runtime enforcement architecture

Those concerns belong to downstream projects and must comply
with the invariant defined here.

---

## Consequences of Violation

If the external effect boundary is violated:

- high-impact actions may occur under weak constraints
- authority may be exercised out of scope
- evidence becomes insufficient for audit
- governance cannot defend responsibility

Such a system cannot be considered compliant with ICE,
regardless of correctness, performance, or intelligence.

---

## Canonical Status

This document is **canonical**.

All ICE runtimes, engines, governance layers, and interfaces must preserve
this invariant.

No downstream project may reinterpret, bypass, or ignore the external effect
boundary.
