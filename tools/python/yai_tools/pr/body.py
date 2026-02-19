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


def _has_validation_command(commands: list[str], needles: list[str]) -> bool:
    lowered = [c.lower() for c in commands]
    return any(any(n in c for n in needles) for c in lowered)


def _build_checklist(
    template: str,
    issue_val: str,
    reason: str,
    mp_id: str,
    runbook: str,
    docs_touched: list[str],
    spec_delta: list[str],
    evidence_positive: list[str],
    evidence_negative: list[str],
    commands: list[str],
) -> str:
    issue_ok = issue_val != "N/A" or bool(reason.strip())
    cmd_count = len([c for c in commands if c.strip()])
    pos_count = len([x for x in evidence_positive if x.strip()])
    neg_count = len([x for x in evidence_negative if x.strip()])
    docs_count = len([x for x in docs_touched if x.strip()])
    spec_count = len([x for x in spec_delta if x.strip()])

    lines = [
        f"- [{'x' if issue_ok else ' '}] Issue linkage valid (`{issue_val}`{' + reason' if issue_val == 'N/A' else ''})",
        f"- [x] Evidence is concrete (positive: {pos_count}, negative: {neg_count})",
        f"- [x] Commands are listed and runnable ({cmd_count})",
    ]

    if template == "docs-governance":
        docs_check = docs_count > 0
        delta_check = spec_count > 0
        link_checks = _has_validation_command(commands, ["yai-docs-doctor", "yai-docs-trace-check", "yai-architecture-check", "markdown-link", "link"])
        lines.extend(
            [
                f"- [{'x' if docs_check else ' '}] Docs touched is explicit ({docs_count})",
                f"- [{'x' if delta_check else ' '}] Spec/contract delta is explicit ({spec_count})",
                f"- [{'x' if link_checks else ' '}] Link/alignment validation command included",
            ]
        )

    if template == "type-a-milestone":
        lines.extend(
            [
                f"- [{'x' if mp_id != 'N/A' else ' '}] MP-ID is set (`{mp_id}`)",
                f"- [{'x' if runbook != 'N/A' else ' '}] Runbook anchor is set (`{runbook}`)",
                "- [ ] Matches runbook \"Done when\" (manual reviewer confirmation)",
            ]
        )

    if template == "type-b-twin-pr":
        twin_hint = _has_validation_command(commands, ["yai-cli", "yai-specs"])
        lines.extend(
            [
                f"- [{'x' if twin_hint else ' '}] Cross-repo commands/evidence included",
                "- [ ] Twin PR links filled in the body (manual author confirmation)",
            ]
        )

    return "\n".join(lines)


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

    if "## Checklist" in md:
        checklist_block = _build_checklist(
            template=template,
            issue_val=issue_val,
            reason=reason,
            mp_id=(mp_id.strip() or "N/A"),
            runbook=(runbook.strip() or "N/A"),
            docs_touched=docs_touched,
            spec_delta=spec_delta,
            evidence_positive=ev_pos,
            evidence_negative=ev_neg,
            commands=cmd_lines,
        )
        md = _set_section(md, "## Checklist", checklist_block)

    return md
