# RPC v1

Editorial pointer aligned with the **target control-plane switch**: the RPC contract is **law-owned** and **code-enforced**.

## Canonical sources (Single Source of Truth)

Authoritative (normative):
- `law/specs/control/control_plane.v1.json` (commands, privilege class, capability tags)
- `law/specs/protocol/protocol.h` + `law/specs/protocol/transport.h` + `law/specs/protocol/commands.h` (wire contract identifiers + fields)
- `law/specs/protocol/yai_protocol_ids.h` (IDs / stable names, if used)

Derived / enforced (implementation, must match law):
- `mind/src/transport/rpc/*` (UDS NDJSON framing + validation + enforcement)
- `mind/src/control/daemon.rs` (gate: ws binding, handshake, arming/role, authority)
- `yai-yx/crates/yx-protocol/*` (client types mirroring law)
- `yai-yx/crates/yx-client/*` (client session + handshake + subscribe)

Non-authoritative:
- chat logs, ad-hoc notes, code comments (useful, never a spec)

> Policy: any behavior change must update `law/specs/*` (and tests).  
> If the code disagrees with `law/specs/*`, the code is wrong.

---

## Transport

- UDS socket: `~/.yai/run/<ws>/control.sock` (current per-workspace daemon)
- Framing: NDJSON / JSON-Lines (1 JSON object per line)
- Connection model:
  - **Session** (recommended): handshake → request/response loop → optional `events_subscribe` streaming
  - Legacy single-request-per-connection may exist during transition, but is not the long-term contract

---

## Envelope (v1)

All runtime-bound requests are wrapped in the v1 envelope.

### RequestEnvelopeV1 (wire shape)
```json
{
  "v": 1,
  "trace_id": "cli-1700000000000-0001",
  "ws_id": "dev",
  "arming": false,
  "role": "user",
  "authority_ref": null,
  "client": {
    "client_kind": "cli",
    "client_version": "0.1.0",
    "capabilities": ["rpc.v1", "events.stream"]
  },
  "request": {
    "type": "status",
    "args": {}
  }
}
```

### ResponseEnvelopeV1 (wire shape)

```json
{
  "v": 1,
  "trace_id": "cli-1700000000000-0001",
  "ws_id": "dev",
  "response": {
    "type": "status",
    "data": { "state": "ready" }
  }
}
```

### Mandatory invariants (enforced)

* `v` MUST be `1` for RPC v1.
* `ws_id` MUST be present and non-empty on all runtime-bound requests.
* `trace_id` MUST be present (client generated preferred; server may generate only for rejection paths).
* `client.client_kind` and `client.client_version` MUST be non-empty.
* Requests not wrapped in the v1 envelope are **rejected** (legacy acceptance is not guaranteed).

---

## Handshake (required)

The first message on a session MUST be `protocol_handshake`.

### Request: protocol_handshake

```json
{
  "v": 1,
  "trace_id": "yx-1700000000000-0001",
  "ws_id": "dev",
  "arming": false,
  "role": "user",
  "authority_ref": null,
  "client": {
    "client_kind": "yx",
    "client_version": "0.1.0",
    "capabilities": ["rpc.v1", "events.stream"]
  },
  "request": { "type": "protocol_handshake", "args": { "protocol_version": 1 } }
}
```

### Response: protocol_handshake_ok (or handshake response type)

```json
{
  "v": 1,
  "trace_id": "yx-1700000000000-0001",
  "ws_id": "dev",
  "response": {
    "type": "protocol_handshake_ok",
    "data": {
      "protocol_version": 1,
      "server_version": "0.1.0",
      "policy": {
        "ws_required": true,
        "arming_required_for_privileged": true
      }
    }
  }
}
```

Enforcement:

* Any non-handshake request before handshake MUST return `ERR_HANDSHAKE_REQUIRED`.
* If protocol versions/capabilities are incompatible, return `ERR_PROTOCOL_VERSION_MISMATCH` (or equivalent).

---

## Privilege gate (arming + role + authority_ref)

Privilege classification is owned by:

* `law/specs/control/control_plane.v1.json`

Runtime enforcement rules:

