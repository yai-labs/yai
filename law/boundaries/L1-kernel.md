# L1 — Kernel Boundary

## Purpose

Define the YAI **Kernel boundary** as the canonical authority for:

* state machine enforcement (what transitions are permitted)
* runtime guards and capability checks (who may transition and under which constraints)
* trace emission (how transitions become accountable artifacts)
* validation of Vault state at runtime entry points

If L1 is bypassed or ambiguous, execution becomes non-governable and YAI ceases to be structurally compliant.

---

## Authority

L1 is the single authority for:

* the canonical state machine (states + transitions)
* transition validation rules and guards
* capability requirements for transitions
* the minimum trace evidence emitted per transition

No downstream system may redefine kernel state transitions or authorize state change outside the kernel boundary.

---

## Definitions

**Kernel**
The enforcement layer that authorizes and executes state transitions under explicit constraints.

**Transition**
A state change from `S -> S'` that is validated, guarded, and trace-emitted.

**Guard**
A mandatory precondition that must hold for a transition to be valid.

**Capability Check**
A verification that the caller/context possesses the authority required to request a transition.

**Audit Trace**
The canonical evidence artifact produced for every valid (and rejected) transition attempt.

---

## In Scope

* State machine rules, states, and transitions
* Transition guards, capability checks, and non-bypassable enforcement
* Trace emission for every transition attempt (allowed or rejected)
* Runtime validation of Vault header + layout compatibility at kernel entry
* Replay-relevant determinism constraints (what must be captured to reproduce)

---

## Out of Scope

* Intent planning, routing, orchestration
* Model calls, agent logic, tool selection
* UI flows, user interaction patterns
* External services integration (network, disk, providers) beyond kernel-defined interfaces

---

## Canonical Contract

### State Machine Surface

L1 defines:

* the canonical state set
* the canonical transition set
* the canonical validity conditions for each transition

A transition is valid iff:

1. it is defined by the L1 state machine
2. all guards hold
3. capability requirements are satisfied
4. the transition emits a trace record meeting the minimum evidence requirements

### Guard Rails and Capabilities

* Every transition MUST declare its required capability set (or authority scope).
* Guards MUST be explicit, testable, and stable.
* Failed guards MUST not mutate state.
* Rejected transitions MUST still produce trace evidence (rejection trace).

### Trace Emission Requirements

For each transition attempt, the kernel MUST emit a trace artifact containing, at minimum:

* transition id (or `(from_state, to_state)` canonical name)
* actor/caller identity or authority reference
* declared intent/purpose (or canonical reason code)
* guard evaluation result(s)
* vault context reference (layout/version + relevant region pointers)
* timestamp / monotonic sequence id (for ordering)
* outcome: `accepted | rejected` + rejection reason when applicable

Trace is not logging. It is the minimum semantic evidence required by YAI invariants.

### Vault Validation

At kernel entry:

* Vault header signature MUST be validated (L0)
* version/layout compatibility MUST be validated
* region bounds/alignment MUST be validated
* kernel MUST refuse execution if the vault is invalid or incompatible

---

## Enforcement / Mechanism

* Formal model is authoritative for the **spec** of behavior:

  * `formal/YAI_KERNEL.tla`
  * `formal/YAI_KERNEL.cfg`
* Runtime enforcement must match the model:

* `../kernel/src/core/*`
* Every transition must be:

  * checked (guards + capabilities)
  * executed (state update)
  * trace-emitted (accepted/rejected)

---

## Interfaces

This boundary binds:

* `formal/YAI_KERNEL.tla`, `formal/YAI_KERNEL.cfg`
* `../kernel/include/yai_kernel.h`, `../kernel/include/kernel.h`
* `../kernel/include/yai_vault.h`
* `specs/protocol/protocol.h`

All downstream components interact with the kernel only through these surfaces (direct state mutation elsewhere is non-compliant).

---

## Compatibility Policy

**Compatible changes**

* adding new transitions with explicit guards/capabilities + trace schema stability
* adding new states if they do not change existing transition meaning
* adding new trace fields in a backward-compatible way

**Breaking changes**

* changing existing transition semantics or guard meaning
* removing states/transitions without a deprecation window
* weakening trace requirements
* drift between formal model and runtime enforcement

Breaking changes require versioning at the kernel boundary and synchronized downstream updates.

---

## Failure Modes

* **Invalid transition without trace** (violates I-001; non-auditable)
* **Silent state change without guard/capability check** (violates A-002 and I-003)
* **Formal/runtime drift** (model says “impossible”, runtime allows it)
* **Vault incompatibility ignored** (L0 is bypassed, system becomes undefined)

---

## Traceability

This boundary must remain compliant with:

* `invariants/I-001-traceability.md`
* `invariants/I-002-determinism.md`
* `invariants/I-003-governance.md`

In particular: transition evidence must be sufficient to reconstruct why a transition occurred or was rejected.
