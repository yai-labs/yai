---
id: RT-0.1-004-D1-GITHUB-ISSUE-COMMENT
title: RT-0.1-004 - D1 GitHub Issue Comment RealTarget Trial
status: draft
owner: qualification
effective_date: 2026-02-25
revision: 1
domain_pack_id: D1-digital/egress-v1
catalog_trial_ref: docs/30-catalog/domains/trials/D1-digital/egress-v1/RT-004-github-issue-comment-egress-v1
---

# RT-0.1-004-D1-GITHUB-ISSUE-COMMENT

RealTarget trial for D1 using a real external effect: GitHub issue comment creation.

## What it proves

- `baseline-deny`: comment is blocked (`effect_applied=false`).
- `baseline-allow`: same action is allowed (`effect_applied=true`) only for the contracted target.

## Required env

- `GH_REPO` example: `yai-labs/yai`
- `GH_ISSUE_NUMBER` example: `199`

Optional:
- `GH_COMMENT_PREFIX` default: `[YAI-SC102-LAUNCH]`
- `GH_COMMENT_CLEANUP=1` to delete created comments in allow runs.

## Run deny

```bash
cd docs/40-qualification/RT-0.1-004-D1-GITHUB-ISSUE-COMMENT
GH_REPO=yai-labs/yai GH_ISSUE_NUMBER=199 BASELINE_ID=baseline-deny ./run/run-once.sh
```

## Run allow

```bash
cd docs/40-qualification/RT-0.1-004-D1-GITHUB-ISSUE-COMMENT
GH_REPO=yai-labs/yai GH_ISSUE_NUMBER=199 BASELINE_ID=baseline-allow ./run/run-once.sh
```

## Evidence

- `baseline.json`
- `timeline.jsonl`
- `decision_records.jsonl`
- `containment_metrics.json`
- `system_state.txt`
- `EVIDENCE_INDEX.md`
