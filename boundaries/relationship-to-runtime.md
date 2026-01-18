# Boundary: Foundation ↔ Runtime

## Purpose

Define and formalize the **conceptual boundary** between the **ICE Foundation**
and the **ICE Runtime**.

This document establishes where **Foundation authority ends**
and where **Runtime responsibility begins**, in order to preserve
a strict separation between **conceptual validity** and **execution behavior**.

The goal is to prevent semantic leakage, implicit authority,
or retroactive justification of runtime behavior
through foundational concepts.

---

## Foundational Position

The ICE Foundation is **pre-execution** and **non-operational**.

It exists to define:

- Axioms: what is assumed to be true
- Structural invariants: what must never be violated
- Conceptual validity: what behavior is allowed to *mean*

The Foundation does **not** execute, schedule, enforce, or operate.

---

## Role of the Foundation

The ICE Foundation:

- Defines **truth**, not behavior
- Establishes **constraints**, not mechanisms
- Specifies **validity conditions**, not execution paths
- Remains **implementation-agnostic by design**

The Foundation constrains the **semantic space**
within which execution may occur.

---

## Role of the Runtime

The ICE Runtime:

- Executes actions and workflows
- Manages lifecycle, scheduling, and coordination
- Handles state transitions and enforcement mechanisms
- Operates under the constraints defined by the Foundation

The Runtime is responsible for **how execution occurs**,
never for **what is conceptually valid**.

---

## Explicit Non-Responsibilities of the Foundation

The ICE Foundation does **not** define, imply, or control:

- Execution order or scheduling semantics
- Concurrency or parallelism models
- Error handling or recovery strategies
- Retry logic or failure management
- Performance characteristics
- Resource allocation or utilization
- Runtime optimization techniques
- Enforcement implementations

Any attempt to introduce these concerns
into Foundation documents is invalid.

---

## Constraint Relationship

The relationship between Foundation and Runtime is asymmetric:

- The Foundation **constrains meaning and validity**
- The Runtime **implements behavior within those constraints**

Operational failure is a Runtime concern.  
Conceptual failure is a Foundation concern.

If Runtime behavior violates an axiom or invariant,
the Runtime is incorrect.
The Foundation remains authoritative and unchanged.

---

## Boundary Violations

The following constitute **boundary violations**:

- Introducing execution semantics at the Foundation level
- Encoding lifecycle or scheduling rules as axioms or invariants
- Using Foundation documents to justify Runtime design choices
- Allowing Runtime behavior to redefine or reinterpret axioms

Such violations invalidate ICE compliance.

---

## Canonical Status

This boundary definition is **canonical and authoritative**.

Any Runtime claiming ICE compliance must demonstrate
that it operates entirely within the constraints defined here.

The Foundation constrains the Runtime.  
The Runtime never extends the Foundation.