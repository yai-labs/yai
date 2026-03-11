---
id: WS-COMMAND-VERTICALIZATION-REPORT-v0.1.0
status: active
owner: runtime-governance
updated: 2026-03-10
scope: [yai, yai-law, yai-sdk, yai-cli]
---

# Workspace/Graph/DB/Knowledge Command Verticalization Report (v0.1.0)

## 1) Source of truth used
- Law registry: `/Users/francescomaiomascio/Developer/YAI/law/registry/commands.v1.json`
- YAI runtime dispatch: `/Users/francescomaiomascio/Developer/YAI/yai/lib/core/session/session.c`
- CLI parser/app/help: `/Users/francescomaiomascio/Developer/YAI/cli/src/parse/parse.c`, `/Users/francescomaiomascio/Developer/YAI/cli/src/app/app.c`, `/Users/francescomaiomascio/Developer/YAI/cli/src/help/help.c`
- SDK command constants: `/Users/francescomaiomascio/Developer/YAI/sdk/include/yai_sdk/workspace.h`, `/Users/francescomaiomascio/Developer/YAI/sdk/include/yai_sdk/graph.h`, `/Users/francescomaiomascio/Developer/YAI/sdk/include/yai_sdk/data.h`, `/Users/francescomaiomascio/Developer/YAI/sdk/include/yai_sdk/knowledge.h`

## 2) Global registry snapshot (law)
- Total commands in registry: **2812**
- Entry points: **{'run': 1931, 'bundle': 200, 'config': 60, 'ws': 21, 'gov': 200, 'inspect': 200, 'verify': 200}**
- Surface classes: **{'plumbing': 1982, 'surface': 18, 'ancillary': 812}**
- Stability classes: **{'experimental': 2794, 'stable': 18}**
- Top groups (count): **[('boot', 200), ('bundle', 200), ('control', 200), ('core', 200), ('exec', 200), ('governance', 200), ('ingress', 200), ('inspect', 200), ('knowledge', 200), ('lifecycle', 200), ('memory', 200), ('orch', 200), ('substrate', 200), ('verify', 200), ('workspace', 12)]**

## 3) Workspace-focused cross-layer inventory
- Workspace command IDs in registry: **12**
- Workspace command IDs implemented in runtime dispatch (`yai`): **48**
- Workspace command IDs directly recognized by CLI parser: **20**
- Workspace command IDs exposed as SDK constants (`workspace.h`): **11**

### 3.1 Full Workspace command-ID matrix (runtime union)
| command_id | registry | runtime(yai) | cli(parse) | sdk(constants) | status |
| --- | --- | --- | --- | --- | --- |
| `yai.workspace.artifacts.list` | no | yes | no | no | validated_now |
| `yai.workspace.authority.list` | no | yes | no | no | validated_now |
| `yai.workspace.clear` | yes | yes | yes | no | implemented_cli_only |
| `yai.workspace.create` | no | yes | yes | no | validated_now |
| `yai.workspace.current` | yes | yes | yes | no | validated_now |
| `yai.workspace.debug.resolution` | no | yes | no | no | validated_now |
| `yai.workspace.debug_resolution` | yes | yes | yes | no | validated_now |
| `yai.workspace.destroy` | no | yes | yes | no | implemented_cli_only |
| `yai.workspace.domain.get` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.domain.set` | no | yes | no | no | validated_now |
| `yai.workspace.domain_get` | yes | yes | yes | no | implemented_cli_only |
| `yai.workspace.domain_set` | yes | yes | yes | no | validated_now |
| `yai.workspace.enforcement.status` | no | yes | no | no | validated_now |
| `yai.workspace.events.tail` | no | yes | no | no | validated_now |
| `yai.workspace.evidence.list` | no | yes | no | no | validated_now |
| `yai.workspace.governance.list` | no | yes | no | no | validated_now |
| `yai.workspace.graph.artifact` | no | yes | no | no | validated_now |
| `yai.workspace.graph.authority` | no | yes | no | no | validated_now |
| `yai.workspace.graph.decision` | no | yes | no | no | validated_now |
| `yai.workspace.graph.evidence` | no | yes | no | no | validated_now |
| `yai.workspace.graph.governance` | no | yes | no | no | validated_now |
| `yai.workspace.graph.lineage` | no | yes | no | no | validated_now |
| `yai.workspace.graph.recent` | no | yes | no | no | validated_now |
| `yai.workspace.graph.summary` | no | yes | no | yes | validated_now |
| `yai.workspace.graph.workspace` | no | yes | no | no | validated_now |
| `yai.workspace.inspect` | yes | yes | yes | yes | validated_now |
| `yai.workspace.lifecycle.maintain` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.lifecycle.model` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.lifecycle.status` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.open` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.policy.activate` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.policy.attach` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.policy.detach` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.policy.dry_run` | no | yes | no | no | runtime_only_not_cli |
| `yai.workspace.policy.effective` | no | yes | no | no | validated_now |
| `yai.workspace.policy_activate` | no | yes | yes | yes | implemented_cli_sdk_not_validated_now |
| `yai.workspace.policy_attach` | no | yes | yes | yes | implemented_cli_sdk_not_validated_now |
| `yai.workspace.policy_detach` | no | yes | yes | yes | implemented_cli_sdk_not_validated_now |
| `yai.workspace.policy_dry_run` | no | yes | yes | yes | implemented_cli_sdk_not_validated_now |
| `yai.workspace.policy_effective` | yes | yes | yes | no | validated_now |
| `yai.workspace.prompt_context` | no | yes | yes | no | implemented_cli_only |
| `yai.workspace.query` | no | yes | no | yes | validated_now |
| `yai.workspace.reset` | no | yes | yes | no | implemented_cli_only |
| `yai.workspace.run` | yes | yes | yes | yes | validated_now |
| `yai.workspace.set` | yes | yes | yes | yes | validated_now |
| `yai.workspace.status` | yes | yes | yes | yes | validated_now |
| `yai.workspace.switch` | yes | yes | yes | no | validated_now |
| `yai.workspace.unset` | yes | yes | yes | yes | validated_now |

