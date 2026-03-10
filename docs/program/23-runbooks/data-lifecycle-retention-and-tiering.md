---
id: RB-DATA-LIFECYCLE-RETENTION-TIERING
status: draft
owner: runtime
effective_date: 2026-03-10
revision: 1
depends_on:
  - docs/program/23-runbooks/enforcement-to-record-persistence.md
  - docs/program/23-runbooks/graph-materialization-from-runtime-records.md
  - docs/program/23-runbooks/db-first-read-path-cutover.md
  - docs/program/23-runbooks/filesystem-cleanup-and-archive-execution.md
---

# DP-15A — Data Lifecycle, Retention and Tiering Model

## 1) Scope
DP-15A defines the lifecycle model; DP-15B will implement jobs/mechanics.

## 2) Canonical lifecycle-aware data classes
- `DLC-1 Runtime Operational State`
- `DLC-2 Governance State`
- `DLC-3 Event Records`
- `DLC-4 Decision Records`
- `DLC-5 Evidence Records`
- `DLC-6 Authority + Artifact Metadata`
- `DLC-7 Graph Truth`
- `DLC-8 Transient Cognition`

## 3) Tier model
- `T-1 Hot`: immediate operational reads.
- `T-2 Warm`: query-friendly, compacted/less granular.
- `T-3 Cold`: archive-oriented, retrievable, not interactive-default.
- `T-4 Lineage/Summary Preserved`: anchors, rollups, chain refs, snapshots.
- `T-5 Transient`: ephemeral, non-authoritative cognition state.

## 4) Retention principles by class
- Runtime operational state: current hot, historical snapshots bounded.
- Governance state: active hot, superseded warm/cold, lineage always preserved.
- Event records: hot window bounded, historical compactable, cold/archive eligible.
- Decision records: longer-lived than raw events; effect/rationale/linkage preserved.
- Evidence records: obligation/class dependent retention; lineage mandatory.
- Authority/artifact metadata: current hot; relation history compactable; ownership refs preserved.
- Graph truth: core durable; noisy episodic tail compactable/prunable.
- Transient cognition: TTL-bound, never archival truth.

## 5) Workspace partitioning discipline
Partition keys are mandatory across lifecycle policies:
- `workspace_id`
- `time_window`
- `data_class`
- `lifecycle_state` where relevant

Goals:
- bounded scan radius
- explicit scope isolation
- deterministic purge/archive boundaries

## 6) Anti-leakage lifecycle invariants
- no cross-workspace hot reads without explicit authorized relation
- no compaction merge that erases scope boundaries
- summaries must keep scope identity and provenance
- purge/retention must not leave cross-scope orphan refs

## 7) Event/evidence lifecycle model
Event model:
- bounded hot window
- warm compaction eligibility
- cold/archive threshold
- lineage/summary preservation required

Evidence model:
- retention profile by evidence class/obligation
- full-detail vs summary retention split
- decision/evidence lineage preserved even after detail compaction

## 8) Governance retention model
- active governance: hot
- approved inactive/superseded: warm/cold
- attach/apply transitions retained with lineage
- baseline/custom and supersede chains preserved across tier transitions

## 9) Authority/artifact retention model
Authority:
- current authority resolution context in hot
- historical resolution summaries warm/cold

Artifact metadata:
- current ownership/status in hot
- touch/linkage history compactable
- minimum ownership + evidence/decision linkage preserved

## 10) Graph truth lifecycle model
Graph is split into:
- canonical core (durable): workspace/governance/decision/authority/artifact nodes + critical lineage edges
- noisy episodic tail (compactable/prunable): high-frequency low-value relation chatter
- summary-preserved layer: neighborhood rollups + anchor lineage

Rule: graph truth is not an infinite raw append mirror.

## 11) Bounded-growth design rules
- bounded hot set per data class
- bounded active graph neighborhood per workspace
- summary-over-detail past thresholds
- lineage-preserving compaction only
- no unbounded hot growth by default

## 12) Rollup/snapshot eligibility
Eligible outputs:
- workspace event window rollups
- governance activity summaries
- decision/evidence counters and trend slices
- authority/artifact linkage summaries
- graph neighborhood summaries and lineage anchors

## 13) Policy expression model
Lifecycle/retention policy expression must be:
- class-scoped
- workspace-aware
- state-aware
- time-windowed
- obligation-aware (compliance/evidence)
- versioned + validable

Canonical policy model matrix:
- `docs/program/audit-convergence/DATA-LIFECYCLE-RETENTION-MATRIX-v0.1.0.md`

## 14) Verification model (DP-15A)
DP-15A formal verification targets:
- hot->warm eligibility defined per class
- governance lineage preservation invariants
- event/evidence detail reduction with trail preservation
- graph core vs graph tail distinction
- transient cognition excluded from archival truth
- workspace scope invariants under lifecycle transitions

## 15) Handoff to DP-15B
DP-15B implements execution mechanisms:
- compaction jobs
- pruning and rollup
- archive moves
- orphan detection/repair
- anti-leakage runtime guards under churn/volume

DP-15A is model closure; DP-15B is operational closure.
