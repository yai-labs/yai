# L2 — Engine Boundary

## Purpose

Define the **Engine boundary** as the deterministic execution layer that:

* consumes intent and commands
* performs constrained effects (transport/storage) under declared limits
* writes results back to the vault
* produces execution evidence

L2 executes under constraints defined by L0 (vault layout / IDs) and L1 (state machine / authority / trace requirements).

L2 holds **no authority**.

---

## Authority

L2 has **no authority to authorize execution**, change policy, redefine IDs, or mutate kernel state semantics.

L2 is authoritative only for:

* deterministic execution mechanics (how commands are executed once authorized)
* effect application *within* declared limits
* result materialization (writing outputs/status to the vault)
* emission of effect evidence required for traceability and replay

---

## Definitions

**Intent / Command**
A request to perform a deterministic operation, routed through kernel governance.

**Effect**
A constrained interaction with transport/storage or any external surface.

**Execution Evidence**
The minimal artifact emitted by L2 so that kernel traces remain reconstructable (I-001) and replayable (I-002).

---

## In Scope

* Execution loop and command handling (consume authorized commands, produce results)
* Vault read/write for:

  * command inputs
  * command results
  * execution status
  * engine-side evidence references
* Transport and storage access under explicit limits (quotas, allowlists, caps)
* Abstract cost accounting hooks (time/compute/io/tokens/energy/risk as attributes)
* External-effect boundary classification support (provide evidence when effects occur)

---

## Out of Scope

* Intent planning, routing, or agent policy
* Model routing, prompts, scoring, or tool selection logic
* Authority changes, capability grants, or policy mutation
* State ID / command ID definition (owned by L0 + specs)
* Kernel state transitions (owned by L1)
* UI, observability UX, dashboards (tooling is downstream)

---

## Canonical Contract

### No-Authority Rule

L2 MUST NOT:

* execute unless kernel state allows it
* invent new commands or reinterpret command semantics
* mutate IDs, constants, or protocol meaning
* bypass guards by direct vault mutation

### State-Gated Execution

For every command execution, L2 MUST verify:

* current kernel state is in the engine-allowed set
* command id is recognized (L0/specs)
* command payload is structurally valid for that id
* any required capability/authority reference is present in the request context (even if L2 cannot grant it)

If any check fails, L2 MUST refuse execution and emit rejection evidence (not “silent no-op”).

### Determinism Rule

Given the same:

* vault inputs
* command payload
* declared configuration limits
* deterministic seeds / bounded nondeterminism declarations

L2 MUST produce behaviorally equivalent results within the declared scope.

Any nondeterminism must be explicit, bounded, and traceable (I-002).

### Evidence Emission Rule

L2 must produce evidence sufficient to support kernel trace reconstruction, including:

* command id + stable command instance id
* inputs hash / references
* outputs hash / references
* effects performed (transport/storage) and their targets
* cost attributes (abstract, not billing)
* outcome code (success/failure/refused) + reason

Important: kernel owns transition traces (L1).
Engine evidence MUST link to kernel trace ids / transition ids where applicable.

### External Effect Boundary Support

If an execution crosses the external effect boundary (I-006), L2 MUST provide augmented evidence:

* target identity (where the effect went)
* scope/impact classification
* justification reference (intent / authority reference id)
* failure semantics and partial-effect reporting

---

## Enforcement / Mechanism

* Engine loop guards must refuse execution outside allowed kernel states.
* Resource caps must be enforced locally (time, memory, IO, tokens, energy where measurable).
* Storage/transport must be mediated through engine gates that:

  * enforce allowlists/quotas
  * emit evidence
  * preserve replay-relevant parameters

---

## Interfaces

* `yai-engine/include/engine_bridge.h`
* `yai-engine/include/transport_client.h`
* `yai-engine/include/shared_constants.h`
* `specs/protocol/*`
* `kernel/include/yai_vault.h`

(Any additional interface introduced by L2 must not expand authority; it must remain an execution surface.)

---

## Failure Modes

* Executes outside allowed kernel state (violates L1 gating)
* Writes results that do not match command schema / ids (violates L0)
* Performs external effects without augmented evidence (violates I-006)
* Produces untraceable outcomes (violates I-001)
* Introduces unbounded nondeterminism (violates I-002)
* Bypasses kernel trace semantics by mutating state implicitly (violates I-003)

---

## Traceability

L2 must remain compliant with:

* `invariants/I-001-traceability.md`
* `invariants/I-002-determinism.md`
* `invariants/I-003-governance.md`
* `invariants/I-005-abstract-cost-accountability.md`
* `invariants/I-006-external-effect-boundary.md`

Specifically: engine evidence must be attachable to authorized execution contexts and remain reconstructable.
