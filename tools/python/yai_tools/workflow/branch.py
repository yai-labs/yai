from __future__ import annotations

import re

from yai_tools._core.git import checkout_new_branch
from yai_tools._core.text import normalize_issue


def _slug(s: str) -> str:
    s = s.strip().lower()
    s = re.sub(r"[^a-z0-9]+", "-", s)
    s = re.sub(r"-{2,}", "-", s)
    return s.strip("-")


def make_branch_name(change_type: str, issue: str, reason: str, area: str, desc: str) -> str:
    t = _slug(change_type)
    a = _slug(area)
    d = _slug(desc)

    if not t:
        raise ValueError("type is empty after slug")
    if not a:
        raise ValueError("area is empty after slug")
    if not d:
        raise ValueError("desc is empty after slug")

    issue_val = normalize_issue(issue)
    if issue_val == "N/A":
        if not reason.strip():
            raise ValueError("reason is required when issue is N/A")
        if t in {"docs", "chore", "meta"}:
            return f"meta/{a}-{d}"
        if t == "hotfix":
            return f"hotfix/{a}-{d}"
        raise ValueError("issue N/A is allowed only for meta/docs/chore/hotfix")

    # Standard: <type>/<issue>-<area>-<desc>
    return f"{t}/{issue_val[1:]}-{a}-{d}"


def maybe_checkout(name: str) -> None:
    checkout_new_branch(name)
