---
id: RB-WSV-2-RUNTIME-MAPPING-CANONICALIZATION
status: active
owner: runtime-cli-sdk-law
related:
  - docs/program/milestone-packs/runtime-baselines/mp-runtime-000-workspace-command-taxonomy-architecture.md
  - docs/program/reports/workspace-verticalization-report.md
effective_date: 2026-03-10
---

# WSV-2 — Runtime Command Mapping and Canonical Path Plan

## 1) Runtime truth baseline

This mapping is grounded in implemented runtime/consumer surfaces, not desired-only taxonomy.

Primary runtime loci used:
- `lib/core/session/session.c`
  - command ID discovery + dispatch routing for `yai.workspace.*`
  - runtime handling for lifecycle/domain/policy/debug/graph/query/lifecycle
- `lib/core/session/utils/session_utils_surface_core.inc.c`
  - status/inspect payload structure (runtime capabilities, read-path, persistence refs)
- `lib/core/session/utils/session_utils_surface_views.inc.c`
  - workspace query family result generation (`yai.workspace.query.result.v1`)
- `lib/core/session/utils/session_utils_lifecycle_execution.inc.c`
  - lifecycle status/maintain payloads

Current CLI exposure loci:
- `/Users/francescomaiomascio/Developer/YAI/cli/src/parse/parse.c`
- `/Users/francescomaiomascio/Developer/YAI/cli/src/help/help.c`
- `/Users/francescomaiomascio/Developer/YAI/cli/src/render/render.c`

Current SDK exposure loci:
- `/Users/francescomaiomascio/Developer/YAI/sdk/include/yai_sdk/workspace.h`
- `/Users/francescomaiomascio/Developer/YAI/sdk/include/yai_sdk/graph.h`
- `/Users/francescomaiomascio/Developer/YAI/sdk/include/yai_sdk/data.h`
- `/Users/francescomaiomascio/Developer/YAI/sdk/include/yai_sdk/knowledge.h`

Integration evidence used:
- `tests/integration/workspace/workspace_graph_read_surfaces.sh`
- `tests/integration/workspace/workspace_db_first_read_cutover.sh`
- `tests/integration/workspace/workspace_graph_materialization_hooks.sh`
- `tests/integration/workspace/workspace_session_binding_contract.sh`
- `tests/integration/workspace/workspace_runtime_contract.sh`

## 2) Canonicalization decision statement

- Canonical operator-facing grammar is `yai ws ...`.
- Raw runtime IDs (`yai.workspace.*`) are implementation substrate, not preferred user grammar.
- Underscore/dotted variants are compatibility-only where duplicated.
- First-class canonical families are preferred over `ws query` when capability is stable and workspace-meaningful.

Normative anchor from WSV-1:

> Workspace-bound runtime truth must surface primarily under `yai ws ...`.

## 3) Mapping status classes

- `canonical-ready`
  - Runtime exists and maps cleanly to canonical path.
- `canonical-via-alias`
  - Runtime exists but canonical CLI grammar differs from internal naming.
- `fallback-only`
  - Keep generic/fallback exposure for now.
- `promote-next`
  - Runtime exists and should become first-class in WSV-3/WSV-4.
- `runtime-only`
  - Runtime exists but not selected/stable for canonical exposure yet.
- `deprecate-path`
  - Existing non-canonical path retained only temporarily.

## 4) Full runtime-to-canonical mapping matrix

Authoritative matrix:
- [workspace-runtime-command-mapping-matrix.md](/Users/francescomaiomascio/Developer/YAI/yai/docs/program/reports/workspace-verticalization-report.md)

The matrix provides, per row:
- runtime command id
- runtime/CLI/SDK availability
- canonical CLI target path
- canonical family
- disposition
- action required
- follow-up owner

Coverage included:
- lifecycle/binding/status/inspect
- domain
- policy
- debug
- graph
- data/query families
- db/store composed surfaces
- recovery/lifecycle surfaces

