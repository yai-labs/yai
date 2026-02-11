# Events v1 (NDJSON)

## Obiettivo
Stabilizzare un Event Plane osservabile sopra il Control Plane UDS per‑WS.

## Trasporto
- UDS per workspace: `~/.yai/run/<ws>/control.sock`
- Subscribe via RPC: `events` (NDJSON stream)

## Formato
- NDJSON: una riga = un JSON completo

## Campi minimi (obbligatori)
- `v`: `1`
- `ts`: epoch seconds
- `ws`: string
- `seq`: u64 monotonic per‑WS
- `kind`: string (snake_case)
- `level`: `debug|info|warn|error`
- `msg`: string breve
- `data`: object (payload, può essere `{}`)

## Eventi minimi (MVP)

### Lifecycle
- `daemon_started`
- `ws_up_started`
- `ws_up_complete`
- `ws_down_started`
- `ws_down_complete`

### Process
- `proc_started { name: "boot|kernel|engine|mind", pid, pgid }`
- `proc_exit { name, pid, pgid, exit_code?, signal? }`

### State
- `status_snapshot { alive, pids, pgid?, runtime_sock?, control_sock? }`
- `status_changed { boot, kernel, engine, mind }`

### Provider
- `provider_discovered { id, endpoint, model }`
- `provider_paired { id, endpoint, model }`
- `provider_attached { id, model }`
- `provider_detached { id }`

### Error
- `error { component:"daemon|kernel|engine|mind|rpc", detail, code? }`

## Snapshot iniziale
Alla connessione `events`, il daemon deve inviare **1 `status_snapshot`** prima dello stream realtime.

## Backpressure
Client lenti: eventi droppati. Il daemon non deve bloccare.

