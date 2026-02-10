# YAI Axioms

This document defines the axioms of YAI.

Axioms are the highest layer of authority in YAI: they constrain meaning, not implementation.
All downstream documents (invariants, boundaries, specs, formal models) must be compatible with these axioms.

If an axiom is violated, the system is not a valid instance of YAI.

## What an Axiom Is in YAI

In YAI, an axiom is:

- A foundational assumption taken as true by definition
- A constraint on meaning and admissible behavior
- A source of authority for all downstream concepts
- Independent from execution, implementation, and empirical observation

An axiom is not a design preference, a best practice, or an optimization strategy.

## Properties

All axioms are:

- Non-configurable: cannot be enabled, disabled, or tuned
- Context-independent: hold across environments and deployments
- Pre-execution: assumed before any runtime begins
- Authoritative: cannot be overridden by any component or document
- Non-derivable: not inferred from behavior or data

## Relationship to the Rest of YAI

Axioms constrain all authority layers:

- L0 (Vault): data layout and state representation must not contradict axioms
- L1 (Kernel): enforcement and state transitions must preserve axioms
- L2 (Engine): execution must remain within the axiom-bounded space
- L3 (Mind): routing and planning must not create meanings forbidden by axioms

## Axioms vs Structural Invariants

Axioms define what is assumed to be true.

Structural invariants define what must never be violated during execution.

Invariants derive their meaning from axioms.
Axioms do not depend on invariants.

## Axioms vs Design Principles

Design principles guide how systems are built.

Axioms constrain what systems are allowed to mean.

Principles may change.
Axioms do not.

## Axioms vs Implementation

Implementations realize behavior.

Axioms constrain the space of valid implementations.

An implementation that contradicts an axiom is invalid by definition.

## Canonical Status

The axioms in this repository are canonical.
Any ambiguity here propagates inconsistency across the ecosystem.

Downstream documents MUST declare which axioms they derive from (e.g. Derives from: A-001, A-002).

## Evolution

Axioms are expected to be:

- Rarely changed
- Explicitly versioned if modified
- Treated as ecosystem-breaking changes

Changes require deliberate review and system-wide alignment.

## Scope

This document does not define:

- Structural invariants
- Runtime semantics
- Governance mechanisms
- Tooling or developer workflows

Those concepts are defined downstream and must remain consistent with these axioms.
