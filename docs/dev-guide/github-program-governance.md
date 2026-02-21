---
id: DEV-GUIDE-GITHUB-PROGRAM-GOVERNANCE
status: active
effective_date: 2026-02-20
revision: 2
owner: governance
---

# GitHub Program Governance (Runbook/MP)

## 1) Purpose
This guide defines the canonical PMO operating model for program delivery across:
- `yai`
- `yai-cli`
- `yai-specs`

It works with:
- runbooks (`docs/runbooks/*`)
- milestone packs (`docs/milestone-packs/*`)
- PR metadata templates (`docs/dev-guide/github-templates.md`)

The model is designed to make phase closure auditable, deterministic, and cross-repo coherent.

## 2) Canonical objects
- **Runbook phase**: execution intent (`0.1.X`) with gate and claim.
- **Milestone (`PHASE: <track>@<phase>`)**: GitHub phase container.
- **Milestone Pack (`MP-*`)**: closure artifact with evidence.
- **MP Closure issue (`mp-closure: ...`)**: notarization artifact for milestone closure.
- **Issue**: scoped work unit.
- **PR**: change unit with explicit evidence.
- **Project item**: board tracking object for status/blockers.

Canonical formats:
- Milestone: `PHASE: <track>@<phase>`
- RB-ID: `RB-*` (runbook identifier only; never milestone naming)
- MP: `MP-<TRACK>-<X.Y.Z>`
- Runbook ref in PR body: `docs/runbooks/<name>.md#<anchor>`

## 3) Project v2 setup (canonical)
Use one cross-repo GitHub Project v2 for the entire delivery program.

### 3.1 Required fields
- `Track` (single-select)
- `Phase` (single-select)
- `Repo` (single-select)
- `Work Type` (single-select)
- `Class` (single-select: `A`, `B`)
- `Runbook Ref` (text)
- `MP-ID` (text)
- `Gate Status` (single-select)
- `Target Date` (date)

### 3.2 Recommended select values
- `Track`: `contract-baseline-lock`, `specs-refactor-foundation`, `root-hardening`, `workspaces-lifecycle`, `engine-attach`, `data-plane`
- `Work Type`: `Issue`, `PR`, `MP Closure`, `Risk`
- `Gate Status`: `Not started`, `Running`, `Blocked`, `Passed`, `Failed`

### 3.3 Required views
- **Program Board**: kanban by `Gate Status`
- **By Track**: grouped table by `Track` + `Phase`
- **Cross-Repo Sync**: filter `Class = B`
- **Milestone Calendar**: date-driven plan/forecast

## 4) Milestone operating model

### 4.1 Home repository
`yai` is the control tower. Official phase milestones are maintained in `yai`.

### 4.2 Cross-repo execution
- `yai-cli` and `yai-specs` do not need duplicate canonical milestones.
- They MUST link their issues/PRs to the controlling `yai` milestone.

### 4.3 Milestone description template
Every `PHASE:*` milestone description SHOULD include:
- objective (phase claim)
- runbook anchor
- done-when gate checklist
- required MP-ID
- linked cross-repo issues/PRs

## 5) Issue/PR contract

### 5.1 PR metadata minimum
Every PR involved in a runbook phase MUST include:
- `Runbook Ref`
- `MP-ID`
- `Class` (`A|B`)
- `Evidence`
- `Commands`

Use canonical template checks from:
- `docs/dev-guide/github-templates.md`

### 5.2 `Issue-ID: N/A` policy
Allowed only for documented governance exceptions.
If used, PR body MUST provide:
- `Issue-ID: N/A`
- `Issue-Reason: <explicit rationale>`

## 6) Closure checklist (copy/paste)

```text
Phase closure checklist (PHASE:*):
[ ] Gate checks passed (local + CI where required)
[ ] MP file exists and is linked
[ ] MP Closure issue exists and is complete
[ ] Runbook phase references updated (if required)
[ ] Linked issues/PRs closed or explicitly waived
[ ] Class B twin PR requirement satisfied
[ ] Evidence is reproducible (commands + outcomes)
```

A phase milestone MUST NOT be closed until all items are satisfied.

## 7) Weekly operating ritual
Cadence: once per week, fixed slot.

Agenda:
1. Triage blocked items (`Gate Status = Blocked`)
2. Review `Class = B` items missing twin PR linkage
3. Burn-down by milestone (`PHASE:*`) and track
4. Validate upcoming closure prerequisites (MP, evidence, gate)

Outputs:
- updated board statuses,
- explicit owner/action for blockers,
- closure forecast by phase.

## 8) Failure modes and mitigations

### F1: Milestone closed without MP
- Symptom: phase marked done but no closure artifact.
- Mitigation: reopen milestone; block closure until MP is published.

### F2: Cross-repo drift
- Symptom: `yai-specs` change not reflected in consumers.
- Mitigation: create linked issues in impacted repos; keep milestone open until pin+verify alignment.

### F3: Green CI but weak evidence
- Symptom: checks pass, but no reproducible negative/positive proof in PR.
- Mitigation: enforce PR metadata contract and evidence checklist before closure.

## 9) Relationship to other guides
- Delivery mechanics: `docs/dev-guide/cross-repo-workflow.md`
- PR metadata templates/gates: `docs/dev-guide/github-templates.md`
- Runbook execution: `docs/runbooks/README.md`
- Milestone closure artifacts: `docs/milestone-packs/README.md`

## 10) Defaults (canonical assumptions)
- One Project v2 cross-repo.
- Official phase milestones in `yai`.
- `yai-cli` and `yai-specs` use linked issues/PRs, not duplicate canonical milestones.
- Every runbook phase maps to one `PHASE: <track>@<phase>` milestone.
- No milestone closure without MP and verifiable evidence.

## 11) Audit Convergence Alignment (v0.1.0)

For Infra Grammar closure governance, use these canonical references:
- `docs/program-delivery/audit-convergence/EXECUTION-PLAN-v0.1.0.md`
- `docs/program-delivery/audit-convergence/AUDIT-CONVERGENCE-MATRIX-v0.1.0.md`
- `docs/audits/claims/infra-grammar.v0.1.json`

Operational rule:
- issues/PRs that close a runbook phase should include claim IDs from the registry.
