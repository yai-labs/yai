---
id: UNIFIED-RUNTIME-MANUAL-TEST-COMMAND-PACK-v0.1.0
status: active
owner: runtime-governance
updated: 2026-03-10
related:
  - docs/program/audit-convergence/UNIFIED-RUNTIME-ECOSYSTEM-VERIFICATION-MATRIX-v0.1.0.md
  - docs/program/24-milestone-packs/unified-runtime-closeout/CL-1-UNIFIED-RUNTIME-ECOSYSTEM-CLOSEOUT.md
---

# Unified Runtime Manual Test Command Pack (v0.1.0)

Assumption: local multi-repo root is `/Users/francescomaiomascio/Developer/YAI`.

## Section A — Structural checks

### A1) `yai` topology and canonical families

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
ls include/yai
ls lib
test -d include/yai/brain && echo "FAIL include/yai/brain present" || echo "OK no include/yai/brain"
test -d lib/brain && echo "FAIL lib/brain present" || echo "OK no lib/brain"
ls lib/exec/agents lib/exec/orchestration
ls lib/data lib/graph lib/knowledge
```

Healthy signal:
- `OK no include/yai/brain`
- `OK no lib/brain`
- canonical family directories present.

Blocker signal:
- active `include/yai/brain` or `lib/brain` directory exists.

### A2) `yai-law` machine-readable contract coherence

```bash
cd /Users/francescomaiomascio/Developer/YAI/law
rg -n "workspace-first|runtime_family_targets|disallowed_subsystem_targets|canonical_runtime_families" \
  manifests/runtime.entrypoints.json manifests/governance-attachability.constraints.v1.json \
  schema/workspace_governance_attachment.v1.schema.json
```

Healthy signal:
- explicit workspace-first model and canonical runtime-family targets.

Acceptable transitional outcome:
- `brain`/`mind` may appear only in disallowed/deprecated/compatibility context.

## Section B — Build and automated checks

### B1) `yai` build + hard runtime integration block

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
make clean
make yai -j4
tests/integration/workspace/workspace_session_binding_contract.sh
tests/integration/workspace/workspace_inspect_surfaces.sh
tests/integration/workspace/workspace_real_flow.sh
tests/integration/workspace/workspace_runtime_contract.sh
tests/integration/workspace/workspace_db_first_read_cutover.sh
tests/integration/workspace/workspace_event_evidence_sink_hardening.sh
tests/integration/workspace/workspace_governance_persistence.sh
tests/integration/workspace/workspace_authority_artifact_persistence.sh
tests/integration/workspace/workspace_brain_graph_transient.sh
tests/integration/workspace/workspace_graph_materialization_hooks.sh
tests/integration/workspace/workspace_graph_read_surfaces.sh
```

Healthy signal:
- build succeeds
- each script above prints `: ok`.

Blocker signal:
- any script above fails; treat as runtime closeout blocker until fixed or de-authorized.

### B2) `yai-law` structural/registry validation

```bash
cd /Users/francescomaiomascio/Developer/YAI/law
make check
make validate-law-registry
```

Healthy signal:
- `[check] OK`
- `[registry] OK`

Blocker signal:
- registry/schema/manifest validation failure.

### B3) `yai-sdk` validation

```bash
cd /Users/francescomaiomascio/Developer/YAI/sdk
make test
```

Healthy signal:
- `sdk_smoke: ok`
- `workspace_smoke: ok`
- `public_surface_smoke: ok`
- `models_contract_smoke: ok`

