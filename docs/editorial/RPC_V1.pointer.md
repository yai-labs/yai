# RPC v1 (Law-owned, Code-enforced)

Editorial pointer aligned with the target control-plane switch: the RPC contract is law-owned and code-enforced.

## Canonical sources (Single Source of Truth)

**Authoritative (normative):**
- `law/specs/control/control_plane.v1.json`
- `law/specs/protocol/protocol.h`
- `law/specs/protocol/transport.h`
- `law/specs/protocol/commands.h`
- `law/specs/protocol/yai_protocol_ids.h` (if used)

**Derived / enforced (must match law):**
- `kernel/*` (control-plane server)
- `tools/*` (CLI client)
- `engine/*` (optional client or observer)
- `mind/*` (workspace client, not entry point)

**Non-authoritative:**
- logs, ad-hoc notes, code comments

**Policy:**
- any behavior change MUST update `law/specs/*` + tests
- if code disagrees with `law/specs/*`, the code is wrong

---

## Transport

- **UDS socket (workspace):** `~/.yai/run/<ws_id>/control.sock`
- **UDS socket (root, target):** `~/.yai/run/root.sock`
- **Framing:** NDJSON / JSON-Lines (1 JSON object per line)
- **Encoding:** UTF-8
- **Connection model:**
  - session: handshake → request/response loop → optional event stream
  - legacy one-request-per-conn is transitional only

---

## Envelope (v1) — Canonical wire shape

### RequestEnvelopeV1
```json
{
  "v": 1,
  "ws_id": "dev",
  "trace_id": "cli-1700000000000-0001",
  "arming": false,
  "role": "user",
  "request": {
    "type": "status",
    "payload": {}
  }
}
```

### ResponseEnvelopeV1
```json
{
  "v": 1,
  "ws_id": "dev",
  "trace_id": "cli-1700000000000-0001",
  "ok": true,
  "response": {
    "type": "status",
    "payload": { "state": "ready" }
  }
}
```

### ErrorEnvelopeV1
```json
{
  "v": 1,
  "ws_id": "dev",
  "trace_id": "cli-1700000000000-0009",
  "ok": false,
  "error": {
    "code": "ws_id_required",
    "message": "ws_id is required for runtime-bound requests",
    "detail": { "required_fields": ["v","trace_id","ws_id","arming","role","request"] }
  }
}
```

**Mandatory invariants:**
- `v` MUST be 1
- `trace_id` MUST be present (<= 64 chars)
- `ws_id` MUST be present and non-empty for runtime-bound requests
- `request.type` MUST be non-empty
- `request.payload` MUST be a JSON object or null
- Requests not wrapped in the v1 envelope are rejected

---

## Handshake (required-first)

The first message on a session MUST be `protocol_handshake`.

### Request: protocol_handshake
```json
{
  "v": 1,
  "ws_id": "dev",
  "trace_id": "cli-...-0001",
  "arming": false,
  "role": "user",
  "request": {
    "type": "protocol_handshake",
    "payload": {
      "client_version": "0.1.0",
      "capabilities": ["rpc.v1", "events.stream"]
    }
  }
}
```

### Response: protocol_handshake_ok
```json
{
  "v": 1,
  "ws_id": "dev",
  "trace_id": "cli-...-0001",
  "ok": true,
  "response": {
    "type": "protocol_handshake_ok",
    "payload": {
      "protocol_version": 1,
      "server_version": "0.1.0",
      "capabilities": ["rpc.v1", "events.stream"],
      "policy": { 
        "ws_required": true, 
        "arming_required_for_privileged": true 
      }
    }
  }
}
```

**Enforcement:**
- any non-handshake request before handshake → `handshake_required`
- incompatible protocol/capabilities → `unsupported_protocol`

---

## Privilege gate (arming + role)

Privilege classification is owned by: `law/specs/control/control_plane.v1.json`

**Runtime enforcement rules:**
- **safe requests:** allowed after handshake + ws binding
- **privileged requests:** require `arming=true` AND `role=operator`
- **destructive requests:** privileged + extra confirmation fields (command-specific)
- `authority_ref` is staged; do not create parallel ad-hoc auth fields in v1

**Law invariant:**
- no external effects without arming + operator

---

## Requests & responses

The canonical list of request types + their privilege class lives in: `law/specs/control/control_plane.v1.json`

This document is an index only.

---

## Event streaming

`events_subscribe` switches the session to streaming:
- only valid after handshake
- server emits `events_started` then repeated `event`
- events MUST use the event envelope defined by law specs

---

## Error codes (minimum)

- `bad_request`
- `handshake_required`
- `unsupported_protocol`
- `ws_id_required`
- `ws_id_mismatch`
- `not_armed`
- `role_required`
- `unauthorized`
- `not_found`
- `internal_error`