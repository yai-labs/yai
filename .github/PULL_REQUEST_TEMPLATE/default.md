## IDs
- Issue-ID: #<issue-number> OR N/A
- Issue-Reason (required if N/A): <why this PR is allowed without an issue>
- MP-ID (optional): MP-<TRACK>-0.1.<X>
- Runbook (optional): docs/runbooks/<name>.md
- Base-Commit: <40-char-sha>

## Classification
- Compatibility: A | B | C
- Repositories impacted: yai | yai + yai-cli | yai + yai-specs | all

## Objective
<what changes in behavior, in one paragraph>

## Scope guardrails
In-scope:
- ...
Out-of-scope:
- ...

## Changes
- ...

## Contract delta (if any)
- Envelope:
- Authority:
- Errors:
- Logging:

## Evidence (minimum)
Positive:
- ...
Negative:
- ...

## Commands run
```bash
# paste exact commands
```

## Checklist
- [ ] Scope is small and reviewable
- [ ] Evidence includes at least 1 positive + 1 negative case
- [ ] If cross-repo, twin PR links are included (or explicitly N/A)
