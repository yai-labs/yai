# TEST_HARDFAIL (Manuale)

## Precondizioni
- `yai` buildato in release
- kernel/engine/mind build ok

## Test 1 — Up + sanity
```bash
yai down --ws dev --force

yai up --ws dev --build --detach
yai status --ws dev
```
Atteso:
- kernel alive
- runtime sock esiste
- control sock esiste

## Test 2 — Hard-fail kernel
```bash
KPID=$(ps aux | grep "[y]ai-kernel" | grep dev | awk '{print $2}' | head -n1)
kill -KILL "$KPID"
```
Atteso:
- eventi: `kernel_dead`
- status: kernel alive=false
- runtime.sock rimosso

## Test 3 — Cleanup
```bash
yai down --ws dev --force
```
Atteso:
- nessun `yai-kernel` in `ps`
- nessun socket `yai_runtime_dev.sock`
- nessun `control.sock`

## Test 4 — Idempotenza
```bash
yai down --ws dev --force
yai down --ws dev --force
```
Atteso:
- nessun errore

