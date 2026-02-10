
# I-006 — External Effect Boundary as a Structural Invariant

## Purpose

This document defines the **external effect boundary** as a **structural invariant**
in YAI.

YAI must distinguish between transitions whose effects are confined to YAI-controlled
state and transitions that produce **external or irreversible effects**.

The purpose of this boundary is to ensure that the points of highest risk and
highest consequence are subject to **stronger authority and evidence constraints**
and remain defensible under governance.

Without an explicit external effect boundary, YAI cannot remain governable
where it matters most.

---

## Definition

In YAI:

- An **internal transition** is a valid state transition whose effects remain confined
  to YAI-controlled state and are, in principle, isolatable or reversible within the
  execution model.
- An **external-effect transition** is a valid state transition that produces effects
  outside YAI-controlled state that are irreversible or not reliably reversible.

The **external effect boundary** is the canonical predicate that separates these
two classes of transitions.

YAI must be able to determine, at the conceptual level, whether a transition
crosses this boundary.

---

## Canonical Boundary Predicate

In YAI, the boundary is defined by a canonical predicate:

**ExternalEffect(t)** → {true, false}

Where `t` is a candidate transition.

Requirements:

- The classification must be **determinable before execution** of the effect.
- The predicate must be derived from **declared effect surfaces** (e.g. providers,
  OS calls, network, filesystem, actuators, remote APIs).
- The predicate is **semantic by consequence**, not an API list.

If a transition cannot be classified at decision time, it cannot be treated as valid.

---

## Invariant Status

The external effect boundary is a **structural invariant** in YAI.

As such:

- It is **not optional**
- The requirement is **non-bypassable**
- The boundary predicate MUST be **explicitly declared and auditable** for each
  execution surface that can produce effects
- It applies uniformly across all YAI components

Any YAI system that cannot distinguish external-effect transitions, or that treats
external effects as internal, is **not a valid instance of YAI**.

---

## Boundary Consequences

If a transition crosses the external effect boundary, YAI requires:

- **Strengthened authority** (appropriate to scope and impact)
- **Augmented semantic evidence** with a minimum set of fields:
  - target identity
  - effect class
  - irreversibility justification
  - authority reference
  - declared intent / purpose
  - risk attribution (I-005)
  - mitigation or rollback note (may be “none”)
- **Abstract cost accountability** including risk attribution (I-005)
- **Non-bypassability**: no component may execute external effects outside the boundary

Internal transitions remain subject to all other YAI invariants, but do not require
the strengthened conditions above.

---

## Relationship to YAI Axioms

This invariant derives from YAI axioms:

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

## Invalid Patterns

The following patterns are **invalid** in YAI:

- external effects treated as internal transitions
- classification made post-hoc (after execution)
- effect surfaces not declared or not auditable
- external-effect transitions without risk attribution (I-005)
- bypass via side-channels (plugins, shell access, filesystem writes, or provider calls outside the boundary predicate)

---

## Consequences of Violation

If the external effect boundary is violated:

- high-impact actions may occur under weak constraints
- authority may be exercised out of scope
- evidence becomes insufficient for audit
- governance cannot defend responsibility

Such a system cannot be considered compliant with YAI,
regardless of correctness, performance, or intelligence.

---

## Canonical Status

This document is **canonical**.

All YAI runtimes, engines, governance layers, and interfaces must preserve
this invariant.

No downstream project may reinterpret, bypass, or ignore the external effect
boundary.
