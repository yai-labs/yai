# Data Plane Pre-Pilot Readiness Checklist

Use this checklist to declare DP baseline readiness for the first pre-pilot slice.

## 1) Build and baseline health

- [ ] `make -j4 yai` succeeds.
- [ ] runtime startup/cleanup is stable (no stale socket/process blockers).
- [ ] `workspace_demo_matrix.sh` is green.

## 2) Persistence domains

- [ ] event/evidence sinks persisted and inspectable.
- [ ] governance persistence domain active.
- [ ] authority and artifact metadata domains active.
- [ ] brain graph truth persisted and transient cognition separated.

## 3) Governance and lifecycle discipline

- [ ] review/approval/apply no-skip boundaries are enforced.
- [ ] candidate cannot bypass approval to attach/apply.
- [ ] workspace-governance linkage is typed and inspectable.

## 4) Operator query surfaces (DP-8)

- [ ] `workspace_operator_query_surfaces.sh` is green.
- [ ] query families are available: workspace/governance/events/evidence/authority/artifacts/graph.
- [ ] deterministic result shapes are present: summary_card/table/detail_record/timeline.
- [ ] unsupported families fail deterministically (`BAD_ARGS`, `unsupported_query_family`).

## 5) No-backend-leakage discipline

- [ ] canonical flow remains `cli -> sdk -> yai`.
- [ ] no direct backend query is required for operator canonical usage.
- [ ] backend role is documented as internal implementation detail.

## 6) Pre-pilot narrative readiness

- [ ] can demonstrate live governance object + apply state.
- [ ] can demonstrate event/decision/evidence trail tied to workspace.
- [ ] can demonstrate authority/artifact linkage.
- [ ] can demonstrate graph summary + transient summary with authority distinction.

## 7) Honest residual-risk declaration

- [ ] non-claims are explicit (no production HA/federation/cockpit claim).
- [ ] file export/debug surfaces are presented as secondary, not primary truth.
- [ ] remaining query/graph limitations are explicitly declared.
