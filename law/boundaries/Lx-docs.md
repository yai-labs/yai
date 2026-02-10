# Lx — Docs Boundary

## Purpose

Define the **documentation boundary** of YAI Law:

* what is **normative** (source of truth)
* what is **explanatory** (interpretation / guidance)
* how changes are tracked to prevent drift

This boundary exists to ensure YAI remains **governable over time** by preventing
silent divergence between “what the system is” and “what downstream repos do”.

---

## Authority

Lx is the authority for **YAI Law as documentation**.

It defines:

* which documents are normative
* which documents are non-normative
* what constitutes a law-level change
* how downstream repos must align

Lx does not define end-user manuals, product docs, or tutorials.

---

## Normative vs Explanatory

### Normative (Source of Truth)

Normative documents are binding. They define what must be true.

In this repository, the following are **normative**:

* `axioms/*`
* `invariants/*`
* `boundaries/*`
* `specs/protocol/*`
* `formal/YAI_KERNEL.tla`, `formal/YAI_KERNEL.cfg` (when present)

If a downstream repo contradicts these, the downstream repo is wrong.

### Explanatory (Non-Normative)

Explanatory documents may help readers understand or apply the law, but do not
define authority.

Examples of explanatory content:

* conceptual notes
* examples
* walkthroughs
* tutorial-style guidance
* downstream READMEs that describe usage

Explanatory content must never override normative content.

---

## In Scope

* Definition and maintenance of all normative law surfaces:

  * axioms
  * invariants
  * boundaries
  * protocol specs / ABI declarations
  * formal models
* Definition of change-control rules for law-level modifications
* Drift prevention rules across repositories

---

## Out of Scope

* Usage guides, tutorials, “getting started”
* Tooling/build instructions (unless they are enforcement-critical and explicitly declared)
* Product/UI documentation
* Downstream repo READMEs and implementation notes (except for alignment references)

---

## Change Control

### Law-Level Change Rule

Any change that affects the meaning of system execution must update YAI Law first.

A change is law-level if it affects any of the following:

* protocol IDs, schemas, ABI, or shared-memory layout (L0)
* kernel state machine semantics or guards (L1)
* execution constraints that affect validity/compliance (axioms/invariants)
* external-effect classification rules (I-006)
* traceability, determinism, governance, or reconfiguration requirements

### Law Change Record Requirement

Any PR that introduces a law-level change MUST include an explicit change record
(either in PR body or a dedicated section/file) that states:

* what normative document(s) changed
* what semantic behavior is impacted
* what downstream repos must update to remain aligned
* whether the change is breaking or compatible

Silent law changes are non-compliant.

---

## Enforcement / Mechanism

* ABI/protocol changes MUST update this repo before downstream implementations.
* Downstream repos MUST align to the latest law; divergence is a violation, not a variant.
* Any downstream behavior that cannot be traced to a law surface is invalid by definition.

---

## Interfaces

Normative interfaces include:

* `axioms/*`
* `invariants/*`
* `boundaries/*`
* `specs/protocol/*`
* `formal/YAI_KERNEL.tla`
* `formal/YAI_KERNEL.cfg`

---

## Failure Modes

* ABI/protocol/state machine changes without law updates
* Conflicting “truth sources” across repos (duplicate, contradictory specs)
* Hidden policy shifts without traceable authority updates
* Explanatory docs treated as binding (role inversion)
* Drift accepted as “implementation choice”

---

## Traceability

This boundary must comply with:

* `invariants/I-001-traceability.md`
* `invariants/I-003-governance.md`
