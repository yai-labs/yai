# YAI Global Stress Test Dataset (v1)

Questo dataset serve per test **end-to-end** la tua architettura YAI: Authority (Law/Kernel), Control Plane (daemon/RPC/events),
Execution (Engine/Mind), Memory stratificata (episodic/semantic/vector/activation) e Provider lifecycle.

Contenuto
- `YAI_Global_Stress_Test_v1.xlsx`  → foglio di lavoro con 85 prompt/test + seed data
- `prompts.csv`                    → stesso elenco in CSV
- `seed/semantic_nodes.jsonl`      → nodi semantici seed (sintetici)
- `seed/semantic_edges.jsonl`      → archi semantici seed
- `seed/episodic_events.jsonl`     → eventi episodici seed (NDJSON)
- `scripts/import_seed_via_cli.sh` → importer minimal via CLI (semantic add-node/add-edge)
- `scripts/load_events_log.sh`     → carica events.log per episodic ingest

Nota su privacy
I dati “birra 2024-02-13” e simili sono **sintetici** (tag `seed`) e servono solo a testare la pipeline.
Non inserire mai dati personali reali in seed dataset.

Nota su "Ecoh-style"
Nel foglio trovi prompt Enterprise/Governance ispirati a scenari reali (BI/Analytics, Salesforce, PA compliance, ISO).
Non è per “colpire” nessuno: è per stressare davvero governance e audit.

## Uso rapido (manuale)

1) Build
- yai-core: `make all`
- yai-mind: `cargo build --release`

2) Import seed nodes/edges (opzionale, se vuoi popolare il grafo)
- Imposta `BIN` e ws:
  ```bash
  export BIN="20 20 101 12 61 79 80 81 98 702 701 33 100 204 250 395 398 399 400command -v yai)"
  export WS="dev"
  bash scripts/import_seed_via_cli.sh
  ```

3) Episodic ingest (events.log → episodic)
```bash
bash scripts/load_events_log.sh
# opzionale: se hai comando ingest nel graph layer
# "$BIN" graph episodic ingest-events --ws "$WS"
```

4) Esegui prompt/test
- Apri `YAI_Global_Stress_Test_v1.xlsx` e vai in ordine per cluster.

5) Eventi
- `"$BIN" events --ws "$WS"` deve restare stabile (multi-client).

## Prerequisiti (pre‑R2)
- `yai` binary disponibile (da `yai-mind`).
- Embedding locale ONNX disponibile:
  - `~/.yai/models/embeddings/all-MiniLM-L6-v2/model.onnx`
  - `~/.yai/models/embeddings/all-MiniLM-L6-v2/tokenizer.json`
- `events.log` popolato (da `scripts/load_events_log.sh`).

## Test a blocchi (consigliato)

### Blocco A — Setup e Seed
```bash
export BIN="20 20 101 12 61 79 80 81 98 702 701 33 100 204 250 395 398 399 400command -v yai)"
export WS="dev"

cd ~/Developer/YAI/yai-core
make all
cd ~/Developer/YAI/yai-mind
cargo build --release

cd ~/Developer/YAI/yai-core/docs/datasets/yai-dataset-global-stress-v1
bash scripts/import_seed_via_cli.sh
bash scripts/load_events_log.sh
```
**Expected**
- `semantic.sqlite` creato in `~/.yai/run/$WS/`
- `events.log` presente in `~/.yai/run/$WS/`

### Blocco B — Query grafi (semantic → vector → activation)
```bash
$BIN graph query --ws "$WS" --text "runtime sock" --k 8
```
**Expected**
- `embedder: onnx`
- nodi ordinati per activation
- archi coerenti con seed

### Blocco C — Events (multi-client)
```bash
$BIN events --ws "$WS"
```
Apri un secondo terminale e ripeti lo stesso comando.
**Expected**
- stream continuo senza crash
- eventi con `ws_*` e `proc_*` in up/down

### Blocco D — Control plane sanity
```bash
$BIN status --ws "$WS" --json | jq .
```
**Expected**
- `control_sock_exists=true`
- `runtime_sock_exists=true` se ws up
- `halt_reason=null` se ws healthy

### Blocco E — Episodic ingest check (pre‑R2)
```bash
$BIN graph query --ws "$WS" --text "episodic" --k 4
```
**Expected**
- nessun crash (episodic derivato da events.log)

### Blocco F — Cleanup (optional)
```bash
$BIN down --ws "$WS"
rm -f ~/.yai/run/$WS/events.log
rm -f ~/.yai/run/$WS/semantic.sqlite
rm -f ~/.yai/run/$WS/vector.usearch
```

## Troubleshooting rapido
- **embedder fallback hash** → manca ONNX: riesegui `./scripts/fetch-embeddings.sh`
- **events empty** → manca `events.log`: riesegui `scripts/load_events_log.sh`
- **no nodes** → verifica `scripts/import_seed_via_cli.sh` e id formato `node:<kind>:<slug>`

## Mapping ai layer (come leggere i test)
- Authority/Law: deve rifiutare o far rifiutare al kernel; lascia audit/eventi.
- Lifecycle/Control: deve garantire ws lock, status JSON coerente, cleanup sockets.
- Memory/Episodic: usa solo eventi persistiti; niente invenzioni.
- Memory/Semantic: spiega e collega i documenti/spec presenti nel repo.
- Graph/Vector/Activation: embedding→top-k→spreading; output deterministico.
- Providers/LLM: discover/pair/attach/detach e persistenza trust.
