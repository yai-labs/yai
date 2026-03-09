# Workspace Isolation Model (WS-7/8)

## Scope
This model hardens workspace containment at runtime-routing and state-namespace level. It does not claim OS-level sandboxing.

## Boundary Types
- Execution boundary: each control-flow executes against one workspace binding.
- Namespace boundary: runtime metadata and resolution state are scoped to `ws/<workspace_id>`.
- State boundary: inferred/effective/debug snapshots are workspace-owned.
- Trace boundary: resolution trace refs are persisted per workspace manifest.
- Artifact boundary: runtime state roots are anchored to `~/.yai/run/<workspace_id>`.

## Global vs Workspace-Scoped
- Workspace-scoped: declared/inferred/effective context, resolution summaries, trace refs, runtime attach flags, manifest lifecycle state.
- Global read-only: embedded law payload, registries, static protocol contracts.
- Global routed services: runtime daemon and dispatch plane, with strict workspace scope guard on runtime control-call path.

## Enforcement Rules
- If a workspace binding is active, runtime control-calls targeting another workspace are rejected (`cross_workspace_scope_denied`).
- Binding resolution fails closed on stale/invalid namespace signals.
- Manifest namespace consistency is validated at read time:
  - `session_binding` must match workspace id when present.
  - `runtime_state_root` and `metadata_root` must match expected per-workspace roots.
- On namespace mismatch, state is marked invalid and surfaced through status/inspect reasons.

## Boundary Signals in Surfaces
- `workspace status` exposes `namespace_scope`, `namespace_valid`, `boundary_reason`.
- `workspace inspect` exposes `boundary` object with namespace + enforcement state.
- `workspace policy/debug` expose workspace namespace and validity.

## Threat Assumptions Covered
- Accidental cross-workspace contamination by routing mismatch.
- Stale/forged binding file causing ambiguous scope.
- Manifest path drift causing workspace runtime-state collision.

## Residual Risk (Out of Scope for WS-7/8)
- No OS-level sandbox or privilege separation.
- No kernel-level memory isolation claims.
- No hardened multitenant hostile-code containment.

## Forward Hooks for stronger isolation
- isolation mode evolution (`process` -> stricter modes).
- workspace namespace tokens already exposed in inspect/status for future enforcement expansion.
- explicit boundary-reason propagation for incident/debug pipelines.