## 5) Alias and normalization policy

### 5.1 Canonical grammar normalization

- `yai.workspace.debug_resolution` -> `yai ws debug resolution`
- `yai.workspace.domain_get` -> `yai ws domain get`
- `yai.workspace.domain_set` -> `yai ws domain set`
- `yai.workspace.policy_dry_run` -> `yai ws policy dry-run`

### 5.2 Compatibility-only variants

Retained runtime compatibility forms (non-canonical grammar):
- dotted domain/policy/debug variants (`domain.get`, `domain.set`, `policy.attach`, `policy.detach`, `policy.effective`, `debug.resolution`)
- underscore runtime IDs used by existing parser routes

Policy:
- canonical help/docs/examples use nested `ws <family> <subcommand>` only.
- compatibility variants remain accepted but de-authorized from primary help.

## 6) Primary-path vs fallback-path mapping

### A) First-class canonical families (direct operator path)

- `ws` lifecycle and binding: `create/open/set/switch/current/status/inspect/unset/clear/reset/destroy`
- `ws graph`: `summary/workspace/governance/decision/evidence/authority/artifact/lineage/recent`
- `ws policy`: `attach/detach/activate/dry-run/effective`
- `ws domain`: `get/set`
- `ws debug`: `resolution`
- `ws recovery`: `status/load/reopen` (per plan below)

### B) Fallback generic query (short-term)

Remain fallback/substrate until promoted:
- `ws data events/evidence/governance/authority/artifacts/enforcement`
- `ws knowledge transient/memory`
- `ws db classes/count/tail` early phase projections

Rationale:
- runtime substrate already supports them via `yai.workspace.query` families,
- canonical commands should wrap substrate with stable operator grammar (WSV-3),
- avoid exposing family selector internals as primary UX.

## 7) DB/store command plan

Target commands and current backing:

- `yai ws db status`
  - Runtime basis now: `yai.workspace.status` + `yai.workspace.inspect` (`runtime_capabilities.data`, `read_path`).
  - Direct runtime ID exists: no.
  - WSV-3: CLI composition command (canonical).

- `yai ws db bindings`
  - Runtime basis now: status binding fields + inspect `root_model` and `runtime_capabilities.workspace_binding`.
  - Direct runtime ID exists: no.
  - WSV-3: CLI composition; WSV-4 typed SDK model.

- `yai ws db stores`
  - Runtime basis now: inspect persistence refs (`*_store_ref` across event/evidence/governance/authority/artifact/graph/transient).
  - Direct runtime ID exists: no.
  - WSV-3: CLI projection command from inspect.

- `yai ws db classes`
  - Runtime basis now: query-family availability inventory.
  - Direct runtime ID exists: no.
  - Near-term status: fallback-backed synthetic projection.

- `yai ws db count`
  - Runtime basis now: query-family aggregate counts.
  - Direct runtime ID exists: no.
  - Near-term status: fallback-backed aggregate; later dedicated runtime ID optional.

- `yai ws db tail`
  - Runtime basis now: `events.tail` + query-family tail semantics.
  - Direct runtime ID exists: partial (`yai.workspace.events.tail`).
  - Near-term status: canonical wrapper over existing tail/read substrate.

Summary:
- `ws db` can be exposed in WSV-3 mostly through composition over existing status/inspect/query substrate.
- New runtime IDs are optional for performance/strict typing, not required for initial canonicalization.

## 8) Recovery command plan

Target recovery mapping:

- `yai ws recovery status`
  - Runtime basis now: `status/current/inspect` recovery fields (`runtime_capabilities.recovery`, `declared_context_source`, read-path state).
  - Direct runtime ID exists: no dedicated `recovery.status`.
  - WSV-3: expose as canonical composed command.

- `yai ws recovery load`
  - Runtime basis now: lifecycle maintenance substrate (`yai.workspace.lifecycle.maintain`) + open/set semantics.
  - Direct runtime ID exists: partial (lifecycle maintain).
  - Disposition: runtime-only until UX contract is fixed.