* Safe requests: allowed after handshake + valid ws binding.
* Privileged requests: require `arming=true` AND `role=operator`.
* Destructive requests: privileged + additional confirmation fields (command-specific) + audit emission.
* `authority_ref` is the authoritative handle for operator authority.
  * In early v1, it may be optional or "dev-mode".
  * Target is: privileged/destructive MUST require authority material (leases/tokens), but that is staged separately.

**No external effects without arming + operator.** (law invariant)

---

## Requests (v1)

The canonical list of request types is defined in:

* `law/specs/control/control_plane.v1.json` (source)

This document is an editorial index of currently supported types:

* `ping`
* `protocol_handshake`
* `status`
* `up`
* `down`
* `providers_discover`
* `providers_list`
* `providers_pair`
* `providers_attach`
* `providers_detach`
* `providers_revoke`
* `providers_status`
* `dsar_request`
* `dsar_status`
* `dsar_execute`
* `chat_sessions_list`
* `chat_session_new`
* `chat_session_select`
* `chat_history`
* `chat_send`
* `shell_exec`
* `events_subscribe`

Notes:

* A request is identified by `request.type`.
* Arguments must live under `request.args` and must be JSON objects.
* Adding/removing request types requires updating `law/specs/control/control_plane.v1.json`.

---

## Responses (v1)

* `pong`
* `protocol_handshake_ok` (or handshake response variant)
* `status`
* `up_ok`
* `down_ok`
* `providers`
* `provider_status`
* `providers_ok`
* `dsar_created`
* `dsar_state`
* `dsar_executed`
* `chat_sessions`
* `chat_session`
* `chat_history`
* `chat_sent`
* `shell_exec_result`
* `events_started`
* `event`
* `error`

Notes:

* A response is identified by `response.type`.
* Streaming events use `response.type = event`.

---

## Event streaming

`events_subscribe` opens a streaming mode on the session.

Rules:

* Only valid after handshake.
* Server MUST first emit `events_started`.
* Then emit repeated `event` responses.
* Events MUST be domain-enveloped (see `docs/specs/EVENTS_V1.md` and `law/specs/...`).

Example `event` response:

```json
{
  "v": 1,
  "trace_id": "yx-1700000000000-0042",
  "ws_id": "dev",
  "response": {
    "type": "event",
    "data": {
      "v": 1,
      "event_id": "01HV...",
      "ts": 1700000000123,
      "ws": "dev",
      "topic": "status_changed",
      "schema_id": "yai.events.status_changed.v1",
      "severity": "info",
      "payload": { "state": "ready" }
    }
  }
}
```

---

## Error model

Errors are returned as a normal response with `type = error`.

```json
{
  "v": 1,
  "trace_id": "cli-1700000000000-0009",
  "ws_id": "dev",
  "response": {
    "type": "error",
    "data": {
      "code": "ERR_WS_REQUIRED",
      "message": "ws_id is required for runtime-bound requests",
      "detail": {
        "required_fields": ["v", "trace_id", "ws_id", "arming", "role", "client", "request"]
      }
    }
  }
}
```

Canonical error codes (minimum set):

* `ERR_PROTOCOL_INVALID`
* `ERR_HANDSHAKE_REQUIRED`
* `ERR_PROTOCOL_VERSION_MISMATCH`
* `ERR_WS_REQUIRED`
* `ERR_WS_MISMATCH`
* `ERR_ARMING_REQUIRED`
* `ERR_ROLE_REQUIRED`
* `ERR_AUTHORITY_REQUIRED`
* `ERR_NOT_SUPPORTED`
* `ERR_INTERNAL`

---

## Compatibility & migration note

Historical note: `mind/src/transport/rpc/protocol.rs` is an implementation detail and may change.
The contract does **not** live there.

Target direction:

* the *wire contract* and command registry are law-owned under `law/specs/`
* code in `mind/` and `yai-yx/` must conform to law, validated by tests and CI

---

## Operational notes

* CLI and YX are clients of the same RPC contract.
* Cockpit (YX) is a UI over the runtime; it must not own the control plane contract.
* Changes to envelope fields, request types, privilege classes, or error codes require:
  1. update `law/specs/*`
  2. update enforcement + tests
  3. update this index document if it lists the affected types