# YAI Law — Terminology

This directory defines the controlled vocabulary of the YAI Law.

Its purpose is to prevent semantic drift, ambiguity, and reinterpretation
of foundational concepts across repositories and over time.

Terminology here is authoritative.

---

## Purpose

The terminology layer establishes:

- canonical meanings of foundational terms
- strict boundaries between similar or overloaded concepts
- stable definitions that downstream projects must respect
- a shared language for reasoning about YAI

This directory does not explain implementations or usage.
It defines meaning and authority, not behavior.

---

## Canonical Authority

All terms defined here:

- take precedence over informal usage elsewhere
- must be used consistently across YAI repositories
- may not be redefined by downstream projects
- may only evolve through explicit Law changes

If a term is ambiguous, it is considered undefined until clarified here.
Downstream projects MUST NOT invent local definitions for undefined terms.

---

## What Belongs Here

This directory contains:

- the canonical glossary of YAI terms
- distinctions between closely related concepts
- disallowed or deprecated terminology
- notes on semantic scope and constraints

Terms typically defined here include (non-exhaustive):

- execution
- authority
- state
- traceability
- determinism
- governance
- intent
- vault
- kernel
- engine
- mind
- consciousness
- external effect boundary
- abstract cost accountability
- docs authority boundary (Lx)

---

## What Does NOT Belong Here

This directory does NOT contain:

- implementation details
- API names or function signatures
- product branding
- UI labels or UX wording
- marketing language

Those belong to downstream documentation layers.

---

## Relationship to Other Law Layers

Axioms define what is assumed to be true.

Invariants define what must never be violated.

Boundaries define what the Law does not cover.

Terminology defines how these concepts are named and referenced.

Terminology does not introduce new truth.
It stabilizes existing truth.

---

## Canonical Files

`glossary.md`
The authoritative glossary of YAI terms.

Additional files MUST exist only to clarify meaning,
not to expand scope.

---

## Disallowed Terminology

The following terms are disallowed as Law claims unless explicitly defined here:

- autonomy
- self-authorizing behavior
- implicit authority

---

## Stability and Evolution

Terminology is expected to be:

- stable
- minimal
- precise

Changes are rare and high-impact.
Renaming or redefining a term is considered a breaking conceptual change
for the entire ecosystem.

---

## Final Note

If a concept cannot be named precisely,
it cannot be reasoned about safely.

Terminology is not auxiliary.
It is a structural component of the YAI Law.