- `yai ws recovery reopen`
  - Runtime basis now: `yai.workspace.open`.
  - Direct runtime ID exists: yes (`workspace.open`).
  - WSV-3: canonical command maps to runtime open.

## 9) Runtime gaps and non-blocking deferrals

Known gaps from matrix:
- Graph commands are runtime-implemented but not first-class CLI-exposed (`promote-next`).
- Data/knowledge families are runtime-accessible through query substrate but not canonical CLI families (`promote-next`).
- DB family lacks dedicated runtime IDs; canonical surface should start as composition (`promote-next` / `fallback-only`).
- Recovery family lacks a dedicated full trio of runtime IDs (`status/load/reopen`); `status/reopen` can be exposed now, `load` remains runtime-only until behavior contract is stabilized.
- Registry drift exists (law currently exposes only subset of workspace IDs); WSV-5 owns canonicalization.

None of these block WSV-3 CLI parser/help implementation, because canonical targets and dispositions are now explicit.

## 10) Cross-repo action map

### WSV-3 (CLI)
- Add canonical families/subcommands:
  - `ws graph *`
  - `ws data *`
  - `ws knowledge *`
  - `ws db *`
  - `ws recovery *`
- Add `ws open` and normalize `switch` alias semantics.
- Keep dotted/underscore IDs as compatibility only; do not expose as primary help.

### WSV-4 (SDK)
- Add typed helpers for graph/data/knowledge/db/recovery surfaces.
- Reduce reliance on raw command constants for common reads.
- Keep raw IDs internal-facing where possible.

### WSV-5 (law registry)
- Align `commands.v1.json` with canonical workspace taxonomy coverage.
- Mark deprecated dotted/underscore variants as compatibility/transitional where retained.

### WSV-6 (docs/help/smoke)
- Rewrite command examples to canonical `ws` families.
- Add smoke coverage for new verticalized `ws` families and fallback paths.

## 11) Required treatment of known findings

Applied directly in this mapping:
- Runtime has wider workspace capability than current CLI exposure.
- Graph family is runtime-ready and explicitly marked `promote-next` to first-class CLI.
- DB/store inspectability is runtime-real and mapped into `ws db` composition plan.
- SDK currently leans on constants/query substrate and is mapped for typed promotion.
- Law registry drift is identified and assigned to WSV-5.

## 12) Verification

### A. Coverage verification
All major workspace-bound areas from WSV-1 are classified with one disposition class.

### B. Canonical-path verification
Graph/data/policy/domain/debug/recovery surfaces have deterministic canonical `yai ws ...` targets.

### C. Ambiguity verification
Ownership boundaries are explicit:
- `ws graph` relational truth,
- `ws data` record views,
- `ws db` storage/binding/store diagnostics,
- `ws knowledge` transient/memory/provider support,
- `ws query` fallback only.

### D. Downstream readiness verification
WSV-3 can proceed mechanically from matrix rows without re-deciding grammar or command ownership.

## 13) Final statement

WSV-2 closes the taxonomy-to-runtime bridge:
- runtime truth mapped,
- canonical paths assigned,
- aliases normalized,
- first-class vs fallback decided,
- DB and recovery plans made explicit,
- downstream owners assigned.

This is an implementation-driving contract, not a design note.

## Appendix A — Inline mapping matrix snapshot

The full matrix below is intentionally duplicated here so WSV-3 can execute from this runbook without external reinterpretation.

