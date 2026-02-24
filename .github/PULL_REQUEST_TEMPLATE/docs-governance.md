## IDs
- Issue-ID: #<issue-number>
- Issue-Reason (required if N/A): <required when Issue-ID is N/A>
- Closes-Issue: Closes #<issue-number> OR N/A
- MP-ID: MP-<TRACK>-<X.Y.Z> OR N/A
- Runbook: N/A or yai-infra governance reference OR N/A
- Base-Commit: <40-char-sha>

## Issue linkage
- Closes #<issue-id>  <!-- required unless Issue-Reason provided -->

## Classification
- Classification: <FEATURE|FIX|DOCS|OPS|META>
- Compatibility: <A|B|C>

## Objective
<what doc/policy changes and why>

## Docs touched
- ...

## Spec/Contract delta
- ...

## Evidence
- Positive:
  - <case 1>
- Negative:
  - <case 1>

## Commands run
```bash
# exact commands
```

## Checklist
- [ ] No broken links
- [ ] Templates updated consistently (if touched)
- [ ] Doc aligns with current repo state (no "future tense" lying)
