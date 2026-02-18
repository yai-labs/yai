# Toolchain Contract v1 (YAI 0.1.x)

This is the workflow contract for changes in this repo.

## Rule 0 — You are not allowed to “wing it”
Every change must be traceable: why it exists, what it changed, and how it was tested.

## Issues
Default: every branch must be anchored to an Issue.

Allowed exception:
- governance / repo-tooling bootstrap
- small doc-only fixes

If exception is used:
- PR must include `Issue-ID: N/A`
- PR must include a non-empty `Issue-Reason`

## Branches
Canonical patterns:
- `<type>/<issue>-<area>-<desc>` (example: `feat/123-root-hardening-forward`)
- `meta/<area>-<desc>` (only when Issue-ID is N/A)

Tool support:
- Use `tools/bin/yai-branch` to generate canonical names.

## Pull requests
PR body is mandatory and must include:
- IDs (Issue-ID + Base-Commit)
- Objective
- Evidence (positive + negative)
- Commands run (bash code block)

Tool support:
- Use `tools/bin/yai-pr-body` to generate a PR body from templates.

## Agents (Codex)
Allowed:
- create branch
- commit
- push

Not allowed:
- open PR
- merge PR
