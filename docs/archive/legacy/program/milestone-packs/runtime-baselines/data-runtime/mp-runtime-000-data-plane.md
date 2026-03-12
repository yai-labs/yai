---

> Historical topology note: legacy labels (root/kernel/engine) may appear in historical artifacts.
> Canonical operator path is `cli -> sdk -> yai` on `~/.yai/run/control.sock`.
id: RB-DATA-PLANE
title: Data Plane Program
status: draft
owner: runtime
effective_date: 2026-03-09
revision: 4
supersedes:
  - RB-DATA-PLANE@rev3
  - RB-DATA-PLANE@rev2
  - RB-DATA-PLANE@rev1
depends_on:
  - RB-ROOT-HARDENING
  - RB-WORKSPACES-LIFECYCLE
  - RB-ENGINE-ATTACH
  - RB-MIND-REDIS-STM
adr_refs:
  - docs/program/adr/adr-protocol-006-unified-rpc.md
  - docs/program/adr/adr-workspace-007-workspace-isolation.md
  - docs/program/adr/adr-orchestration-009-engine-attachment.md
  - docs/program/adr/adr-contracts-011-contract-runbook-lock.md
  - docs/program/adr/adr-program-012-audit-convergence-gates.md
decisions:
  - docs/program/adr/adr-protocol-006-unified-rpc.md
  - docs/program/adr/adr-workspace-007-workspace-isolation.md
  - docs/program/adr/adr-contracts-011-contract-runbook-lock.md
  - docs/program/adr/adr-program-012-audit-convergence-gates.md
related:
  adr:
    - docs/program/adr/adr-protocol-006-unified-rpc.md
    - docs/program/adr/adr-workspace-007-workspace-isolation.md
    - docs/program/adr/adr-orchestration-009-engine-attachment.md
    - docs/program/adr/adr-contracts-011-contract-runbook-lock.md
    - docs/program/adr/adr-program-012-audit-convergence-gates.md
  specs:
    - ../governance/REGISTRY.md
    - ../governance/contracts/control/schema/control_call.v1.json
    - ../governance/contracts/control/schema/exec_reply.v1.json
    - ../governance/contracts/control/schema/authority.v1.json
    - ../governance/contracts/protocol/include/transport.h
    - ../governance/contracts/protocol/include/yai_protocol_ids.h
    - ../governance/contracts/vault/include/yai_vault_abi.h
    - ../governance/contracts/vault/schema/vault_abi.json
    - ../governance/registry/commands.v1.json
  test_plans:
    - ops/evidence/qualification/test-plans/hardfail.md
  tools:
    - tools/bin/yai-check-pins
    - tools/bin/yai-verify
    - tools/bin/yai-gate
    - tools/bin/yai-suite
tags:
  - runtime
  - data-plane
  - storage
  - audit-convergence
---

# RB-DATA-PLANE - Data Plane Program (rev4)

## 1) Purpose
Define the canonical, governed persistence program for YAI.

Data Plane is not a backend choice. Data Plane is the persistent substrate for
`core`, `exec`, `data`, `graph`, `knowledge`, and governance lifecycle surfaces.

## 2) Program framing

### Dominant model
- `cli -> sdk -> yai` is canonical operator path.
- Inside `yai`, responsibilities are stratified in `core`, `exec`, `data`,
  `graph`, `knowledge`.
- `governance` remains normative source; `ops` remains closure evidence sink.
- `yai-daemon` may run edge-side for distributed acquisition but does not
  introduce a second data-plane source of truth.

### Declassed legacy center
- `mind-redis-stm.md` is historical component/backend guidance, not DP center.
- Redis is a candidate backend role for transient cognition, not DP identity.

## 3) Non-negotiable invariants
1. No direct client write to storage backends.
2. Workspace scope and path-jail boundaries are mandatory.
3. Authority/lifecycle-gated state cannot be bypassed.
4. Deterministic reply semantics are mandatory.
5. Sink-first order is mandatory before rich query surfaces.

## 4) Canonical classes and ownership
DP storage classes and owner mapping are defined in:
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-plane-storage-classes.md`

DP storage topology and persistence layout are defined in:
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-plane-storage-topology.md`

Runtime anchors used by this program:
- `lib/core/workspace/*`
- `lib/core/authority/*`
- `lib/core/session/*`
- `lib/exec/runtime/*`
- `lib/exec/gates/storage_gate.c`
- `lib/governance/mapping/decision_to_evidence.c`
- `lib/governance/mapping/decision_to_audit.c`
- `lib/knowledge/memory/*`
- `lib/graph/*`
- `data/global/knowledge.db`