### B4) `yai-cli` validation

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
make test
```

Healthy signal:
- all guardrails pass (`help`, `output_contract`, `operator_capability_pack`, `workspace_output`, etc.)

## Section C — Runtime and workspace behavioral checks (`yai-cli` operator flow)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
./dist/bin/yai lifecycle up
./dist/bin/yai ws clear || true
./dist/bin/yai ws create prepilot_manual_01
./dist/bin/yai ws set prepilot_manual_01
./dist/bin/yai ws status
./dist/bin/yai inspect runtime
./dist/bin/yai doctor runtime
./dist/bin/yai ws domain set --family economic --specialization payments
./dist/bin/yai ws run payment.authorize provider=bank resource=money-transfer amount=125 authority=reviewer || true
./dist/bin/yai ws policy effective || true
./dist/bin/yai ws debug resolution || true
./dist/bin/yai ws inspect || true
./dist/bin/yai ws unset || true
./dist/bin/yai lifecycle down || true
```

Healthy signal:
- `ws status` shows binding/capability sections.
- `inspect runtime` shows `Liveness` and `Readiness`.
- `ws run` returns structured outcome (can be `DENIED` by policy and is still healthy behavior).

Acceptable transitional outcome:
- `ws inspect` / policy surfaces can return deterministic canonical errors (`BAD ARGS`, `PROTOCOL ERROR`, `SERVER UNAVAILABLE`) in unstable environment conditions.

Blocker signal:
- unknown command taxonomy, ghost output, or malformed non-canonical output shapes.

## Section C2 — Runtime store/graph/switch/recovery hard checks (`yai`)

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
./build/bin/yai down --force || true
./build/bin/yai >/tmp/yai_manual_hard.log 2>&1 &
YAI_PID=$!
sleep 1
YAI_SOCK_PATH="$HOME/.yai/run/control.sock" python3 tests/integration/runtime_handshake/test_handshake.py
./build/bin/yai down --force || true
kill "$YAI_PID" 2>/dev/null || true
wait "$YAI_PID" 2>/dev/null || true

ls -la "$HOME/.yai/run/data" | head
find "$HOME/.yai/run/data" -maxdepth 4 -type d \( -name data -o -name graph -o -name knowledge -o -name transient \) | head -n 40
find "$HOME/.yai/run" -path "*/runtime/graph/index.v1.json" -o -path "*/events/decision-records.v1.ndjson" | head -n 40
```

Healthy signal:
- handshake succeeds.
- workspace runtime data roots and graph/event artifacts exist under `~/.yai/run`.

Blocker signal:
- no runtime data roots created for active workspaces.
- no graph index or decision/evidence stores after running integration block.

## Section C3 — Access graph and DB for a selected workspace (manual, copy-paste)

Assumption:
- runtime is up
- workspace is selected in CLI (example: `prepilot_manual_01`)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
WS=prepilot_manual_01

# 1) Operator-visible workspace/runtime truth
./dist/bin/yai ws current --json | jq .
./dist/bin/yai ws status --json | jq .
./dist/bin/yai inspect workspace --json | jq '.identity,.runtime_capabilities,.graph_persistence,.knowledge_transient_persistence,.read_path'

# 2) Workspace data root (DB + LMDB + graph/transient)
ls -la "$HOME/.yai/run/data/$WS"
ls -la "$HOME/.yai/run/data/$WS/data"
ls -la "$HOME/.yai/run/data/$WS/data/lmdb"

# 3) Runtime graph/event/evidence files for the same workspace
ls -la "$HOME/.yai/run/$WS/runtime/graph"
ls -la "$HOME/.yai/run/$WS/events"
tail -n 5 "$HOME/.yai/run/$WS/runtime/graph/persistent-nodes.v1.ndjson" | jq .
tail -n 5 "$HOME/.yai/run/$WS/runtime/graph/persistent-edges.v1.ndjson" | jq .
tail -n 5 "$HOME/.yai/run/$WS/events/decision-records.v1.ndjson" | jq .
tail -n 5 "$HOME/.yai/run/$WS/events/evidence-records.v1.ndjson" | jq .

# 4) DuckDB direct read (data plane truth)
duckdb -readonly "$HOME/.yai/run/data/$WS/data/data_plane.duckdb" -csv -c "select count(*) as total_records from yai_records;"
duckdb -readonly "$HOME/.yai/run/data/$WS/data/data_plane.duckdb" -csv -c "select record_class, count(*) as n from yai_records group by 1 order by 1;"
duckdb -readonly "$HOME/.yai/run/data/$WS/data/data_plane.duckdb" -csv -c "select record_class, record_key, created_at from yai_records order by created_at desc limit 20;"
```

