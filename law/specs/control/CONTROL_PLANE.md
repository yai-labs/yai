# YAI Control Plane (Canonical)

This document defines the **canonical control plane** for YAI runtimes.
It is **normative**: implementations MUST conform.
Where this document conflicts with ad-hoc code or UI behavior, the code/UI must change.

Canonical schema source:
- `law/specs/control/control_plane.v1.json`

---

## Principles

1. **Authority is workspace-scoped**
   - One control plane daemon owns one workspace (`ws`) at a time.
   - Authority is represented by the daemon lock + audit trail.

2. **Clients never execute lifecycle directly**
   - CLI / YX / automations do not spawn runtime processes directly.
   - They only speak RPC to the control plane.

3. **RPC is the official channel**
   - Lifecycle (`up`, `down`), status (`status`), providers, DSAR, shell, and event subscription are controlled via RPC.
   - Local process inspection is not a control-plane API.

4. **Workspace namespacing is mandatory**
   - No global control sockets.
   - All runtime and control endpoints are bound to `ws`.

5. **Idempotent controls**
   - `up`, `down`, `status` MUST be safe to repeat.
   - Repeated calls must not cause drift or undefined side effects.

---

## Paths (Canonical)

Workspace runtime directory:
```
~/.yai/run/<ws>/
```

Required files:
- `control.sock` — RPC UDS endpoint for the daemon
- `lock` — workspace authority lock (exclusive ownership)
- `daemon.pid` — daemon PID (diagnostics only)
- `session.json` — session snapshot (pids/pgid/socket paths, best-effort)

Runtime endpoint (current per-ws):
```
/tmp/yai_runtime_<ws>.sock
```

Rules:
- Implementations may accept templates (`{ws}`) but MUST resolve to a per-workspace path.
- `session.json` is not a contract; it is a diagnostic artifact. RPC remains the contract.
- Storage/logs/trace are workspace-scoped (law invariant).

---

## Locking & Authority

- A daemon MUST acquire `~/.yai/run/<ws>/lock` before serving RPC for that workspace.
- If the lock is held, a second daemon MUST refuse to start for that workspace.
- Clients MUST treat lock contention as a normal error case (e.g., "daemon already running").

---

## Process Group Discipline

All runtime processes launched for a workspace MUST be placed in a single process group.

Requirements:
- The daemon MUST own the process group lifecycle.
- `down` MUST attempt group termination first, then escalate deterministically.

Canonical kill order:
1. `killpg(SIGTERM)`
2. wait (bounded timeout)
3. `killpg(SIGKILL)`
4. fallback: kill individual PIDs (diagnostic last resort)

---

## RPC Canon

Transport:
- UDS: `~/.yai/run/<ws>/control.sock`
- Framing: NDJSON / JSON-Lines (one JSON object per line)

Envelope:
- RPC wire format is defined by `RPC v1` (see `docs/specs/RPC_V1.md`)
- `ws_id` MUST be present on all runtime-bound requests
- Session handshake is required when running in session mode (`protocol_handshake`)

Requests (editorial index; canonical list is in `control_plane.v1.json`):
- `ping`
- `protocol_handshake`
- `status`
- `up`
- `down`
- `providers_discover`
- `providers_list`
- `providers_pair`
- `providers_attach`
- `providers_detach`
- `providers_status`
- `providers_revoke`
- `dsar_request`
- `dsar_status`
- `dsar_execute`
- `chat_sessions_list`
- `chat_session_new`
- `chat_session_select`
- `chat_history`
- `chat_send`
- `shell_exec`
- `events_subscribe`

Responses (editorial index; canonical list is in `control_plane.v1.json`):
- `pong`
- `protocol_handshake_ok`
- `status`
- `up_ok`
- `down_ok`
- `providers`
- `provider_status`
- `providers_ok`
- `dsar_created`
- `dsar_state`
- `dsar_executed`
- `chat_sessions`
- `chat_session`
- `chat_history`
- `chat_sent`
- `shell_exec_result`
- `events_started`
- `event`
- `error`

---

## Invariants

- All runtime-bound requests MUST carry `ws_id` and MUST be rejected on mismatch.
- Privileged requests MUST be gated (`arming=true` + `role=operator`) and audited.
- Events MUST be schema-versioned (see `docs/specs/EVENTS_V1.md` + law registry).
- The control plane must remain enforceable without UI (CLI-first verifiable behavior).

---

## Notes on Evolution

- Current architecture is per-workspace daemon + per-workspace control socket.
- A future multi-tenant runtime may introduce a global runtime kernel, but:
  - workspace binding, gating, and audit semantics MUST remain unchanged
  - per-workspace isolation MUST remain enforceable at L1/L2 boundaries
