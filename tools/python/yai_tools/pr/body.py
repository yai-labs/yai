from __future__ import annotations

import re
from pathlib import Path

from yai_tools._core.git import head_sha
from yai_tools._core.paths import repo_root
from yai_tools._core.text import has_kv_line, normalize_issue, set_kv_line


TEMPLATE_MAP = {
    "default": "default.md",
    "docs-governance": "docs-governance.md",
    "type-a-milestone": "type-a-milestone.md",
    "type-b-twin-pr": "type-b-twin-pr.md",
}


def _template_path(template: str) -> Path:
    if template not in TEMPLATE_MAP:
        raise ValueError(f"unknown template '{template}'. expected: {', '.join(TEMPLATE_MAP.keys())}")
    return repo_root() / ".github" / "PULL_REQUEST_TEMPLATE" / TEMPLATE_MAP[template]


def _set_section(md: str, heading: str, content: str) -> str:
    pattern = rf"({re.escape(heading)}\n)([\s\S]*?)(?=\n## |\Z)"
    repl = rf"\1{content}\n"
    return re.sub(pattern, repl, md, count=1)


def _fmt_bullets(items: list[str]) -> str:
    return "\n".join([f"- {x}" for x in items])


def generate_pr_body(
    template: str,
    issue: str,
    reason: str,
    mp_id: str,
    runbook: str,
    classification: str,
    compatibility: str,
    objective: str,
    docs_touched: list[str],
    spec_delta: list[str],
    evidence_positive: list[str],
    evidence_negative: list[str],
    commands: list[str],
) -> str:
    path = _template_path(template)
    md = path.read_text(encoding="utf-8")

    issue_val = normalize_issue(issue)
    md = set_kv_line(md, "Issue-ID", issue_val)

    if issue_val == "N/A":
        r = reason.strip()
        if not r:
            raise ValueError("Issue-Reason is required when Issue-ID is N/A")
        md = set_kv_line(md, "Issue-Reason (required if N/A)", r)
        if has_kv_line(md, "Issue-Reason"):
            md = set_kv_line(md, "Issue-Reason", r)

    md = set_kv_line(md, "MP-ID", mp_id.strip() or "N/A")
    md = set_kv_line(md, "Runbook", runbook.strip() or "N/A")
    md = set_kv_line(md, "Classification", classification.strip().upper())
    md = set_kv_line(md, "Compatibility", compatibility.strip().upper())
    md = set_kv_line(md, "Base-Commit", head_sha())

    objective_val = objective.strip()
    if not objective_val:
        raise ValueError("--objective is required")
    if not evidence_positive:
        raise ValueError("at least one --evidence-positive is required")
    if not evidence_negative:
        raise ValueError("at least one --evidence-negative is required")
    if not commands:
        raise ValueError("at least one --command is required")

    if template == "docs-governance":
        if not docs_touched:
            raise ValueError("--docs-touched is required for docs-governance")
        if not spec_delta:
            raise ValueError("--spec-delta is required for docs-governance")

    md = _set_section(md, "## Objective", objective_val)

    if "## Docs touched" in md and docs_touched:
        md = _set_section(md, "## Docs touched", _fmt_bullets([x.strip() for x in docs_touched if x.strip()]))
    if "## Spec/Contract delta" in md and spec_delta:
        md = _set_section(md, "## Spec/Contract delta", _fmt_bullets([x.strip() for x in spec_delta if x.strip()]))

    ev_pos = [x.strip() for x in evidence_positive if x.strip()]
    ev_neg = [x.strip() for x in evidence_negative if x.strip()]
    evidence_block = "- Positive:\n" + "\n".join([f"  - {x}" for x in ev_pos]) + "\n- Negative:\n" + "\n".join(
        [f"  - {x}" for x in ev_neg]
    )
    md = _set_section(md, "## Evidence", evidence_block)

    cmd_lines = [x.strip() for x in commands if x.strip()]
    commands_block = "```bash\n" + "\n".join(cmd_lines) + "\n```"
    md = _set_section(md, "## Commands run", commands_block)

    return md
