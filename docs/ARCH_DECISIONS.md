# Architecture Decisions (Law-Aligned)

This document captures architectural commitments.
It is intentionally stable and grounded in `law/` invariants.

---

## ADR-001 — Single Runtime Per Machine, Multiple Workspaces (Target)

**Decision**
Adopt a **single runtime per machine** (kernel + engine), supporting **multiple workspaces**.

**Constraints**
- Every runtime-bound request MUST carry `ws_id`.
- Kernel/engine MUST enforce per-workspace isolation (no cross-ws effects).
- Storage/logs/trace MUST remain per-workspace by default.

**Law Alignment**
L1/L2 enforce authority and effect boundaries; L3 remains proposal-only.

**Status**
Target architecture (design locked; implementation staged).

---

## ADR-002 — Root Control Plane Per Machine (Entry Point)

**Decision**
Introduce a **Root Control Plane** as the **entry point** for cockpit/CLI.
Root exists **per machine**, is **runtime-aware**, and is the only component a cockpit connects to initially.

**Responsibilities (Root)**
- Runtime status and health.
- Workspace discovery/listing.
- Workspace attach/detach (routing to a specific workspace plane).
- Enforcing the machine-level boundary: *one cockpit, many workspaces*.

**Non-Goals**
- Root does not own L3 cognition or workspace memory/graph.
- Root does not execute external effects; it routes and enforces boundaries.

**Status**
Target architecture; implemented initially as a minimal stub (A2).

---

## ADR-003 — Mind Per Workspace (Workspace Plane)

**Decision**
Maintain **one Mind per workspace** as the **workspace plane** (userland):
graph, providers state, chat sessions, and proposal-only cognition.

**Constraints**
- Mind is workspace-scoped and cannot be an entry point for the whole machine runtime.
- No cross-workspace inference/state sharing inside Mind.

**Law Alignment**
Mind remains L3 proposal-only; any effectful execution must be routed via lower layers with explicit authority.

**Status**
Current architecture and long-term commitment (role clarified).

---

## ADR-004 — Protocol Contract (Strict, Single Contract)

**Decision**
RPC must enforce:
- `protocol_handshake` (version + capability check)
- `ws_id` mandatory on all runtime-bound requests
- `arming=true` + `role=operator` for privileged commands
- Deterministic, auditable errors (code + trace_id + ws_id)

**Notes**
- Cockpit/CLI first speak to Root Control Plane; then they attach to a workspace plane.
- The contract is unified across UI and CLI (no parallel protocols).

**Status**
Handshake + gating enforced now; ws_id enforced immediately (match/routing semantics evolve with multi-tenant runtime).

---

## ADR-005 — Event Schema Versioning + Validation Policy

**Decision**
All events MUST include:
- `schema_id`
- `event_version` (or `v`)

Validation policy is explicit: `off | warn | strict`.

**Status**
Schema/version required; validation can tighten over time.

---

## Implementation Phases (Summary)

1. **Protocol & Gate (A)**:
   enforce handshake, arming/role; introduce strict envelope and error model; ws_id mandatory.
2. **Root Control Plane Stub (A2)**:
   add machine-level root socket (status, workspaces.list, workspace.attach); cockpit connects to root first.
3. **Event Discipline (B)**:
   event envelope + schema registry + validation policy.
4. **Workspace Guard (C)**:
   strict workspace selection end-to-end (UI + CLI + tauri + daemon).
5. **Connection Lifecycle (D)**:
   session semantics + subscribe robustness + reconnect correctness.
6. **Authority & Proof (E/F/G/H/I)**:
   authority leases, anti-replay, delegation, scoped capabilities.
7. **Runtime Cutover**:
   migrate from per-ws daemons to true multi-tenant runtime while keeping contracts stable.
