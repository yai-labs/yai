# GitHub Issue Templates (Phase/MP Closure)

## Canonical naming
- Milestone: `PHASE: <track>@<phase>`
- Phase issue: `runbook: <RB-ID> — <phase> <short title>`
- MP closure issue: `mp-closure: <MP-ID> — <phase> Closure`

## Mapping rules
- `track` is the program family (`contract-baseline-lock`, `root-hardening`, ...).
- `phase` is the delivery slice (`0.1.0`, `0.1.1`, ...).
- `RB-ID` anchors the runbook (`RB-*`) and appears only in the phase issue title.
- `MP-ID` is phase-specific (`MP-<TRACK>-<phase>`) and appears in both phase issue and MP closure title/body.

## Required labels
- Phase issue:
  - `runbook`
  - `track:<track>`
  - `phase:<phase>`
  - `governance`
  - `class:A` (if applicable)
- MP closure:
  - `mp-closure`
  - `track:<track>`
  - `phase:<phase>`
  - `governance`
- PR:
  - `track:<track>`
  - `phase:<phase>`
  - keep existing functional labels (for example `type:docs` / `work-type:docs`)

## Label badge color policy
- Colors are assigned automatically by label namespace (no manual per-label tuning):
  - `phase:*` -> blue
  - `track:*` -> green
  - `class:*` -> amber
  - `type:*` / `work-type:*` -> light blue
  - `area:*` -> deep blue
  - governance/core labels (`runbook`, `governance`, `mp-closure`) keep fixed canonical colors
- Source of truth is in `tools/python/yai_tools/cli.py` (`label-sync` + label ensure paths).
- `tools/bin/yai-dev-fix-phase --apply` also normalizes colors for labels touched by phase issues/PRs.

## Templates in `.github/ISSUE_TEMPLATE`
- `phase-issue.yml`: execution issue for the phase.
- `mp-closure.yml`: closure notarization issue for the phase.
- Both templates are manual fallback; tooling remains authoritative for labels/milestone assignment.
- `phase-issue.yml` includes `Class` input (`A/B/C`) so class tagging is explicit and non-hardcoded.

## CLI workflow
Create phase issue:

```bash
tools/bin/yai-dev-issue phase \
  --track contract-baseline-lock \
  --phase 0.1.0 \
  --rb-id RB-CONTRACT-BASELINE-LOCK \
  --title "Pin Baseline Freeze" \
  --rb-anchor docs/runbooks/contract-baseline-lock.md#0.1.0 \
  --mp-id MP-CONTRACT-BASELINE-LOCK-0.1.0 \
  --repo yai-labs/yai
```

Create MP closure issue:

```bash
tools/bin/yai-dev-issue mp-closure \
  --track contract-baseline-lock \
  --phase 0.1.0 \
  --mp-id MP-CONTRACT-BASELINE-LOCK-0.1.0 \
  --repo yai-labs/yai
```

Retroactive alignment (dry-run by default):

```bash
tools/bin/yai-dev-fix-phase \
  --track contract-baseline-lock \
  --phase 0.1.0 \
  --repo yai-labs/yai
```

Apply changes:

```bash
tools/bin/yai-dev-fix-phase \
  --track contract-baseline-lock \
  --phase 0.1.0 \
  --repo yai-labs/yai \
  --apply
```

Sync palette repo-wide (dry-run/apply):

```bash
tools/bin/yai-dev-label-sync --repo yai-labs/yai
tools/bin/yai-dev-label-sync --repo yai-labs/yai --apply
```
