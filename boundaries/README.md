# ICE Foundation Boundaries

This directory defines the **canonical boundaries** between the ICE Foundation
and all downstream ICE domains.

Boundaries formalize **where Foundation authority ends**
and **where downstream responsibility begins**.

They exist to prevent scope creep, semantic leakage,
and implicit extension of foundational authority.

---

## What a Boundary Is in ICE

In the context of ICE, a boundary is:

- A formal separation of **conceptual authority**
- A definition of **non-responsibility**
- A guardrail against semantic overreach
- A constraint on how downstream systems may interpret Foundation concepts

A boundary is **not** an integration guide,
an implementation contract, or a technical interface.

---

## Role of Boundaries

Boundaries serve to:

- Protect the Foundation from operational concerns
- Prevent downstream domains from redefining axioms or invariants
- Make responsibility shifts explicit
- Ensure long-term architectural stability

Without explicit boundaries, foundational systems tend to
absorb responsibilities they were never meant to carry.

---

## Canonical Foundation Boundaries

The following boundary documents are authoritative:

- **Foundation → Runtime**  
  Execution, lifecycle, and enforcement

- **Foundation → Engine / Infrastructure**  
  System construction and operation

- **Foundation → AI / Intelligence**  
  Reasoning, inference, and adaptation

- **Foundation → Consciousness**  
  Long-term memory, causality, and decision continuity

- **Foundation → Protocols**  
  Communication and interaction semantics

- **Foundation → Providers**  
  External services, platforms, and dependencies

- **Foundation → Observability**  
  Measurement, inspection, and system visibility

- **Foundation → Documentation**  
  Epistemic authority versus explanatory content

Each boundary is defined in its own document
and must remain consistent with ICE axioms and structural invariants.

---

## Canonical Status

Boundary definitions are **authoritative**.

Any ICE domain claiming compliance
must demonstrate that it operates entirely
within the limits defined by its boundary.

Boundaries may evolve **only** when
a new domain becomes formally recognized.

---

## Scope Notes

This directory does not define:

- Domain internals
- Implementation details
- Tooling or workflows
- Integration mechanisms

Those belong exclusively to downstream repositories.
