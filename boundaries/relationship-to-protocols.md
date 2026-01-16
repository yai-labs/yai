# Boundary: Foundation → Protocols

## Purpose

Define the boundary between the **ICE Foundation** and the **ICE Protocols** domain.

This document establishes what the Foundation constrains with respect to
communication, contracts, and interoperability, and what it explicitly does
**not** define, encode, or transmit.

The goal is to prevent conceptual authority from leaking into
communication formats or transport-level decisions.

---

## Role of the Foundation

The ICE Foundation:

- Defines axioms and structural invariants
- Constrains what kinds of interactions are **valid**
- Establishes requirements for authority, traceability, and determinism
- Defines semantic correctness of interactions

The Foundation defines **meaning and validity**, not communication.

---

## Role of Protocols

ICE Protocols:

- Define communication formats and contracts
- Encode messages, events, and interactions
- Enable interoperability between ICE components and external systems
- Handle serialization, transport, and versioning concerns

Protocols answer **how systems talk to each other**,  
never **what is true** or **what is allowed**.

---

## Explicit Non-Responsibilities of the Foundation

The Foundation does NOT define:

- Message schemas or wire formats
- Transport mechanisms or networking layers
- API shapes or endpoint definitions
- Version negotiation or backward compatibility strategies
- Serialization or deserialization logic
- Interoperability standards

All such concerns belong exclusively to the Protocols domain.

---

## Constraint Relationship

- The Foundation constrains **what interactions may mean**
- Protocols encode and transmit those interactions
- Protocols may evolve independently
- Protocols may not reinterpret axioms or invariants

If a protocol allows an interaction that violates an invariant,
the protocol is invalid — not the Foundation.

---

## Invalid Boundary Violations

The following are invalid:

- Encoding authority rules inside protocol definitions
- Treating protocol acceptance as semantic validity
- Allowing transport success to imply correctness
- Using protocols to bypass invariants

Such actions constitute a boundary violation.

---

## Canonical Status

This boundary is authoritative.

Any protocol claiming ICE compliance must demonstrate
that its contracts and interactions operate strictly
within the constraints defined here.

The Foundation constrains Protocols.  
Protocols never redefine the Foundation.
