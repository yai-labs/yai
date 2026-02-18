from __future__ import annotations

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


def generate_pr_body(
    template: str,
    issue: str,
    reason: str,
    mp_id: str,
    runbook: str,
    classification: str,
    compatibility: str,
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

    return md
