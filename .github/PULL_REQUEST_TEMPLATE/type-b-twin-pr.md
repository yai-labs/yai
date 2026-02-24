## IDs
- Issue-ID: #<issue-number>
- Issue-Reason (required if N/A): <required when Issue-ID is N/A>
- Closes-Issue: Closes #<issue-number> OR N/A
- MP-ID: MP-<TRACK>-<X.Y.Z> OR N/A
- Runbook: docs/runbooks/<name>.md#<anchor> OR N/A
- Base-Commit: <40-char-sha>

## Issue linkage
- Closes #<issue-id>  <!-- required unless Issue-Reason provided -->

## Classification
- Classification: <FEATURE|FIX|DOCS|OPS|META>
- Compatibility: <A|B|C>

## Twin PR links (required)
- yai-cli PR: <link or N/A>
- yai-law PR: <link or N/A>

## Objective
<single objective across repos>

## Contract delta
- ...

## Evidence
- Positive:
  - <case 1>
- Negative:
  - <case 1>

## Commands run
```bash
# exact commands, per repo
```

## Checklist
- [ ] PRs are reviewable as one unit
- [ ] Evidence covers success + deterministic rejects where relevant
