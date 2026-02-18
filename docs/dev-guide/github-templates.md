# GitHub Templates Workflow

This guide defines which GitHub template to use for each case in `yai`.

## Issue templates

- Bug: `.github/ISSUE_TEMPLATE/bug.yml`
- Feature: `.github/ISSUE_TEMPLATE/feature.yml`
- Runbook phase delivery: `.github/ISSUE_TEMPLATE/runbook-phase.yml`
- Docs governance/templates: `.github/ISSUE_TEMPLATE/docs-governance.yml`

## PR templates

- Generic: `.github/PULL_REQUEST_TEMPLATE/default.md`
- Milestone Type A: `.github/PULL_REQUEST_TEMPLATE/type-a-milestone.md`
- Milestone Type B (Twin PR): `.github/PULL_REQUEST_TEMPLATE/type-b-twin-pr.md`
- Docs governance: `.github/PULL_REQUEST_TEMPLATE/docs-governance.md`

## Required IDs in PR body

Every PR must include:

- `Issue-ID: #<number>` (or `N/A` when justified)
- `MP-ID: MP-...-X.Y.Z` (or `N/A`)
- `Compatibility: A|B|N/A`
- `Base-Commit: <40-char-sha>` (or `N/A`)

Type B additionally requires:

- `yai-cli PR: https://...`

## Validation

PR metadata is validated by:

- `.github/workflows/validate-pr-metadata.yml`