| runtime command id | current runtime availability | current CLI exposure | current SDK exposure | canonical CLI target path | canonical family | disposition | action required | follow-up owner |
|---|---|---|---|---|---|---|---|---|
| `yai.workspace.create` | yes (direct dispatch) | yes (`ws create`) | no const/helper | `yai ws create` | lifecycle | canonical-ready | keep canonical | WSV-3 CLI docs/help sync |
| `yai.workspace.open` | yes (direct dispatch) | no direct `ws open` | no const/helper | `yai ws open` | lifecycle | promote-next | add parser/help/renderer mapping | WSV-3 CLI |
| `yai.workspace.set` | yes | yes (`ws set`) | const (`YAI_SDK_CMD_WORKSPACE_SET`) + bind helper | `yai ws set` | lifecycle | canonical-ready | keep canonical | WSV-3/WSV-4 |
| `yai.workspace.switch` | yes | yes (`ws switch`) | switch helper (context) | `yai ws switch` (alias to `set`) | lifecycle | canonical-via-alias | document alias=set; keep behavior identical | WSV-3 CLI |
| `yai.workspace.current` | yes | yes (`ws current`) | no const/helper | `yai ws current` | lifecycle | canonical-ready | add SDK const/helper | WSV-4 SDK |
| `yai.workspace.status` | yes | yes (`ws status`) | const + helper | `yai ws status` | lifecycle | canonical-ready | keep canonical | WSV-3/WSV-4 |
| `yai.workspace.inspect` | yes | yes (`ws inspect`) | const | `yai ws inspect` | lifecycle | canonical-ready | keep canonical | WSV-3/WSV-4 |
| `yai.workspace.unset` | yes | yes (`ws unset`) | const + unbind helper | `yai ws unset` | lifecycle | canonical-ready | keep canonical | WSV-3/WSV-4 |
| `yai.workspace.clear` | yes | yes (`ws clear`) | clear binding helper only | `yai ws clear` | lifecycle | canonical-ready | keep canonical | WSV-3/WSV-4 |
| `yai.workspace.reset` | yes | yes (`ws reset`) | no const/helper | `yai ws reset` | lifecycle | canonical-ready | add SDK const/helper | WSV-4 SDK |
| `yai.workspace.destroy` | yes | yes (`ws destroy`) | no const/helper | `yai ws destroy` | lifecycle | canonical-ready | add SDK const/helper | WSV-4 SDK |
| `yai.workspace.domain_get` | yes | yes (`ws domain get`) | no const/helper | `yai ws domain get` | domain | canonical-via-alias | canonicalize to nested grammar | WSV-3 CLI + WSV-5 law |
| `yai.workspace.domain.get` | yes | no direct | no | `yai ws domain get` | domain | deprecate-path | keep internal compatibility only | WSV-3 CLI + WSV-5 law |
| `yai.workspace.domain_set` | yes | yes (`ws domain set`) | no | `yai ws domain set` | domain | canonical-via-alias | canonical nested grammar; keep runtime id | WSV-3 CLI + WSV-5 law |
| `yai.workspace.domain.set` | yes | no direct | no | `yai ws domain set` | domain | deprecate-path | compatibility only; remove from canonical docs | WSV-5 law + WSV-6 docs |
| `yai.workspace.policy_attach` | yes | yes (`ws policy attach`) | const | `yai ws policy attach` | policy | canonical-ready | keep canonical | WSV-3/WSV-4 |
| `yai.workspace.policy_activate` | yes | yes (`ws policy activate`) | const | `yai ws policy activate` | policy | canonical-ready | keep canonical | WSV-3/WSV-4 |
| `yai.workspace.policy_detach` | yes | yes (`ws policy detach`) | const | `yai ws policy detach` | policy | canonical-ready | keep canonical | WSV-3/WSV-4 |
| `yai.workspace.policy_dry_run` | yes | yes (`ws policy dry-run`) | const | `yai ws policy dry-run` | policy | canonical-via-alias | hyphenated CLI grammar stays canonical | WSV-3 CLI + WSV-5 law |
| `yai.workspace.policy_effective` | yes | yes (`ws policy effective`) | no const/helper | `yai ws policy effective` | policy | canonical-ready | add SDK const/helper | WSV-4 SDK |
| `yai.workspace.policy.attach` | yes | no direct | no | `yai ws policy attach` | policy | deprecate-path | keep runtime compat only | WSV-5 law |
| `yai.workspace.policy.detach` | yes | no direct | no | `yai ws policy detach` | policy | deprecate-path | keep runtime compat only | WSV-5 law |
| `yai.workspace.policy.effective` | yes | no direct | no | `yai ws policy effective` | policy | deprecate-path | keep runtime compat only | WSV-5 law |
| `yai.workspace.debug_resolution` | yes | yes (`ws debug resolution`) | no const/helper | `yai ws debug resolution` | debug | canonical-via-alias | canonical nested grammar | WSV-3 CLI + WSV-5 law |
| `yai.workspace.debug.resolution` | yes | no direct | no | `yai ws debug resolution` | debug | deprecate-path | runtime compat only | WSV-5 law |
| `yai.workspace.graph.summary` | yes | no direct | const (`YAI_SDK_CMD_WORKSPACE_GRAPH_SUMMARY`) | `yai ws graph summary` | graph | promote-next | add CLI subcommand and renderer route | WSV-3 CLI |
| `yai.workspace.graph.workspace` | yes | no direct | no | `yai ws graph workspace` | graph | promote-next | add CLI subcommand | WSV-3 CLI |
| `yai.workspace.graph.governance` | yes | no direct | no | `yai ws graph governance` | graph | promote-next | add CLI subcommand | WSV-3 CLI |
| `yai.workspace.graph.decision` | yes | no direct | no | `yai ws graph decision` | graph | promote-next | add CLI subcommand | WSV-3 CLI |
| `yai.workspace.graph.evidence` | yes | no direct | no | `yai ws graph evidence` | graph | promote-next | add CLI subcommand | WSV-3 CLI |
| `yai.workspace.graph.authority` | yes | no direct | no | `yai ws graph authority` | graph | promote-next | add CLI subcommand | WSV-3 CLI |
| `yai.workspace.graph.artifact` | yes | no direct | no | `yai ws graph artifact` | graph | promote-next | add CLI subcommand | WSV-3 CLI |
| `yai.workspace.graph.lineage` | yes | no direct | no | `yai ws graph lineage` | graph | promote-next | add CLI subcommand | WSV-3 CLI |
| `yai.workspace.graph.recent` | yes | no direct | no | `yai ws graph recent` | graph | promote-next | add CLI subcommand | WSV-3 CLI |
| `yai.workspace.query` + `events` | yes (family via query) | no direct `ws query` grammar | query const + data/knowledge family constants | `yai ws data events` | data | promote-next | add canonical subcommand over query substrate | WSV-3 CLI + WSV-4 SDK |
| `yai.workspace.query` + `evidence` | yes | no direct | query const + data family | `yai ws data evidence` | data | promote-next | canonical wrapper over query | WSV-3 CLI + WSV-4 SDK |
| `yai.workspace.query` + `governance` | yes | no direct | query const + data family | `yai ws data governance` | data | promote-next | canonical wrapper over query | WSV-3 CLI + WSV-4 SDK |
| `yai.workspace.query` + `authority` | yes | no direct | query const + data family | `yai ws data authority` | data | promote-next | canonical wrapper over query | WSV-3 CLI + WSV-4 SDK |
| `yai.workspace.query` + `artifact` | yes | no direct | query const + data family | `yai ws data artifacts` | data | promote-next | canonical wrapper over query | WSV-3 CLI + WSV-4 SDK |
| `yai.workspace.query` + `enforcement` | yes | no direct | query const + data family | `yai ws data enforcement` | data | promote-next | canonical wrapper over query | WSV-3 CLI + WSV-4 SDK |
| `yai.workspace.query` + `transient` | yes | no direct | query const + knowledge family | `yai ws knowledge transient` | knowledge | promote-next | canonical wrapper over query | WSV-3 CLI + WSV-4 SDK |
| `yai.workspace.query` + `memory` | yes | no direct | query const + knowledge family | `yai ws knowledge memory` | knowledge | promote-next | canonical wrapper over query | WSV-3 CLI + WSV-4 SDK |
| `yai.workspace.governance.list` | yes | no direct | no | `yai ws data governance` | data | canonical-via-alias | allow as internal runtime alias | WSV-3 CLI + WSV-5 law |
| `yai.workspace.events.tail` | yes | no direct | no | `yai ws data events` / `yai ws db tail` | data/db | canonical-via-alias | choose primary target in WSV-3 (`data events`) | WSV-3 CLI |
| `yai.workspace.evidence.list` | yes | no direct | no | `yai ws data evidence` | data | canonical-via-alias | internal alias only | WSV-3 CLI + WSV-5 law |
| `yai.workspace.authority.list` | yes | no direct | no | `yai ws data authority` | data | canonical-via-alias | internal alias only | WSV-3 CLI + WSV-5 law |
| `yai.workspace.artifacts.list` | yes | no direct | no | `yai ws data artifacts` | data | canonical-via-alias | internal alias only | WSV-3 CLI + WSV-5 law |
| `yai.workspace.enforcement.status` | yes | no direct | no | `yai ws data enforcement` | data | canonical-via-alias | internal alias only | WSV-3 CLI + WSV-5 law |
| `yai.workspace.lifecycle.model` | yes (query family=lifecycle) | no direct | no | `yai ws recovery status` | recovery | runtime-only | expose later if semantics stabilized | later runtime + WSV-3 |
| `yai.workspace.lifecycle.status` | yes | no direct | no | `yai ws recovery status` | recovery | promote-next | canonical wrapper + renderer | WSV-3 CLI |
| `yai.workspace.lifecycle.maintain` | yes | no direct | no | `yai ws recovery load` | recovery | runtime-only | needs UX contract before exposure | later runtime wave |
| `yai.workspace.run` | yes | yes (`ws run`) | const | `yai ws run` | exec/workspace | canonical-ready | keep canonical | WSV-3 docs/help |
| `yai.workspace.prompt_context` | yes | yes (`ws prompt-context`/`prompt-token`) | no | `yai ws inspect` adjunct | inspect | runtime-only | keep utility, not family anchor | WSV-6 docs |
| *(composed)* `status+inspect` recovery fields | yes (indirect in payload) | yes via existing commands | yes via inspect/status models | `yai ws recovery status` | recovery | promote-next | add command composition now | WSV-3 CLI |
| *(composed)* `open` (+lifecycle status) | yes | partial (`ws set/switch`, no `ws open`) | partial | `yai ws recovery reopen` | recovery | canonical-via-alias | map to `workspace.open` | WSV-3 CLI |
| *(composed)* query+store refs | yes (inspect/query) | no direct | partial (query const) | `yai ws db status` | db | promote-next | implement CLI composition using inspect/status/query | WSV-3 CLI |
| *(composed)* inspect refs (`*_store_ref`) | yes | no direct | no typed helper | `yai ws db stores` | db | promote-next | add CLI renderer and SDK typed reader | WSV-3 CLI + WSV-4 SDK |
| *(composed)* status+inspect binding fields | yes | no direct | no typed helper | `yai ws db bindings` | db | promote-next | compose from `binding_status/runtime_capabilities/read_path` | WSV-3 CLI + WSV-4 SDK |
| *(composed)* query family inventory | yes | no direct | constants only | `yai ws db classes` | db | fallback-only | start as synthetic projection from query families | WSV-3 CLI |
| *(composed)* query counts | yes | no direct | constants only | `yai ws db count` | db | fallback-only | start with query-backed aggregate | WSV-3 CLI |
| *(composed)* `events.tail` + query families | yes | no direct | constants only | `yai ws db tail` | db | fallback-only | short-term: query-based tail; later dedicated ID | WSV-3 CLI + later runtime |

