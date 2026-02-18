## IDs
- Issue-ID: #<issue-number>
- MP-ID: MP-<TRACK>-<X.Y.Z>
- Runbook: docs/runbooks/<name>.md
- Base-Commit: <40-char-sha>

## Classification
- Compatibility: B
- Repositories impacted: yai + yai-cli (+ yai-specs if contract touched)

## Twin PR links (required)
- yai-cli PR: <https://github.com/.../pull/...>
- yai-specs PR: <link or N/A>

## Objective

## Contract delta
- Envelope:
- Authority:
- Errors:
- Logging:

## Evidence (minimum)
- Positive:
  - <case 1>
  - <case 2>
- Negative:
  - <case 1>
  - <case 2>

## Commands run
```bash
# paste exact commands
```

## Compatibility window / rollout

## Checklist
- [ ] Root/core and CLI changes are both linked
- [ ] Twin PRs are reviewable as one unit
- [ ] Evidence includes deterministic rejects and expected success paths
- [ ] Release/tag only after cross-repo evidence is complete
