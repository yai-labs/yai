---
id: UNIFIED-RUNTIME-ECOSYSTEM-VERIFICATION-MATRIX-v0.1.0
status: active
owner: runtime-governance
updated: 2026-03-10
related:
  - docs/program/24-milestone-packs/unified-runtime-closeout/CL-1-UNIFIED-RUNTIME-ECOSYSTEM-CLOSEOUT.md
---

# Unified Runtime Ecosystem Verification Matrix (v0.1.0)

Result classes:
- `PASS`
- `PASS WITH DEBT`
- `FAIL`

| Domain | Repo(s) | Canonical truth expected | Automatic check / verification method | Evidence expected | Result | Residual issue / blocker | Owner / follow-up |
| --- | --- | --- | --- | --- | --- | --- | --- |
| No active `brain` public namespace | `yai` | `include/yai/brain` absent from active tree | `test -d include/yai/brain && echo bad || echo ok` | `ok` | PASS | none | n/a |
| No active `lib/brain` implementation tree | `yai` | `lib/brain` absent from active tree | `test -d lib/brain && echo bad || echo ok` | `ok` | PASS | none | n/a |
| `exec` owns agents/orchestration | `yai` | agents/orchestration live under `lib/exec/*` | `ls lib/exec/agents lib/exec/orchestration` | directories present and populated | PASS | none | n/a |
| Workspace-first binding implemented | `yai` | workspace binding/readiness surfaces active | `tests/integration/workspace/workspace_session_binding_contract.sh`; `tests/integration/workspace/workspace_inspect_surfaces.sh` | both scripts green; selected/bound/stale/invalid states validated | PASS | none | n/a |
| Runtime behavior integration scripts coherence | `yai` | canonical integration scripts aligned with current runtime semantics | `tests/integration/workspace/workspace_real_flow.sh`; `tests/integration/workspace/workspace_runtime_contract.sh` | both scripts green with active-vs-cross-workspace checks | PASS | none | n/a |
| Runtime surfaces expose `data/graph/knowledge/exec` truth | `yai` | capability-aware surfaces visible without legacy projection | `tests/integration/workspace/workspace_runtime_contract.sh`; `tests/integration/workspace/workspace_inspect_surfaces.sh` | `runtime_capabilities` includes `exec/data/graph/knowledge`; no `brain_persistence` in canonical inspect payload | PASS | none | n/a |
| Persistent store binding verified | `yai` | active workspace binds real runtime data roots and db-first read path | `tests/integration/workspace/workspace_runtime_contract.sh`; `tests/integration/workspace/workspace_db_first_read_cutover.sh` | `~/.yai/run/data/<ws>/{data,graph,knowledge,transient}` exists; `read_path.mode=db_first` and fallback behavior coherent | PASS | none | n/a |
| Event/evidence/governance persistence verified | `yai` | runtime actions emit durable records visible to query/inspect | `tests/integration/workspace/workspace_event_evidence_sink_hardening.sh`; `tests/integration/workspace/workspace_governance_persistence.sh`; `tests/integration/workspace/workspace_authority_artifact_persistence.sh` | all scripts green and persistence artifacts emitted under runtime stores | PASS | none | n/a |
| Graph materialization + graph read/query verified | `yai` | runtime records materialize graph state and read surfaces reflect it | `tests/integration/workspace/workspace_graph_materialization_hooks.sh`; `tests/integration/workspace/workspace_graph_read_surfaces.sh`; `tests/integration/workspace/workspace_brain_graph_transient.sh` | graph node/edge files + indices created; graph query families return db-first payloads | PASS | none | n/a |
| Workspace switch + recovery/load semantics verified | `yai` | active workspace switch changes runtime scope; recovery state is surfaced | `tests/integration/workspace/workspace_runtime_contract.sh`; `tests/integration/workspace/workspace_session_binding_contract.sh` | cross-workspace calls denied until switch; inspect exposes `runtime_capabilities.recovery` with tracked state | PASS | none | n/a |
| Registry/primitives/governable objects aligned | `yai-law` | family ownership aligns to unified runtime | `make check`; `make validate-law-registry` | all checks `OK` | PASS | none | n/a |
| Runtime entrypoints/constraints/manifests/schemas aligned | `yai-law` | workspace-first + canonical families; disallowed subsystem targets | inspect `manifests/runtime.entrypoints.json`, `manifests/governance-attachability.constraints.v1.json`, `schema/workspace_governance_attachment.v1.schema.json` | explicit canonical families + disallowed `brain/mind` as canonical | PASS | none | n/a |
| Law docs free of active legacy topology truth | `yai-law` | no active canonical brain/mind topology in authoritative path | grep on docs + runtime surface markers | active docs aligned; legacy mentions are explicitly historical/de-authorized | PASS | non-canonical historical subtree retained outside active path | law archival hygiene |
| DX terminology contract reflected across repos | `yai`,`law`,`sdk`,`cli` | runtime/workspace/binding/readiness/family vocabulary converged | cross-repo grep on active docs + contract docs | terms align to contract; no major active contradiction | PASS | none | n/a |
| Architecture docs aligned cross-repo | `yai`,`law`,`sdk`,`cli` | same unified runtime model | inspect top architecture docs in each repo | no competing active topology narrative | PASS | none | n/a |
| Operational docs aligned cross-repo | `yai`,`law`,`sdk`,`cli` | runbooks/guides/checklists teach unified model | scan active runbooks/guides + spot checks | workspace-first + capability-aware semantics in active paths | PASS | none | n/a |
| SDK public surface aligned | `yai-sdk` | unified runtime family public surface | `make test` + surface docs check | `public_surface_smoke: ok` and canonical headers/docs | PASS | none | n/a |
| SDK models aligned | `yai-sdk` | workspace-first + readiness/binding model truth | `make test` | `models_contract_smoke: ok` | PASS | none | n/a |
| SDK examples/smoke path aligned | `yai-sdk` | examples/quickstart use corrected surface/models | `make test` | `sdk_smoke`, `workspace_smoke`, `help_index_smoke` pass | PASS | none | n/a |
| CLI taxonomy/help aligned | `yai-cli` | command/help teach unified runtime | `make test` (help guardrails) | `help_guardrail: ok`, `porcelain_help_guardrail: ok` | PASS | hidden compatibility aliases are non-canonical and excluded from primary help | cli alias sunset (non-blocking) |
| CLI output/status/inspect aligned | `yai-cli` | liveness vs readiness; selected vs bound; family visibility | `make test` (output/operator/workspace guardrails) | guardrails green incl. workspace/status/runtime sections | PASS | none | n/a |
| CLI docs/examples/smoke/guardrails aligned | `yai-cli` | operator truth path coherent | `make test` + docs scan | all CLI guardrails pass and docs updated | PASS | none | n/a |

## Command evidence snapshot (executed 2026-03-10)

- `yai-sdk`: `make test` -> PASS
- `yai-cli`: `make test` -> PASS
- `yai-law`: `make check` and `make validate-law-registry` -> PASS
- `yai`: hard verification block PASS:
  - `workspace_session_binding_contract.sh`
  - `workspace_inspect_surfaces.sh`
  - `workspace_real_flow.sh`
  - `workspace_runtime_contract.sh`
  - `workspace_db_first_read_cutover.sh`
  - `workspace_event_evidence_sink_hardening.sh`
  - `workspace_governance_persistence.sh`
  - `workspace_authority_artifact_persistence.sh`
  - `workspace_brain_graph_transient.sh`
  - `workspace_graph_materialization_hooks.sh`
  - `workspace_graph_read_surfaces.sh`

## Blocker classification

- Blocking contradictions in active canonical truth path: **none identified**
- Non-blocking transitional/historical/test debt: **present only outside canonical active truth path**
