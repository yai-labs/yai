# TEST_EVENTS (Manuale)

## Precondizioni
- `yai` buildato in release
- `yai up` funzionante

## Test 1 — Multi‑client
```bash
yai up --ws dev --detach
yai events --ws dev
```
Aprire 2 terminali aggiuntivi:
```bash
yai events --ws dev
```
**Atteso**: tutti ricevono eventi, nessun crash daemon.

## Test 2 — Snapshot (attach tardivo)
```bash
yai up --ws dev --detach
sleep 5
yai events --ws dev --tail 5
```
**Atteso**: `status_snapshot` + ultimi eventi.

## Test 3 — Kill engine/mind/kernel
```bash
kill -TERM <engine_pid>
```
**Atteso**: `proc_exit(engine)` + `status_changed` coerente.

```bash
kill -TERM <mind_pid>
```
**Atteso**: `proc_exit(mind)` + `status_changed` coerente.

```bash
kill -TERM <kernel_pid>
```
**Atteso**: `kernel_dead`, `ws_down_complete`, socket runtime rimosso.

## Test 4 — Down pulito
```bash
yai down --ws dev
```
**Atteso**: `ws_down_started` + `proc_exit` + `ws_down_complete`.

