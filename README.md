# ICE Runtime


[![License](https://img.shields.io/github/license/francescomaiomascio/ice-runtime)](LICENSE)
[![Stars](https://img.shields.io/github/stars/francescomaiomascio/ice-runtime)](https://github.com/francescomaiomascio/ice-runtime/stargazers)
[![Issues](https://img.shields.io/github/issues/francescomaiomascio/ice-runtime)](https://github.com/francescomaiomascio/ice-runtime/issues)
[![Last Commit](https://img.shields.io/github/last-commit/francescomaiomascio/ice-runtime)](https://github.com/francescomaiomascio/ice-runtime/commits/main)


<p align="center">
  <img src="docs/assets/ice-runtime-hero.png" alt="ICE Runtime execution architecture" width="85%">
</p>


> **The execution core of the ICE environment**  
> Orchestrating agents, workflows, and system state with explicit authority.

---

## What is ICE Runtime

ICE Runtime is the **authoritative execution layer** of the ICE ecosystem.

It is responsible for transforming abstract specifications into
**controlled, observable, and deterministic execution**.

ICE Runtime coordinates how intelligent systems:
- start execution
- transition state
- orchestrate agents and workflows
- enforce execution invariants
- terminate safely and explicitly

It is not a framework.
It is not a deployment tool.
It is not an application runtime in the traditional sense.

ICE Runtime is the **execution substrate** on which all ICE-compliant systems operate.

---

## Why ICE Runtime Exists

Modern intelligent systems often blur critical boundaries.

Execution, orchestration, inference, lifecycle management, and state control
are frequently entangled into opaque runtime behavior.

ICE Runtime exists to **separate these concerns explicitly**.

Its purpose is to make execution:

- intentional
- inspectable
- governable
- auditable over time

ICE Runtime treats execution as a **first-class system property**, not
as a side-effect of code running.

It exists to answer a fundamental question:

> **When is execution valid, and who has authority over it?**

---

## Core Responsibilities

ICE Runtime is responsible for:

- Defining a single authoritative execution entry point
- Managing lifecycle phases explicitly
- Orchestrating agents and workflows deterministically
- Enforcing forward-only execution semantics
- Managing and exposing system state transitions
- Emitting structured execution signals for observability
- Acting as the operational bridge between theory and implementation

ICE Runtime deliberately **does not** implement:

- user interfaces
- domain-specific business logic
- model inference
- long-term memory systems

These concerns belong to other ICE domains.

---

## Execution Model

ICE Runtime operates as a **controlled execution environment**.

Execution follows explicit phases:

### 1. Bootstrap
Validation of preconditions and authority handoff into runtime control.

### 2. Initialization
Registration and wiring of runtime components.

### 3. Execution
Deterministic orchestration of agents and workflows.

### 4. Observation
Emission of structured execution events and state snapshots.

### 5. Termination
Controlled shutdown with invariant enforcement.

Execution is **forward-only**.  
Rollback is **never implicit**.  
State transitions are **explicit and inspectable**.

---

## Position in the ICE Ecosystem

ICE Runtime is one domain within a modular system.

- **ICE Foundation**  
  Defines axioms, invariants, and non-negotiable execution rules.

- **ICE Engine**  
  Implements higher-level reasoning and decision-making logic  
  *(currently private)*.

- **ICE Observability**  
  Provides structured introspection, tracing, and analysis of execution.

ICE Runtime sits **between specification and execution**.

It is the layer where **theory becomes operational**.

---

## Project Status

ICE Runtime is under **active development**.

- APIs are evolving
- execution semantics are stabilizing
- public interfaces may change
- backward compatibility is not guaranteed yet

This repository should be considered **pre-stable**.

Design correctness and architectural clarity are prioritized over velocity.

---

## Getting Started

At this stage, ICE Runtime is intended for:

- contributors
- researchers
- system architects
- early adopters exploring execution-centric system design

Usage examples, reference implementations, and setup guides
will be introduced incrementally as the runtime matures.

---

## Contributing

Contributions are welcome and encouraged.

However, ICE Runtime enforces strict architectural boundaries.

Before contributing:

- understand the execution model
- read ICE Foundation documents
- avoid introducing implicit behavior
- avoid hidden or emergent state transitions

More detailed contribution guidelines will follow.

---

## License

This project is licensed under the terms specified in the LICENSE file.
