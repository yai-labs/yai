---
id: WORKSPACE-VERTICALIZATION-MANUAL-TEST-COMMAND-PACK-v0.1.0
status: active
owner: runtime-governance
updated: 2026-03-10
scope: [yai, yai-law, yai-cli, yai-sdk]
related:
  - docs/program/audit-convergence/WORKSPACE-VERTICALIZATION-ECOSYSTEM-VERIFICATION-MATRIX-v0.1.0.md
  - tests/integration/workspace/workspace_verticalization_closeout.sh
---

# Workspace Verticalization Manual Test Command Pack (v0.1.0)

All commands are copy-paste ready.

## A) Prerequisites

Run once:

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
./dist/bin/yai --version
./dist/bin/yai help ws
```

Healthy signal:
- `help ws` lists `graph`, `db`, `data`, `knowledge`, `policy`, `domain`, `recovery`, `debug`, `query`.

Blocker signal:
- `INTERNAL ERROR registry unavailable` -> fix `YAI_SDK_COMPAT_REGISTRY_DIR`.

## B) Workspace lifecycle / binding

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
WS=prepilot_manual_wsv6_01

./dist/bin/yai lifecycle up || true
./dist/bin/yai ws clear || true
./dist/bin/yai ws create "$WS"
./dist/bin/yai ws open "$WS" || true
./dist/bin/yai ws set "$WS"
./dist/bin/yai ws switch "$WS"
./dist/bin/yai ws current
./dist/bin/yai ws status
./dist/bin/yai ws inspect
```

Healthy signal:
- `Workspace set/current/status/inspect` show selected/bound workspace and runtime capability block.

Acceptable transitional signal:
- `SERVER UNAVAILABLE` or `PROTOCOL ERROR` if runtime endpoint is not up.

## C) Graph family (`ws graph ...`)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"

./dist/bin/yai ws graph summary || true
./dist/bin/yai ws graph workspace || true
./dist/bin/yai ws graph governance || true
./dist/bin/yai ws graph decision || true
./dist/bin/yai ws graph evidence || true
./dist/bin/yai ws graph authority || true
./dist/bin/yai ws graph artifact || true
./dist/bin/yai ws graph lineage || true
./dist/bin/yai ws graph recent || true
```

Healthy signal:
- structured workspace/graph output with query/result summaries.

Acceptable transitional signal:
- `BAD ARGS` / `PROTOCOL ERROR` / `SERVER UNAVAILABLE` (runtime not ready or missing active binding).

## D) DB family (`ws db ...`)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"

./dist/bin/yai ws db status || true
./dist/bin/yai ws db bindings || true
./dist/bin/yai ws db stores || true
./dist/bin/yai ws db classes || true
./dist/bin/yai ws db count || true
./dist/bin/yai ws db tail || true
```

Healthy signal:
- status/inspect/query-derived output for store binding and classes/count/tail.

Note:
- `ws db` is first-class CLI surface but partly composition-backed.

## E) Data family (`ws data ...`)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"

./dist/bin/yai ws data events || true
./dist/bin/yai ws data evidence || true
./dist/bin/yai ws data governance || true
./dist/bin/yai ws data authority || true
./dist/bin/yai ws data artifacts || true
./dist/bin/yai ws data enforcement || true
```

Healthy signal:
- query result payloads for selected family.

## F) Knowledge family (`ws knowledge ...`)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"

./dist/bin/yai ws knowledge status || true
./dist/bin/yai ws knowledge transient || true
./dist/bin/yai ws knowledge memory || true
./dist/bin/yai ws knowledge providers || true
./dist/bin/yai ws knowledge context || true
```

Healthy signal:
- inspect/query visibility for transient/memory/providers/context.

Note:
- some subcommands are query-substrate-backed.

## G) Policy family (`ws policy ...`)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
OBJ=customer.default.org-workspace-contextual-review

./dist/bin/yai ws policy dry-run "$OBJ" || true
./dist/bin/yai ws policy attach "$OBJ" || true
./dist/bin/yai ws policy activate "$OBJ" || true
./dist/bin/yai ws policy effective || true
./dist/bin/yai ws policy detach "$OBJ" || true
```

Healthy signal:
- effective stack/effect/authority/evidence summaries in policy output.

## H) Domain / recovery / debug

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"

./dist/bin/yai ws domain get || true
./dist/bin/yai ws domain set --family scientific --specialization parameter-governance || true

./dist/bin/yai ws recovery status || true
./dist/bin/yai ws recovery load || true
./dist/bin/yai ws recovery reopen "$WS" || true

./dist/bin/yai ws debug resolution || true
```

