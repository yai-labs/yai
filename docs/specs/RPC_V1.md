# RPC v1 (Control Plane)

## Transport
- **UDS** per workspace
- Path: `~/.yai/run/<ws>/control.sock`

## Versioning
- `rpc_version = 1` (implicit)
- Backwards compatible changes only within v1

## Error Model
- Server returns `type=error` with `message` on failure
- Transport failures => client must treat as daemon unavailable

## Request Types (v1)

### `ping`
No payload.

### `status`
No payload.

### `up`
Payload:
- `build` (bool)
- `no_engine` (bool)
- `no_mind` (bool)
- `ai` (bool)
- `timeout_ms` (int, optional)

### `down`
Payload:
- `force` (bool)
- `shutdown` (bool)

### `providers_discover`
Payload (optional):
- `endpoint` (string, optional)
- `model` (string, optional)

### `providers_list`
No payload.

### `providers_pair`
Payload:
- `id` (string)
- `endpoint` (string)
- `model` (string)

### `providers_attach`
Payload:
- `id` (string)
- `model` (string, optional)

### `providers_detach`
No payload.

### `providers_status`
No payload.

### `providers_revoke`
Payload:
- `id` (string)

### `events_subscribe`
No payload. Opens an NDJSON stream of events.

## Response Types (v1)

### `pong`
No payload.

### `status`
Payload:
- `state` (RunState or null)
- `alive` (boot/kernel/engine/mind)
- `daemon_pid` (int)

### `up_ok`
No payload.

### `down_ok`
Payload:
- `shutdown` (bool)

### `providers`
Payload:
- `items` (array of ProviderInfo)

### `provider_status`
Payload:
- `active` (ProviderInfo or null)

### `providers_ok`
No payload.

### `events_started`
No payload. Stream follows.

### `error`
Payload:
- `message` (string)

