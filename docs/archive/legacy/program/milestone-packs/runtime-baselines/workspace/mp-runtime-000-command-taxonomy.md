---
id: RB-WSV-1-WORKSPACE-COMMAND-TAXONOMY
status: active
owner: runtime-cli-sdk-governance
effective_date: 2026-03-10
decisions:
  - docs/program/adr/adr-runtime-001-single-runtime.md
  - docs/program/adr/adr-workspace-007-workspace-isolation.md
  - docs/program/adr/adr-program-012-audit-convergence-gates.md
depends_on:
  - docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-runtime-topology.md
  - docs/architecture/cross-repo-naming-and-terminology-contract.md
tags:
  - workspace
  - taxonomy
  - cli
  - sdk
  - governance
  - canonicalization
---

# WSV-1 — Canonical Workspace Command Taxonomy Refoundation

## 1) Canonical command design statement

`yai ws ...` is the canonical primary operator/consumer path for any workspace-bound runtime capability.

Normative rule:

> Any runtime capability whose truth is bound to an active workspace must have its primary canonical operator path under `yai ws ...`.

This includes both:
- lifecycle/binding operations (`create`, `set`, `open`, `status`, `inspect`, ...),
- read/inspect/query surfaces (`graph`, `db`, `data`, `knowledge`, `policy`, `recovery`, `debug`).

Policy implications:
- Generic substrate query remains available as fallback (`yai ws query ...`).
- Generic fallback is not canonical UX where a stable first-class family exists.
- Top-level command families must not absorb workspace-bound surfaces that logically belong under `ws`.

## 2) Canonical workspace command tree (target grammar)

```text
yai ws
├── create
├── open
├── set
├── switch            # canonical alias of `set`
├── current
├── status
├── inspect
├── unset
├── clear
├── reset
├── destroy
├── graph
│   ├── summary
│   ├── workspace
│   ├── governance
│   ├── decision
│   ├── evidence
│   ├── authority
│   ├── artifact
│   ├── lineage
│   └── recent
├── db
│   ├── status
│   ├── bindings
│   ├── stores
│   ├── classes
│   ├── count
│   └── tail
├── data
│   ├── events
│   ├── evidence
│   ├── governance
│   ├── authority
│   ├── artifacts
│   └── enforcement
├── knowledge
│   ├── status
│   ├── transient
│   ├── memory
│   ├── providers
│   └── context
├── policy
│   ├── attach
│   ├── detach
│   ├── activate
│   ├── dry-run
│   └── effective
├── domain
│   ├── get
│   └── set
├── recovery
│   ├── status
│   ├── load
│   └── reopen
├── debug
│   └── resolution
└── query             # generic fallback, non-primary
    └── <family>
```

## 3) Command family responsibility split

- `ws graph`
  - Graph truth and relational read surfaces bound to active workspace.
  - Neighborhood/lineage/authority/evidence/governance/decision projections.

- `ws db`
  - Storage-layer and binding health for workspace data plane.
  - Store registry/classes/bindings/count/tail diagnostics.

- `ws data`
  - Persisted runtime records and record-derived surfaces.
  - Event/evidence/governance/authority/artifact/enforcement reads.

- `ws knowledge`
  - Workspace-bound knowledge support state.
  - Transient/memory/provider/context visibility.

- `ws policy`
  - Workspace governance attachment/effective policy operations.
  - Attach/detach/activate/dry-run/effective.

- `ws domain`
  - Workspace domain read/write (`get`, `set`).

- `ws recovery`
  - Recovery/load/reopen status and actions.

- `ws debug`
  - Resolution/debug surfaces.

- `ws query`
  - Generic low-level fallback for capabilities not yet promoted.
  - Non-canonical for stable operator workflows.

## 4) Primary-path vs fallback-path policy

A capability must be promoted to first-class `ws <family> <subcommand>` when at least one applies:
- already implemented in runtime and used in tests/smokes,
- stable enough for operator use,
- semantically important for workspace inspectability,
- repeatedly used by humans or SDK consumers,
- clearer as named command than as family selector argument.

A capability may stay under `ws query` only if it is:
- transitional,
- too raw/low-level,
- unstable,
- substrate-generic and not yet suitable as stable UX.

Promotion rule:
- Once a dedicated first-class path exists, docs/help/examples must use it as primary.
- `ws query` remains compatibility/substrate escape hatch.

## 5) Naming and grammar rules

Canonical CLI grammar rules:
- Use nested, human-readable grammar under `ws`.
  - Examples: `ws graph summary`, `ws policy dry-run`, `ws domain get`.
- Avoid exposing internal runtime IDs as user grammar.
  - Internal: `yai.workspace.graph.summary`
  - Canonical CLI: `yai ws graph summary`
- Prefer hyphenated user verbs where needed (`dry-run`) over underscore IDs.
- Keep compatibility aliases thin and transitional.

Grammar layers (must stay distinct):
- Internal runtime command IDs: `yai.workspace.*`
- Canonical CLI grammar: `yai ws ...`
- Compatibility aliases: accepted temporarily, hidden from primary help when possible.
- Deprecated grammar: supported only for migration window; never primary docs/help path.

Alias policy for WSV program:
- `ws switch` is canonical alias to `ws set` (identical behavior target).
- Dot/underscore legacy forms may route internally but must not be the canonical grammar.

## 6) Runtime-to-canonical mapping examples (implementation-grade)

The table below defines mapping anchors for WSV-2 runtime mapping and WSV-4 CLI grammar implementation.

