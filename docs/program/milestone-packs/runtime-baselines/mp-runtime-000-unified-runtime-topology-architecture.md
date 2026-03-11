# RF-1 — Unified Runtime Topology Target and Namespace Kill Plan

Status: active, implementation-contract.
Scope: `yai`.

## 1. Architectural Decision Statement

YAI no longer treats `brain` as a canonical runtime subsystem.

Canonical runtime families are:
- `core`
- `exec`
- `data`
- `graph`
- `knowledge`
- `law`
- `platform`
- `protocol`
- `support`

Binding decisions:
- Agents canonical placement: `exec`.
- Orchestration canonical placement: `exec`.
- Graph truth canonical placement: `graph`.
- Cognition/reasoning/memory/providers canonical placement: `knowledge`.
- Runtime startup/shutdown authority: `core` lifecycle + `core` workspace boundary.

This runbook is normative for RF-2..RF-8. RF-2 must be executable as a mechanical move/rename/delete refactor from this contract.

## 2. Canonical Target Tree (Implementation Target)

```text
.
├── cmd/
│   └── yai/main.c
├── include/yai/
│   ├── api/
│   ├── core/
│   ├── data/
│   │   ├── binding.h
│   │   ├── records.h
│   │   ├── query.h
│   │   ├── retention.h
│   │   ├── archive.h
│   │   └── store.h
│   ├── graph/
│   │   ├── graph.h
│   │   ├── ids.h
│   │   ├── lineage.h
│   │   ├── materialization.h
│   │   ├── query.h
│   │   └── summary.h
│   ├── knowledge/
│   │   ├── activation.h
│   │   ├── cognition.h
│   │   ├── episodic.h
│   │   ├── memory.h
│   │   ├── catalog.h
│   │   └── semantic.h
│   ├── exec/
│   │   ├── agents.h
│   │   ├── orchestration.h
│   │   ├── agent_binding.h
│   │   ├── runtime_bridge.h
│   │   ├── runtime_model.h
│   │   ├── network_policy.h
│   │   ├── provider_policy.h
│   │   ├── resource_policy.h
│   │   ├── runtime.h
│   │   ├── storage_policy.h
│   │   └── transport_client.h
│   ├── law/
│   ├── platform/
│   ├── protocol/
│   └── support/
├── lib/
│   ├── core/
│   │   ├── authority/
│   │   ├── dispatch/
│   │   ├── enforcement/
│   │   ├── lifecycle/
│   │   ├── session/
│   │   └── workspace/
│   │       ├── workspace_binding.c
│   │       └── workspace_recovery.c
│   ├── data/
│   │   ├── binding/
│   │   │   ├── store_binding.c
│   │   │   └── workspace_binding.c
│   │   ├── lifecycle/
│   │   │   ├── archive.c
│   │   │   ├── compaction.c
│   │   │   ├── integrity_checks.c
│   │   │   ├── pruning.c
│   │   │   ├── retention.c
│   │   │   └── rollup.c
│   │   ├── query/
│   │   ├── records/
│   │   └── store/
│   ├── graph/
│   │   ├── domains/
│   │   ├── materialization/
│   │   ├── query/
│   │   └── state/
│   ├── knowledge/
│   │   ├── cognition/
│   │   │   └── reasoning/
│   │   ├── memory/
│   │   └── providers/
│   ├── exec/
│   │   ├── agents/
│   │   ├── bridge/
│   │   ├── gates/
│   │   ├── orchestration/
│   │   ├── runtime/
│   │   └── transport/
│   ├── law/
│   ├── platform/
│   ├── protocol/
│   └── support/
└── docs/
    └── program/23-runbooks/
        └── unified-runtime-topology-refoundation.md
```

## 3. Canonical Namespace Families and Responsibility Split

- `core`: runtime base authority, workspace boundary resolution, lifecycle authority, enforcement, dispatch, session.
- `exec`: execution actors and orchestration, runtime routing, gates, transport, engine bridge.
- `data`: persisted records, query surfaces, store binding, retention/archive/compaction/pruning.
- `graph`: relational truth state, materialization from runtime records, lineage and summary views.
- `knowledge`: cognition substrate, reasoning, memory, provider substrate.