## 4) CLI families actually exposed now (operator surface)
- Main entrypoints (`yai help --all`): `ws`, `run`, `gov`, `verify`, `inspect`, `bundle`, `config`, `doctor`, `watch`, `help`, `version`
- Workspace subcommands (`yai help ws`):
  - `create`
  - `reset`
  - `destroy`
  - `set`
  - `switch`
  - `current`
  - `status`
  - `inspect`
  - `unset`
  - `clear`
  - `domain get`
  - `domain set`
  - `run`
  - `policy dry-run`
  - `policy attach`
  - `policy activate`
  - `policy detach`
  - `policy effective`
  - `debug resolution`
  - `prompt-context`
  - `prompt-token`
- Gap: no direct operator family yet for `yai ws graph ...` and `yai ws db ...` as first-class CLI subcommands.

## 5) SDK families and constants relevant to WS/Graph/DB/Knowledge
- Workspace command constants:
  - `yai.workspace.graph.summary`
  - `yai.workspace.inspect`
  - `yai.workspace.policy_activate`
  - `yai.workspace.policy_attach`
  - `yai.workspace.policy_detach`
  - `yai.workspace.policy_dry_run`
  - `yai.workspace.query`
  - `yai.workspace.run`
  - `yai.workspace.set`
  - `yai.workspace.status`
  - `yai.workspace.unset`
- Graph family (`graph.h`): `YAI_SDK_GRAPH_CMD_SUMMARY` (`yai.workspace.graph.summary`), `YAI_SDK_GRAPH_CMD_QUERY` (`yai.workspace.query`)
- Data family (`data.h`) query families: `enforcement`, `authority`, `governance`, `evidence`, `artifact` via `yai.workspace.query`
- Knowledge family (`knowledge.h`) query families: `transient`, `memory` via `yai.workspace.query`

## 6) What is verified working now (green evidence)
Validated in current hard-verification run (yai integration):
- Workspace lifecycle/binding: create, set/switch, current, status, inspect, unset
- Runtime action path: run + domain_set + policy_effective + debug_resolution
- Query/data-plane families: workspace/governance/events/evidence/enforcement/authority/artifacts/graph
- Graph family queries: graph.summary, graph.workspace, graph.governance, graph.decision, graph.artifact, graph.authority, graph.evidence, graph.lineage, graph.recent
- DB/store reality: `data_plane.duckdb` + LMDB roots + graph/event/evidence artifacts persisted
- Recovery/read semantics: surfaced in inspect/runtime_capabilities and db-first read-path checks

Evidence scripts used (green):
- `tests/integration/workspace/workspace_session_binding_contract.sh`
- `tests/integration/workspace/workspace_inspect_surfaces.sh`
- `tests/integration/workspace/workspace_real_flow.sh`
- `tests/integration/workspace/workspace_runtime_contract.sh`
- `tests/integration/workspace/workspace_db_first_read_cutover.sh`
- `tests/integration/workspace/workspace_event_evidence_sink_hardening.sh`
- `tests/integration/workspace/workspace_governance_persistence.sh`
- `tests/integration/workspace/workspace_authority_artifact_persistence.sh`
- `tests/integration/workspace/workspace_brain_graph_transient.sh`
- `tests/integration/workspace/workspace_graph_materialization_hooks.sh`
- `tests/integration/workspace/workspace_graph_read_surfaces.sh`

## 7) Critical gaps to verticalize next (for rich WS inspectability)
1. **Registry/runtime drift**: many runtime workspace graph/query IDs are implemented in `yai` but absent from `law/registry/commands.v1.json`.
2. **CLI surface gap**: runtime has graph/query families, but CLI does not expose first-class `ws graph/*` and `ws db/*` families.
3. **SDK ergonomics gap**: constants exist (`query`, `graph.summary`) but no richer typed helper set for graph/db query families.
4. **Canonical-path objective**: promote graph/db families from runtime-only to CLI+SDK first-class operator/consumer surface.

## 8) Recommended verticalization target taxonomy
- `yai ws graph summary|workspace|governance|decision|artifact|authority|evidence|lineage|recent`
- `yai ws db classes|count|tail` (read-only, workspace-bound)
- `yai ws query <family>` kept as generic fallback, but no longer the only graph/data access path
- SDK: typed helpers for graph/data/knowledge query families on top of `yai.workspace.query`