| Runtime command ID / substrate | Canonical CLI grammar | Classification |
|---|---|---|
| `yai.workspace.graph.summary` | `yai ws graph summary` | direct canonical |
| `yai.workspace.graph.workspace` | `yai ws graph workspace` | direct canonical |
| `yai.workspace.graph.governance` | `yai ws graph governance` | direct canonical |
| `yai.workspace.graph.decision` | `yai ws graph decision` | direct canonical |
| `yai.workspace.graph.evidence` | `yai ws graph evidence` | direct canonical |
| `yai.workspace.graph.authority` | `yai ws graph authority` | direct canonical |
| `yai.workspace.graph.artifact` | `yai ws graph artifact` | direct canonical |
| `yai.workspace.graph.lineage` | `yai ws graph lineage` | direct canonical |
| `yai.workspace.graph.recent` | `yai ws graph recent` | direct canonical |
| `yai.workspace.query` + `evidence` | `yai ws data evidence` | candidate promotion |
| `yai.workspace.query` + `governance` | `yai ws data governance` | candidate promotion |
| `yai.workspace.query` + `authority` | `yai ws data authority` | candidate promotion |
| `yai.workspace.query` + `artifact` | `yai ws data artifacts` | candidate promotion |
| `yai.workspace.query` + `enforcement` | `yai ws data enforcement` | candidate promotion |
| `yai.workspace.query` + `events` | `yai ws data events` | candidate promotion |
| `yai.workspace.query` + `transient` | `yai ws knowledge transient` | candidate promotion |
| `yai.workspace.query` + `memory` | `yai ws knowledge memory` | candidate promotion |
| `yai.workspace.policy_attach` | `yai ws policy attach` | direct canonical |
| `yai.workspace.policy_detach` | `yai ws policy detach` | direct canonical |
| `yai.workspace.policy_activate` | `yai ws policy activate` | direct canonical |
| `yai.workspace.policy_dry_run` | `yai ws policy dry-run` | direct canonical |
| `yai.workspace.policy_effective` | `yai ws policy effective` | direct canonical |
| `yai.workspace.domain_get` | `yai ws domain get` | direct canonical |
| `yai.workspace.domain_set` | `yai ws domain set` | direct canonical |
| `yai.workspace.debug_resolution` | `yai ws debug resolution` | direct canonical |
| `yai.workspace.query` + `graph.workspace` | `yai ws graph workspace` | candidate replacement |

Notes:
- Several graph/policy/domain/debug runtime IDs already exist in `yai` dispatch surfaces.
- Registry drift is expected at this stage; WSV-3 canonicalizes registry truth against this taxonomy.
- Candidate promotions remain valid even when currently exposed only via generic query.

## 7) Non-canonical paths and anti-patterns

The following must not be the primary operator truth path for workspace-bound surfaces:
- top-level `yai graph ...` as primary workspace graph read path,
- top-level `yai db ...` as primary workspace-bound read path,
- exposing only `ws query <family>` for stable graph/data/knowledge reads,
- exposing raw runtime command IDs as canonical user grammar,
- duplicating the same workspace-bound capability across multiple primary top-level families.

Anti-sprawl rule:
- Workspace-bound truth has one canonical home: `yai ws ...`.
- Any secondary path must be explicitly marked alias/fallback/deprecated.

## 8) Cross-repo impact and downstream ownership

This taxonomy is normative input for:
- `yai` (WSV-2): runtime command mapping and canonical path plan.
- `yai-governance` (WSV-3): registry canonicalization (`commands.v1.json` and linked surfaces).
- `yai-cli` (WSV-4): parser/help/taxonomy/output integration under `ws` families.
- `yai-sdk` (WSV-5): typed helpers and client model exposure by `ws` families.
- docs/smokes/guardrails (WSV-6): canonical examples and validation path.
- closeout verification (WSV-7): implemented vs exposed vs tested matrix.

## 9) Verification (implementation readiness for WSV-2)

### A. Coverage verification
The canonical tree covers:
- lifecycle/binding,
- graph,
- db,
- data,
- knowledge,
- policy,
- domain,
- recovery,
- debug,
- query fallback.

### B. Runtime relevance verification
The taxonomy is grounded in currently implemented runtime IDs and active integration usage, including:
- `yai.workspace.graph.*`,
- `yai.workspace.policy_*`,
- `yai.workspace.domain_*`,
- `yai.workspace.debug_resolution`,
- `yai.workspace.query` families used in workspace integration suites.

### C. Separation-of-concerns verification
This contract explicitly separates:
- graph vs data,
- db vs data,
- knowledge vs exec,
- policy vs data,
- recovery vs inspect.

### D. Next-step readiness verification
The taxonomy is specific enough for mechanical execution:
- WSV-2 can map runtime IDs to canonical grammar and classify gaps.
- WSV-4 can implement parser/help from canonical tree directly.
- WSV-5 can shape SDK helpers and typed wrappers from this tree.
- WSV-3 can align governance registry command truth to canonical CLI grammar.

## 10) Scope boundary for WSV-1

WSV-1 defines the command contract only.
It does not perform:
- full runtime mapping/refactor,
- full CLI grammar implementation,
- SDK helper/model implementation,
- governance registry rewrite.

Those are explicit downstream tasks (WSV-2..WSV-7).

## 11) Final normative statement

`yai ws ...` is now the canonical command taxonomy boundary for workspace-bound runtime capabilities.
`ws query` remains a substrate fallback, not primary UX.

This runbook is the implementation contract for WSV-2 and must be interpreted as normative, not advisory.