Non-acceptable split:
- agents under `knowledge`.
- orchestration under `knowledge`.
- graph under residual `brain` container.

## 4. Old → New Implementation Matrix

Normative implementation matrix is in:
- `docs/program/reports/runtime-convergence-report.md`

This annex is required for RF-2 mechanical execution and covers all current critical paths in:
- `include/yai/brain/*`
- `lib/brain/*`
- overlap paths already under `lib/exec/*`

## 5. Namespace Kill List

Kill list (container/system level):
- `include/yai/brain` directory and all public `brain` headers.
- `lib/brain` directory and all implementation modules under it.
- `brain_lifecycle` module as an autonomous lifecycle authority.
- runtime topology statements that define `brain` as canonical subsystem.
- naming that preserves `brain` as topological owner after relocation.

Kill list (conceptual):
- separate brain startup/shutdown authority.
- separate brain attach semantics.
- any execution placement outside `exec` for agents/orchestration.

Preserve but relocate:
- cognition/reasoning/memory/providers → `knowledge`.
- graph state/materialization/lineage/summaries → `graph`.
- runtime transport/bridge/agents/orchestration → `exec`.

## 6. Agents and Orchestration Placement (Binding Rule)

Agents:
- canonical family: `exec`
- implementation target: `lib/exec/agents/`
- public surface target: `include/yai/exec/agents.h`

Orchestration:
- canonical family: `exec`
- implementation target: `lib/exec/orchestration/`
- public surface target: `include/yai/exec/orchestration.h`

Overlap consolidation rule:
- existing `lib/exec/agents/*` remains authoritative and absorbs relocated `lib/brain/cognition/agents/*`.
- existing `lib/exec/runtime/*` and `lib/exec/bridge/*` remain authoritative and absorb relocated runtime bridge/transport components.

## 7. Workspace-First Binding Target

Target runtime behavior boundary is workspace-first.

For `yai up`, `yai ws create`, `yai ws open`, `yai ws set`, the runtime target binds:
- runtime state
- persistent stores
- graph truth state
- transient knowledge state
- recovery/load path

This requirement is mandatory for RF-5 and must not be blocked by namespace changes.

## 8. Lifecycle Collapse Target

- no separate lifecycle authority under `brain`.
- startup/shutdown lifecycle authority is `core/lifecycle`.
- capability activation is attached to runtime/workspace lifecycle boundaries.
- `brain_lifecycle.c` has no canonical future in target topology.

## 9. Cross-Repo Impact Statement

RF-1 topology contract impacts:
- `law`: runtime entrypoint naming and topology references.
- `sdk`: surface contracts and namespace references.
- `cli`: command taxonomy, help and output naming.
- docs cross-repo: architecture/program guides and migration pointers.
- tests: naming and include-path expectations in integration/unit suites.

## 10. Verification (RF-1 Contract Checks)

Tree target verification:
- target tree includes destination families for all ex-brain capability classes.

Mapping completeness verification:
- all critical `include/yai/brain/*` and `lib/brain/*` paths are mapped to target action in the annex.

Namespace kill verification:
- kill list is coherent with target tree and matrix actions.

Consolidation verification:
- agents converge in `exec`.
- orchestration converges in `exec`.
- graph does not remain under `brain`.
- knowledge does not absorb execution.

Downstream readiness verification:
- RF-2 can execute as move/rename/delete without redefining topology ownership.

## 11. Downstream Execution Contract

- RF-2: physical relocation and path rewrite (move/rename/delete/include build refs).
- RF-3: public headers and ABI surface rewrite.
- RF-4: lifecycle collapse implementation.
- RF-5: workspace-first initialization/store/graph/knowledge binding.
- RF-6: overlap consolidation and residue cleanup.
- RF-7: runtime status/surface upgrade.
- RF-8: final namespace/container deletion (`include/yai/brain`, `lib/brain`).

## 12. Final Statement

RF-1 is complete only as an implementation contract.
It is not a future-note and not a prose architecture memo.
RF-2 must be executable directly from this contract without reinterpretation.
