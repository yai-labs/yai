# YAI Control Plane (Canonical)

This document defines the canonical control plane for YAI runtimes.
It is normative. Implementations must conform.

## Principles

1. **Single authority per workspace**: one daemon owns one `ws`.
2. **Clients are stateless**: CLI/YX/automation never spawn runtimes directly.
3. **RPC is the official channel**: all lifecycle and status queries go through RPC.
4. **No global sockets**: runtime sockets are namespaced per `ws`.
5. **Idempotent controls**: `up`/`down`/`status` are safe to repeat.

## Paths (Canonical)

Workspace directory:

```
~/.yai/run/<ws>/
```

Files:

- `control.sock` — RPC UDS for the daemon
- `lock` — workspace lock (daemon authority)
- `daemon.pid` — daemon PID
- `session.json` — session state (pids, pgid, socket)

Runtime socket:

```
/tmp/yai_runtime_<ws>.sock
```

Implementations may accept a template with `{ws}` but must resolve to a per‑ws socket.

## Process Groups

- All runtime processes (boot/engine/mind) must be placed in the same process group.
- `down` must kill the process group before falling back to individual PIDs.

## RPC Canon

Messages are JSON lines over UDS (`control.sock`).

Requests:

- `ping`
- `status`
- `up`
- `down`
- `providers_*`
- `events_subscribe`

Responses:

- `pong`
- `status`
- `up_ok`
- `down_ok`
- `providers`
- `provider_status`
- `event`
- `error`

See `control_plane.v1.json` for the canonical schema.