Healthy signal:
- `inspect workspace` shows canonical persistence and capability fields.
- `data_plane.duckdb` exists and `yai_records` query returns rows.
- graph/event/evidence ndjson files exist and contain records.

Blocker signal:
- workspace selected but no `~/.yai/run/data/$WS/data/data_plane.duckdb`.
- graph/event files missing after `ws run` actions.
- `inspect workspace` missing canonical fields (`graph_persistence`, `knowledge_transient_persistence`, `runtime_capabilities`).

## Section C4 — Graph/query surfaces (runtime contract checks)

The canonical graph query/read contract is currently verified by integration surface scripts:

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
tests/integration/workspace/workspace_graph_materialization_hooks.sh
tests/integration/workspace/workspace_graph_read_surfaces.sh
```

Healthy signal:
- both scripts print `: ok`.
- graph summary/lineage/recent/workspace/governance read families return `db_first` read-path payloads.

Blocker signal:
- either script fails; treat as graph/query contract regression.

## Section D — SDK manual checks

```bash
cd /Users/francescomaiomascio/Developer/YAI/sdk
make
./build/tests/public_surface_smoke
./build/tests/models_contract_smoke
./build/tests/workspace_smoke
```

Healthy signal:
- all listed SDK smokes report `ok`.

Additional quick docs checks:

```bash
cd /Users/francescomaiomascio/Developer/YAI/sdk
rg -n "runtime|workspace|exec|data|graph|knowledge|binding|readiness" README.md docs
```

Healthy signal:
- unified runtime vocabulary visible in SDK README/docs.

## Section E — CLI manual checks

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
./dist/bin/yai help --groups
./dist/bin/yai help ws
./dist/bin/yai help inspect runtime
./dist/bin/yai inspect runtime
./dist/bin/yai ws status
./dist/bin/yai doctor runtime
./dist/bin/yai --json inspect runtime | jq .
```

Healthy signal:
- help taxonomy is coherent and workspace-first.
- `inspect runtime` includes explicit liveness/readiness split.
- `ws status` includes runtime capability families.
- JSON output parseable by `jq`.

Fast guardrail commands:

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
tests/integration/help_guardrail.sh
tests/integration/output_contract_v1_guardrail.sh
tests/integration/operator_capability_pack_guardrail.sh
tests/integration/workspace_output_guardrail.sh
```

## Section F — Cross-repo narrative/terminology checks

### F1) Canonical vocabulary convergence

```bash
cd /Users/francescomaiomascio/Developer/YAI
rg -n "workspace-first|core|exec|data|graph|knowledge|liveness|readiness|binding" \
  yai/docs law/docs sdk/docs cli/docs | head -n 200
```

Healthy signal:
- aligned terminology across active docs.

### F2) Legacy topology residue scan

```bash
cd /Users/francescomaiomascio/Developer/YAI
rg -n "\\bbrain\\b|\\bmind\\b" yai law sdk cli | head -n 300
```

Interpretation:
- healthy if hits are either:
  - compatibility alias handling, or
  - explicitly historical/de-authorized docs.
- suspicious/blocking if active canonical docs/help/public surfaces present legacy topology as current truth.

## Fast sanity bundle (copy-paste)

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai && make yai -j4 && tests/integration/workspace/workspace_runtime_contract.sh && tests/integration/workspace/workspace_graph_read_surfaces.sh
cd /Users/francescomaiomascio/Developer/YAI/law && make check && make validate-law-registry
cd /Users/francescomaiomascio/Developer/YAI/sdk && make test
cd /Users/francescomaiomascio/Developer/YAI/cli && export YAI_SDK_COMPAT_REGISTRY_DIR="../law" && make test
```

Expected final signal:
- all four command blocks complete successfully.