YD-3 source-plane baseline classes are owner-side runtime classes:
- `source_node`
- `source_daemon_instance`
- `source_binding`
- `source_asset`
- `source_acquisition_event`
- `source_evidence_candidate`
- `source_owner_link`

## 5) Sink-first execution strategy
Mandatory order:
1. class model
2. backend role model
3. storage topology/layout
4. sink contracts and write paths
5. implementation and cutover
6. read/query/operator surfaces
7. richer workspace↔graph↔workflow semantics

## 6) Program mapping (DP block, 9 deliveries)

### DP-1 — Refoundation of the Canonical Data Plane Model
Canonical model, terminology, boundaries, ownership baseline.

### DP-2 — Canonical Storage Classes and Backend Role Model
Class semantics, owner mapping, backend role fit, separation rules.

### DP-3 — Canonical Storage Topology and Persistence Layout
On-disk/in-store topology, persistence layout, migration-safe structure.

### DP-4 — Event and Evidence Sink Hardening
Event/evidence sink contracts, retention, deterministic failure semantics.

### DP-5 — Governance and Compliance Persistence Integration
Governance object/lifecycle persistence integrated with runtime boundaries.

### DP-6 — Authority and Artifact Metadata Store Integration
Authority state and artifact metadata persistence under canonical ownership.

### DP-7 — Brain Graph Sink and Transient Cognition Backend
Graph truth and transient cognition separation with explicit sink contracts.

### DP-8 — CLI/SDK Data Surfaces and Operator Query Model
Operator/programmatic query surfaces over canonical sinks, no bypass.

### DP-9 — Verification, Qualification and Pre-Pilot Data Closure
Cross-repo verification, qualification evidence, and pre-pilot closure pack.

## 7) Out of scope for current DP block
- distributed replication/HA/federation
- multi-node graph fabric
- rich cross-workspace federated query model
- full workflow persistence model
- full cockpit data fabric

## 7.1) Next block kickoff (DP-10+)
Second block starts with writer closure:
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/mp-runtime-000-enforcement-to-record-persistence.md`

Priority order:
1. close enforcement -> canonical persisted record set writers
2. materialize graph from typed refs (`docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-graph-materialization.md`)
3. cut readers to DB-first canonical sources (`docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-db-first-read-path-cutover.md`).
4. map filesystem operational residues (`docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/mp-runtime-000-filesystem-decommission.md`).
5. execute cleanup/archive of decommissioned residues (`docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-filesystem-cleanup.md`).
6. strengthen DB-backed governance/compliance visibility.
7. define lifecycle/retention/tiering model (`docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-lifecycle.md`).
8. implement compaction/pruning/archive anti-leakage execution model (DP-15B).
9. expose graph baseline read surfaces and summaries (DP-16).
10. close second-block verification/qualification (DP-17).

## 8) Verification matrix baseline
Mandatory lanes:
- pin/contract checks against `governance`
- workspace scope/path-jail checks
- lifecycle/boundary gate checks
- deterministic error/reply checks
- cross-link integrity checks in program docs

Evidence minimum:
- command outputs
- logs
- verification reports
- traceability pointers to runbook and claims

## 9) Failure modes and controls
- Cross-tenant leakage:
  - control: workspace boundary + path-jail enforcement.
- Contract drift:
  - control: pin checks + anchor verification.
- Backend-role drift:
  - control: storage-class/role matrix review gating.
- File-first regression:
  - control: sink contract enforcement and lifecycle gates.

## 10) Rollback policy
- Roll back active DP phase branch only.
- Restore last verified model or sink baseline.
- Re-run mandatory checks before phase reopen.

## 11) Traceability
- `docs/program/reports/audit-convergence-report.md`
- `docs/program/reports/audit-convergence-report.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-plane-storage-classes.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-plane-storage-topology.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/mp-runtime-000-data-sinks.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/mp-runtime-000-evidence-and-event-persistence.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/mp-runtime-000-governance-persistence.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/mp-runtime-000-authority-and-artifact-persistence.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-brain-memory-and-graph-sinks.md` (historical alias note)
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-query-architecture.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-qualification.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/mp-runtime-000-enforcement-to-record-persistence.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-graph-materialization.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-db-first-read-path-cutover.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/mp-runtime-000-filesystem-decommission.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/mp-runtime-000-filesystem-cleanup.md`
- `docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/mp-runtime-000-data-lifecycle.md`

## 12) Definition of Done (program)
- DP-1..DP-9 closures contain explicit evidence links.
- No unresolved drift between code behavior and pinned contracts.
- Data-plane evolution remains sink-first and boundary-governed.
