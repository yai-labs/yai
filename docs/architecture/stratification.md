# Architecture Stratification (2026 Runtime Model)

This file defines the **operational stratification contract** of YAI.

It formalizes:

- Responsibilities per layer
- Coupling boundaries
- Allowed authority flow
- Storage & event domains
- Certification gates

This document is binding for implementation.

---

# Runtime Topology (Canonical)

Machine Runtime:

Root (Machine Plane)
 ├── L1 Kernel (Authority Plane)
 ├── L2 Engine (Execution Plane)
 └── L3 Mind(s) (Workspace Cognitive Planes)

Single runtime per machine.
Multiple workspaces per runtime.
Single canonical entrypoint (Root).

---

# Strata Contract

| Layer | Responsibility | API / ABI Surface | Storage Scope | Events Domain | Certification Gates |
|------|---------------|------------------|--------------|--------------|--------------------|
| L0 | Law specs, canonical ABI, protocol, identifiers | `law/specs/*`, generated headers | ABI + schema artifacts | N/A (spec-level only) | `scripts/verify/law-kernel.sh`, `scripts/verify/core.sh` |
| Root | Machine control plane, routing, runtime registry | root socket, control-plane contract | machine runtime metadata | runtime lifecycle + attach events | `scripts/gates/ws.sh`, `scripts/verify/core.sh` |
| L1 | Kernel authority, session control, isolation, enforcement | `kernel/include/*`, FSM, session registry | per-workspace runtime surface | authority transitions + violations | `scripts/verify/law-kernel.sh` |
| L2 | Engine deterministic execution + gates + cortex | `engine/include/*` | deterministic execution state | execution evidence + gate signals | `scripts/gates/cortex.sh`, `scripts/verify/core.sh` |
| L3 | Mind workspace plane (cognition + memory + providers) | `mind/src/*` | per-workspace graph + provider state | graph + provider + memory events | `scripts/gates/events.sh`, `scripts/gates/graph.sh`, `scripts/gates/providers.sh` |
| L4 | CLI / Cockpit interface layer | `tools/cli/*` | no authority state | command status only | `yai verify core`, `yai verify full` |
| L5 | Deterministic release/test orchestration | `scripts/verify/*`, `scripts/gates/*`, `scripts/suites/*` | test artifacts + logs | pass/fail evidence | `scripts/suites/levels/l0-l7.sh`, `scripts/suites/ops/no-llm-360.sh` |

---

# Authority Flow Rule

Authority flows strictly downward:

Root → L1 (Kernel) → L2 (Engine)

L3 (Mind) proposes only.

Rules:

- L3 cannot execute effects.
- L2 cannot validate authority.
- L1 cannot execute business logic.
- Root cannot mutate workspace memory.

No layer may reinterpret a lower-layer contract.

---

# Entry Point Rule

Only Root is externally connectable.

Forbidden:

- Direct CLI → workspace socket
- Direct CLI → engine
- Direct Mind → engine without L1 approval

Attach protocol:

1. Handshake
2. Root attach workspace
3. Kernel validates
4. Engine executes

---

# Compute Rule

Probabilistic compute lives in L3.

Deterministic commit lives in L1/L2.

No probabilistic path may:

- mutate authority
- bypass gates
- perform external effects

All effects must pass:

Authority → Gate → Deterministic execution → Audit surface

---

# Workspace Isolation Contract

Isolation is enforced at:

1. Session layer (Kernel session registry)
2. Storage layer (per-workspace run directories)
3. RPC routing (Root dispatch)
4. Authority gating (L1 enforcement)

Disallowed:

- Cross-workspace inference sharing
- Shared memory surfaces between Mind instances
- Direct engine invocation without ws context

---

# Memory Subgraphs Contract (L3)

Subgraphs:

- working
- episodic
- semantic
- vector

Allowed directional coupling:

- working → episodic
- working → semantic
- episodic → semantic
- semantic → vector (indexing only)

Forbidden:

- semantic → authority mutation
- episodic → kernel state mutation
- vector → direct execution trigger

All effectful transitions must return to L1/L2.

---

# Allowed Operational Edges

Explicitly allowed runtime relations:

- blocked_by_kernel
- trusted_by
- attached_to
- has_capability
- validated_by
- executed_under

Any new relation must:

1. Be declared in specs.
2. Be introduced in code.
3. Be covered by at least one gate assertion.

---

# Layer Coupling Rules

- L(n) may depend on L(n-1) contracts.
- L(n) may not reinterpret or override lower contracts.
- L4 must remain thin (no hidden authority logic).
- L5 must run deterministically from clean environment.

---

# Boot Canonicalization Rule

Boot is the only machine entrypoint.

Responsibilities:

- Directory integrity
- Vault ABI verification
- Root socket initialization
- Runtime startup

Direct launching of internal binaries is deprecated.

---

# Engine Integration Model (Target)

Engine becomes shared execution plane:

Root
 ├── Kernel (authority)
 ├── Engine (shared execution core)
 └── Workspace contexts (logical only)

No per-workspace engine processes long-term.

---

# Certification Sequence (Deterministic)

1. `scripts/suites/levels/l0-l7.sh`
2. `scripts/suites/ops/no-llm-360.sh`
3. Provider isolation gates
4. Cortex deterministic tests
5. Prompt/LLM suites (separate certification phase)

---

# Non-Negotiable Invariants

- Authority is centralized in L1.
- Execution is deterministic in L2.
- Cognition is proposal-only in L3.
- Entry is centralized in Root.
- Law (L0) defines contracts, never implementation.

---

# Architectural Principle

One Machine Runtime.
Many Workspace Contexts.
One Authority Plane.
One Execution Plane.
Many Cognitive Planes.

Deterministic below.
Probabilistic above.
Governed at the boundary.
