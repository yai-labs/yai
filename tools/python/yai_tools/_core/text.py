from __future__ import annotations

import re


def normalize_issue(issue: str) -> str:
    s = issue.strip()
    if s.lower() in {"n/a", "na"}:
        return "N/A"
    if s.startswith("#"):
        s = s[1:]
    if not re.fullmatch(r"\d+", s):
        raise ValueError("issue must be a number (e.g. 123 or #123) or N/A")
    return f"#{s}"


def set_kv_line(md: str, key: str, value: str) -> str:
    """
    Replace any line containing '<key>:' with '<key>: <value>'.
    Works with bullet variants like '- Key: ...'.
    """
    out = []
    replaced = False
    for line in md.splitlines():
        if re.search(rf"\b{re.escape(key)}\s*:\b", line):
            prefix = re.split(rf"{re.escape(key)}\s*:", line, maxsplit=1)[0]
            # keep leading bullet/indent and rewrite rest
            new_line = f"{prefix}{key}: {value}"
            out.append(new_line)
            replaced = True
        else:
            out.append(line)
    if not replaced:
        # append if missing (rare but safer)
        out.append(f"{key}: {value}")
    return "\n".join(out) + ("\n" if md.endswith("\n") else "")
