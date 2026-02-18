from __future__ import annotations


def generate_issue_body(title: str, issue_type: str, mp_id: str, runbook: str, phase: str) -> str:
    return f"""## Type
{issue_type}

## Title
{title}

## IDs
- MP-ID: {mp_id}
- Runbook: {runbook}
- Phase: {phase}

## Objective
- One clear objective.

## Acceptance Criteria
- [ ] Positive evidence included
- [ ] Negative evidence included

## Commands / Repro
```bash
# exact commands
```
"""
