# Boundary: Foundation → Runtime

## Purpose

Define the boundary between the **ICE Foundation** and the **ICE Runtime**.

This document establishes how the Foundation constrains Runtime **conceptually**,
and what it explicitly does **not** define, control, or implement.

The purpose is to preserve a strict separation between **conceptual authority**
and **execution mechanics**, preventing semantic leakage between layers.

---

## Role of the Foundation

The ICE Foundation:

- Defines **axioms** that establish what is assumed to be true
- Defines **structural invariants** that must never be violated
- Establishes **conceptual validity**, not operational behavior
- Constrains **what execution is allowed to mean**, not how it occurs

The Foundation is **pre-execution** and **non-operational**.

The Foundation does **not** execute.

---

## Role of the Runtime

The ICE Runtime:

- Executes actions, workflows, and state transitions
- Manages lifecycle, scheduling, and coordination
- Enforces invariants through concrete mechanisms
- Operates entirely within constraints defined by the Foundation

The Runtime is responsible for **how** things happen,
never for **what is true**.

---

## Explicit Non-Responsibilities of the Foundation

The Foundation does NOT define:

- Execution order or scheduling semantics
- Concurrency or parallelism models
- Error handling or recovery strategies
- Retry logic or failure management
- Performance characteristics
- Resource allocation or utilization
- Runtime optimization techniques

All of the above belong exclusively to Runtime design
or downstream execution domains.

---

## Constraint Relationship

- The Foundation **constrains** Runtime meaning and validity
- The Runtime **implements** behavior within those constraints
- Runtime behavior may fail operationally
- The Foundation may not fail conceptually

If Runtime behavior violates an axiom or invariant,
the Runtime is incorrect — not the Foundation.

---

## Invalid Boundary Violations

The following are invalid:

- Introducing execution semantics at the Foundation level
- Encoding lifecycle or scheduling rules as axioms or invariants
- Using Foundation documents to justify Runtime implementation choices
- Allowing Runtime to reinterpret or override axioms

Such actions constitute a **boundary violation**.

---

## Canonical Status

This boundary definition is **authoritative**.

Any Runtime implementation claiming ICE compliance
must demonstrate that it operates entirely within
the constraints defined here.

The Foundation constrains Runtime.  
Runtime never extends the Foundation.
