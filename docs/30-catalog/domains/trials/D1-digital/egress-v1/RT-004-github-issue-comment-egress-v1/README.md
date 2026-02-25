---
id: RT-004
title: D1 Egress RealTarget Trial - GitHub issue comment v1
status: draft
owner: catalog
effective_date: 2026-02-25
revision: 1
domain_pack_id: D1-digital/egress-v1
---

# RT-004-github-issue-comment-egress-v1

## Purpose
Prove that D1 governance can block/allow a real business effect on GitHub issue comments.

## Target
- Real effect: create issue comment via GitHub API.
- Endpoint: `api.github.com/repos/<repo>/issues/<issue>/comments`

## What it proves
1. Deny baseline blocks comment creation.
2. Allow baseline permits comment creation only on contracted target.
3. Evidence captures outcome, reason code, policy hash, and effect application state.

## Catalog -> Qualification mapping
- Catalog trial spec (this folder)
- Qualification runner: `docs/40-qualification/RT-0.1-004-D1-GITHUB-ISSUE-COMMENT/`

## Produced evidence
- `baseline.json`
- `timeline.jsonl`
- `decision_records.jsonl`
- `containment_metrics.json`
- `system_state.txt`
- `EVIDENCE_INDEX.md`