Healthy signal:
- domain declared/effective context, recovery state, and resolution diagnostics are visible.

## I) Generic fallback (`ws query <family>`)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"

./dist/bin/yai ws query workspace || true
./dist/bin/yai ws query events || true
./dist/bin/yai ws query evidence || true
./dist/bin/yai ws query governance || true
./dist/bin/yai ws query authority || true
./dist/bin/yai ws query artifact || true
./dist/bin/yai ws query enforcement || true
./dist/bin/yai ws query transient || true
./dist/bin/yai ws query memory || true
```

Policy:
- fallback is valid low-level path, not primary UX where dedicated families exist.

## J) Runtime artifact checks (DB/graph/evidence reality)

```bash
cd /Users/francescomaiomascio/Developer/YAI/cli
WS=${WS:-prepilot_manual_wsv6_01}

ls -la "$HOME/.yai/run/data/$WS" || true
ls -la "$HOME/.yai/run/data/$WS/data" || true
ls -la "$HOME/.yai/run/$WS/runtime/graph" || true
ls -la "$HOME/.yai/run/$WS/events" || true

tail -n 5 "$HOME/.yai/run/$WS/runtime/graph/persistent-nodes.v1.ndjson" | jq . || true
tail -n 5 "$HOME/.yai/run/$WS/runtime/graph/persistent-edges.v1.ndjson" | jq . || true
tail -n 5 "$HOME/.yai/run/$WS/events/decision-records.v1.ndjson" | jq . || true
tail -n 5 "$HOME/.yai/run/$WS/events/evidence-records.v1.ndjson" | jq . || true
```

Optional duckdb checks:

```bash
duckdb -readonly "$HOME/.yai/run/data/$WS/data/data_plane.duckdb" -csv -c "select count(*) as total_records from yai_records;" || true
duckdb -readonly "$HOME/.yai/run/data/$WS/data/data_plane.duckdb" -csv -c "select record_class, count(*) as n from yai_records group by 1 order by 1;" || true
duckdb -readonly "$HOME/.yai/run/data/$WS/data/data_plane.duckdb" -csv -c "select record_class, record_key, created_at from yai_records order by created_at desc limit 20;" || true
```

Healthy signal:
- data-plane and graph artifacts exist and are readable for active workspace.

## K) SDK typed surface checks

```bash
cd /Users/francescomaiomascio/Developer/YAI/sdk
make test
make examples
./dist/bin/example_04_workspace_verticalized || true
```

Healthy signal:
- `workspace_typed_surface_smoke: ok`
- new example compiles and runs (or reports runtime unavailable cleanly).

## L) Law registry and cross-repo sanity checks

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
tests/integration/workspace/workspace_verticalization_closeout.sh

python3 - <<'PY'
import json
p='/Users/francescomaiomascio/Developer/YAI/law/registry/commands.v1.json'
obj=json.loads(open(p,encoding='utf-8').read())
ws=[c for c in obj.get('commands',[]) if c.get('entrypoint')=='ws']
print('ws_commands=',len(ws))
print('topics=',sorted({c.get('topic') for c in ws if c.get('topic')}))
PY
```

Healthy signal:
- closeout script prints `workspace_verticalization_closeout: ok`
- `topics` include `graph`, `db`, `data`, `knowledge`, `policy`, `domain`, `recovery`, `debug`, `query`.

## M) Fast sanity (minimum useful subset)

```bash
cd /Users/francescomaiomascio/Developer/YAI/yai
tests/integration/workspace/workspace_verticalization_closeout.sh

cd /Users/francescomaiomascio/Developer/YAI/cli
export YAI_SDK_COMPAT_REGISTRY_DIR="../law"
./dist/bin/yai help ws
./dist/bin/yai ws graph summary || true
./dist/bin/yai ws db status || true

cd /Users/francescomaiomascio/Developer/YAI/sdk
make test
```

Interpretation:
- if all three blocks pass, the WSV canonical surface is aligned and operationally testable.
