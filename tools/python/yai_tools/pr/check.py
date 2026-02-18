from __future__ import annotations

import re
from pathlib import Path


def _extract(md: str, key: str) -> str:
    m = re.search(rf"{re.escape(key)}\s*:\s*([^\n\r]+)", md, flags=re.IGNORECASE)
    return m.group(1).strip() if m else ""


def check_pr_body(path: str) -> tuple[bool, str]:
    p = Path(path)
    if not p.exists():
        return False, f"PR body file not found: {path}"

    body = p.read_text(encoding="utf-8")

    required = [
        "Issue-ID:",
        "MP-ID:",
        "Runbook:",
        "Base-Commit:",
        "Classification:",
        "Compatibility:",
        "## Evidence",
        "## Commands run",
    ]
    missing = [x for x in required if x not in body]
    if missing:
        return False, f"missing required fields: {', '.join(missing)}"

    banned = [
        "#<issue-number>",
        "<40-char-sha>",
        "MP-<TRACK>-<X.Y.Z>",
        "docs/runbooks/<name>.md#<anchor>",
        "<one paragraph>",
        "<what doc/policy changes and why>",
        "<case 1>",
        "# exact commands",
    ]
    unresolved = [x for x in banned if x in body]
    if unresolved:
        return False, f"unresolved placeholders: {', '.join(unresolved)}"
    if re.search(r"^\s*-\s+\.\.\.\s*$", body, flags=re.MULTILINE):
        return False, "placeholder bullets ('- ...') are not allowed"

    issue = _extract(body, "Issue-ID")
    if not re.fullmatch(r"#\d+|N/A", issue, flags=re.IGNORECASE):
        return False, "Issue-ID must be #<number> or N/A"

    if issue.upper() == "N/A":
        reason = _extract(body, "Issue-Reason") or _extract(body, "Issue-Reason (required if N/A)")
        if not reason or "<required" in reason:
            return False, "Issue-Reason required when Issue-ID is N/A"

    mp_id = _extract(body, "MP-ID")
    if not re.fullmatch(r"MP-[A-Z0-9-]+-\d+\.\d+\.\d+|N/A", mp_id, flags=re.IGNORECASE):
        return False, "MP-ID must be MP-<TRACK>-<X.Y.Z> or N/A"

    runbook = _extract(body, "Runbook")
    if not re.fullmatch(r"docs/runbooks/.+\.md#.+|N/A", runbook, flags=re.IGNORECASE):
        return False, "Runbook must be docs/runbooks/<name>.md#<anchor> or N/A"

    base = _extract(body, "Base-Commit")
    if not re.fullmatch(r"[0-9a-fA-F]{40}", base):
        return False, "Base-Commit must be 40-char SHA"

    ev_match = re.search(r"## Evidence\s*([\s\S]*?)(?:\n##\s|$)", body, flags=re.IGNORECASE)
    evidence = (ev_match.group(1) if ev_match else "").strip()
    if not evidence:
        return False, "Evidence section cannot be empty"
    if re.search(r"\b(TODO|TBD|to be done|lorem ipsum)\b", evidence, flags=re.IGNORECASE):
        return False, "Evidence section has placeholder/TODO text"
    if not re.search(r"-\s+Positive:\s*[\s\S]*-\s+Negative:", evidence, flags=re.IGNORECASE):
        return False, "Evidence must include Positive and Negative subsections"

    cmd_match = re.search(r"## Commands run\s*[\s\S]*?```bash\s*([\s\S]*?)```", body, flags=re.IGNORECASE)
    if not cmd_match:
        return False, "Commands run must include a bash fenced block"
    cmd_body = cmd_match.group(1)
    runnable = [ln.strip() for ln in cmd_body.splitlines() if ln.strip() and not ln.strip().startswith("#")]
    if not runnable:
        return False, "Commands run must include at least one executable command"

    return True, f"PR metadata valid ({path})"
