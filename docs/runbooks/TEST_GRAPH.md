# TEST_GRAPH (Manuale)

## Precondizioni
- `yai` buildato
- Graph v1 presente in yai-mind

## Test 1 — Inserimento nodi/archi
```bash
yai graph add-node --ws dev --id n1 --kind file --meta '{"path":"foo.c"}'
yai graph add-node --ws dev --id n2 --kind error --meta '{"code":"E_RUNTIME"}'
yai graph add-edge --ws dev --src n1 --dst n2 --rel blocked_by_kernel --weight 1.0
```
Atteso: comandi OK, file `~/.yai/run/dev/graph/graph.json` aggiornato.

## Test 2 — Query
```bash
yai graph query --ws dev --text "runtime sock" --k 4
```
Atteso: ritorna nodi + edge (deterministico).

## Test 3 — Persistenza
```bash
# riavvia processo
# (non serve runtime core)
yai graph query --ws dev --text "runtime sock" --k 4
```
Atteso: stessi nodi/edge restano presenti.

