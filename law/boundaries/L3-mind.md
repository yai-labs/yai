# L3 — Mind Boundary

## Purpose

Define the **Mind boundary** as the layer responsible for:

* intent formation
* planning and orchestration
* model/tool routing (proposal only)
* transformation of inputs into valid command requests

L3 produces **proposals**, not execution.

L3 cannot bypass kernel (L1) or engine (L2), and cannot write protected vault fields.

---

## Authority

L3 may:

* propose intent
* propose plans
* propose tool/model selections
* propose command sequences

L3 may NOT:

* authorize execution
* perform state transitions
* execute commands directly
* mutate vault state outside permitted request channels
* redefine protocol IDs or semantics (owned by L0/specs)

Any system where “mind inference causes execution” is invalid (A-002).

---

## Definitions

**Intent**
A declarative statement of what the system is trying to achieve, including constraints and justification references.

**Plan**
An ordered or conditional structure that proposes a set of commands and dependencies, without executing them.

**Command Proposal**
A request that must be representable as a valid command ID + schema defined by L0/specs, and must pass L1 gating.

**Cognitive Reconfiguration**
An explicit transition between cognitive configurations, represented as a Reconfiguration Record (I-004).

---

## In Scope

* Planning and routing logic (task decomposition, sequencing, branching)
* Agent selection and orchestration (choose which internal “agent” proposes what)
* Model adapter usage (LLM calls, embeddings, scoring) for proposal generation
* Tool selection and provider routing (proposal of which tool/provider would be used)
* Input validation, normalization, and policy routing (deciding which constraints apply)
* Production of Reconfiguration Records when assumptions become invalid

---

## Out of Scope

* Direct execution on the vault
* Direct state transition authority (kernel-owned)
* Kernel/engine bypass
* Protocol/ID changes (L0/specs-owned)
* Writing protected vault fields or forging kernel trace ids
* Performing external effects (network/storage side effects) directly

---

## Canonical Contract

### Proposal-Only Rule

L3 outputs MUST be limited to:

* command proposals
* plan graphs / orchestration directives
* metadata required for gating (authority references, intent, constraints)
* evidence references for traceability

If L3 performs effects directly (writes/IO/network), that’s an architectural violation of L1/L2 + I-006.

### Command Validity Rule

Every action proposed by L3 MUST map to:

* a valid command ID (L0/specs)
* a valid command schema (payload shape)
* a declared intent and purpose (I-001 attribution requirement)

If L3 cannot express something as a valid command, it cannot happen.

### No Vault Mutation Rule

L3 MUST NOT write to protected vault fields.

Any vault interaction by L3 must be:

* read-only, or
* confined to explicitly permitted request channels (e.g., “proposal slots”, “intent buffer”) defined by L0/L1

Everything else must route through kernel + engine.

### Bounded Nondeterminism Rule

L3 may use probabilistic inference (LLMs, sampling, heuristics), but it MUST provide:

* bounded nondeterminism declaration (what may vary)
* stable evidence (inputs, prompts/templates, model identity, parameters)
* reproducible envelope (so replays can explain outcomes even if not bit-identical)

L3 nondeterminism is allowed only if it preserves I-001 traceability and I-002 reproducibility within defined scope.

### Reconfiguration Rule

When L3 detects invalid assumptions or cognitive model failure, it MUST:

* suspend/stop proposing execution under invalid assumptions
* emit a Reconfiguration Record (I-004) that includes:

  * invalidation statement
  * prior configuration reference
  * new configuration proposal
  * authority reference required to resume

L3 cannot self-authorize the resumption. Only L1 governance can allow it.

### External Effect Boundary Respect

L3 MUST classify proposed commands as internal vs external-effecting (I-006).

For external-effect proposals, L3 must attach strengthened metadata:

* intended target
* impact scope
* justification reference
* risk attribute (I-005) as an abstract cost signal

L3 proposes; L1 authorizes; L2 executes with augmented evidence.

---

## Enforcement / Mechanism

* L3 outputs must be representable as valid commands only (spec-defined IDs + schemas)
* All execution must route through kernel + engine
* No direct writes to protected vault fields
* Server endpoints must not provide “shortcut execution APIs” that bypass L1/L2
* Proposal artifacts must carry trace linkage fields required by I-001

---

## Interfaces

* `mind/src/*`
* `mind/src/cognition/llm/adapter.rs`
* `mind/src/server.rs`
* `mind/src/transport/bridge/*` (if present; must remain proposal-only)
* `specs/protocol/*`

---

## Failure Modes

* Direct execution without kernel gating (violates A-002, I-003)
* Producing commands that do not match spec IDs/schemas (violates L0)
* Skipping trace/intent attribution (violates I-001)
* Unbounded nondeterminism without evidence (violates I-002)
* Silent adaptation without Reconfiguration Record (violates I-004)
* Proposing external effects without strengthened metadata (violates I-006 / I-005)

---

## Traceability

This boundary must comply with:

* `invariants/I-001-traceability.md`
* `invariants/I-002-determinism.md`
* `invariants/I-003-governance.md`
* `invariants/I-004-cognitive-reconfiguration.md`
* `invariants/I-005-abstract-cost-accountability.md`
* `invariants/I-006-external-effect-boundary.md`
