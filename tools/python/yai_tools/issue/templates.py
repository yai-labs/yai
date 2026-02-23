from __future__ import annotations

import re


def canonical_milestone_title(track: str, phase: str) -> str:
    return f"PHASE: {track}@{phase}"


def canonical_phase_issue_title(rb_id: str, phase: str, short_title: str) -> str:
    short = short_title.strip()
    return f"runbook: {rb_id} — {phase} {short}".strip()


def canonical_mp_closure_title(mp_id: str, phase: str) -> str:
    return f"mp-closure: {mp_id} — {phase} Closure"


def default_rb_id(track: str) -> str:
    return f"RB-{track.replace('_', '-').upper()}"


def default_mp_id(track: str, phase: str = "") -> str:
    base = f"MP-{track.replace('_', '-').upper()}"
    return f"{base}-{phase}" if phase else base


def phase_label(phase: str) -> str:
    return f"phase:{phase}"


def track_label(track: str) -> str:
    return f"track:{track}"


def phase_issue_labels(track: str, phase: str, include_class_label: bool = False) -> list[str]:
    labels = ["runbook", phase_label(phase), track_label(track), "governance"]
    if include_class_label:
        labels.append("class:A")
    return labels


def mp_closure_labels(track: str, phase: str) -> list[str]:
    return ["mp-closure", phase_label(phase), track_label(track), "governance"]


def pr_phase_labels(track: str, phase: str) -> list[str]:
    return [phase_label(phase), track_label(track)]


def normalize_phase(phase: str) -> str:
    txt = phase.strip()
    if not re.fullmatch(r"\d+\.\d+\.\d+", txt):
        raise ValueError(f"invalid phase format '{phase}': expected X.Y.Z")
    return txt


def render_milestone_body(track: str, phase: str, rb_anchor: str, mp_id: str, objective: str = "") -> str:
    obj = objective.strip() or f"Deliver phase `{phase}` for track `{track}` with auditable closure evidence."
    return f"""## Objective
{obj}

## Runbook Anchor
- `{rb_anchor}`

## Done When
- [ ] Phase issue is open and active for this milestone
- [ ] Required implementation PRs are merged
- [ ] Gate status is `Passed`
- [ ] MP Closure issue is completed and closed
- [ ] Evidence links are reviewable and reproducible

## Required MP
- `{mp_id}`

## Evidence Requirements
- [ ] Link all phase PRs (core/docs/specs/cli as needed)
- [ ] Attach gate results (CI logs / command output)
- [ ] Include proof-pack or verification artifacts (if applicable)
- [ ] Declare explicit close decision in MP Closure issue
"""


def render_phase_issue_body(track: str, phase: str, rb_id: str, rb_anchor: str, mp_id: str) -> str:
    return f"""## Header
- Track: `{track}`
- Phase: `{phase}`
- RB-ID: `{rb_id}`
- Milestone: `{canonical_milestone_title(track, phase)}`
- Runbook Anchor: `{rb_anchor}`
- Target MP-ID: `{mp_id}`

## Objective
Execute the runbook phase and keep all related work auditable and linked.

## Operational Checklist
- [ ] Confirm runbook scope and expected outputs
- [ ] Create/update linked implementation PRs
- [ ] Keep `track:*` and `phase:*` labels aligned across items
- [ ] Keep all phase items assigned to the phase milestone

## PRs To Produce
- [ ] Core/runtime PR
- [ ] Docs/governance PR
- [ ] Specs/CLI companion PRs (if required)

## Gate Status
- [ ] Not started
- [ ] Running
- [ ] Passed
- [ ] Failed

## Notes
- Use this issue as the operational source of truth.
- Closure authority is the MP Closure issue for the same phase.
"""


def render_mp_closure_body(track: str, phase: str, mp_id: str, milestone_title: str) -> str:
    return f"""## Header
- Track: `{track}`
- Phase: `{phase}`
- MP-ID: `{mp_id}`
- Milestone: `{milestone_title}`

## Closure Declaration
This issue is the canonical closure record for the phase. Close it only when all evidence is complete.

## Evidence
- [ ] MP doc linked (`docs/20-governance/milestone-packs/...`)
- [ ] All phase PR links listed
- [ ] Gate status is `Passed`
- [ ] Verification artifacts linked (CI logs/proof-pack/command output)

## PR Links
- [ ] PR #...
- [ ] PR #...

## Gate Result
- Gate status: `Not started | Running | Passed | Failed`
- Evidence link(s): <!-- CI/artifact/log URLs -->

## Final Sign-off
- [ ] Evidence reviewed
- [ ] Milestone closure approved
"""
