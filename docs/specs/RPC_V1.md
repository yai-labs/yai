# RPC v1

Editorial pointer aligned with current runtime implementation.

Canonical sources:
- `law/specs/control/control_plane.v1.json`
- `mind/src/transport/rpc/protocol.rs`

Transport:
- UDS (`~/.yai/run/<ws>/control.sock`)
- NDJSON / JSON-lines framing

## Requests

- `ping`
- `status`
- `up`
- `down`
- `providers_discover`
- `providers_list`
- `providers_pair`
- `providers_attach`
- `providers_detach`
- `providers_revoke`
- `providers_status`
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

## Responses

- `pong`
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
- `chat_send`
- `shell_exec`
- `events_started`
- `event`
- `error`

## Error envelope

- `type = error`
- payload:
  - `message` (string)

## Notes

- CLI and YX are clients of the same RPC contracts.
- Lifecycle authority remains in daemon/control plane.
