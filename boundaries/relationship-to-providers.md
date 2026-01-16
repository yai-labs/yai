# Boundary: Foundation → Providers

## Purpose

Define the boundary between the **ICE Foundation** and the **ICE Providers** domain.

This document establishes what the Foundation constrains with respect to
external services, infrastructures, and dependencies, and what it explicitly
does **not** select, integrate, or manage.

The goal is to prevent foundational authority from collapsing
into vendor choice or service-specific assumptions.

---

## Role of the Foundation

The ICE Foundation:

- Defines axioms and structural invariants
- Constrains what forms of dependency are conceptually valid
- Establishes requirements for authority, traceability, and determinism
- Defines limits on delegation to external systems

The Foundation defines **what must remain invariant**,  
not **who provides capabilities**.

---

## Role of Providers

ICE Providers:

- Supply external capabilities (compute, storage, models, services)
- Implement concrete integrations with third-party systems
- Handle vendor-specific APIs, SLAs, and operational constraints
- May change independently of ICE semantics

Providers answer **where capabilities come from**,  
never **what is true** or **what is allowed**.

---

## Explicit Non-Responsibilities of the Foundation

The Foundation does NOT define:

- Vendor selection or preference
- Cloud, on-prem, or hybrid strategies
- Model providers or service marketplaces
- Cost, billing, or quota management
- Availability, redundancy, or failover policies
- Integration credentials or secrets handling

All such concerns belong exclusively to the Providers domain.

---

## Constraint Relationship

- The Foundation constrains **what may be delegated**
- Providers implement delegated capabilities
- Providers may fail or change
- The Foundation must remain unaffected

If a provider violates an invariant,
the provider integration is invalid — not the Foundation.

---

## Invalid Boundary Violations

The following are invalid:

- Encoding provider assumptions as axioms
- Treating provider behavior as authoritative truth
- Allowing vendor constraints to redefine invariants
- Hard-coding provider semantics into the Foundation

Such actions constitute a boundary violation.

---

## Canonical Status

This boundary is authoritative.

Any provider integration claiming ICE compliance
must demonstrate that it operates strictly
within the constraints defined here.

The Foundation constrains Providers.  
Providers never redefine the Foundation.
