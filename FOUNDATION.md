# ICE Foundation Dependency

This project derives its core assumptions, invariants, and semantic boundaries
from **ICE Foundation v1.0.0**.

ICE Foundation defines the axiomatic and invariant layer that constrains:

- what this project is allowed to assume
- what may be inferred
- what may be executed
- what must never be violated

This project does not redefine, reinterpret, or override any foundational concept.

---

## Referenced Foundation

- Repository: https://github.com/francescomaiomascio/ice-foundation
- Version: v1.0.0
- Tag: v1.0.0-foundation

All architectural, behavioral, and execution-level decisions
must be evaluated against the referenced Foundation version.

---

## Authority of the Foundation

ICE Foundation is **authoritative** with respect to:

- axioms
- structural invariants
- epistemic boundaries
- semantic constraints

Any deviation from ICE Foundation invalidates
the conceptual integrity of this project.

---

## Change Policy

If ICE Foundation is updated:

- this project does **not** automatically inherit changes
- any Foundation version update must be explicit
- version upgrades require deliberate review and validation

Foundational assumptions are chosen, not drifted.

---

## Scope Note

This document establishes **dependency**, not implementation.

Details of runtime behavior, tooling, orchestration, or interfaces
belong to this project and its downstream layers,
but must remain compatible with the referenced Foundation.
