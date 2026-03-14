#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

SCAN_ROOTS = [
    "include",
    "lib",
    "tests",
    "tools",
    "Makefile",
    "README.md",
    "FOUNDATION.md",
    "GOVERNANCE.md",
    "COMPATIBILITY.md",
    "VERSIONING.md",
]

FORBIDDEN_PATTERNS = [
    "<yai/core/",
    "include/yai/core",
    " yai/core/",
]


def iter_files(root: Path):
    if root.is_file():
        yield root
        return
    for p in root.rglob("*"):
        if p.is_file():
            yield p


def main() -> int:
    repo = Path(__file__).resolve().parents[2]
    errors: list[str] = []

    if (repo / "include" / "yai" / "core").exists():
        errors.append("forbidden public namespace present: include/yai/core")

    for rel in SCAN_ROOTS:
        root = repo / rel
        if not root.exists():
            continue
        for f in iter_files(root):
            fr = f.relative_to(repo).as_posix()
            if fr.startswith("docs/") or fr.startswith("transitional/"):
                continue
            if fr == "tools/validate/validate_runtime_core_collapse.py":
                continue
            text = f.read_text(encoding="utf-8", errors="ignore")
            for token in FORBIDDEN_PATTERNS:
                if token in text:
                    errors.append(f"{fr}: forbidden core namespace token '{token}'")

    if errors:
        print("runtime_core_collapse: FAIL")
        for e in errors:
            print(" -", e)
        return 1

    print("runtime_core_collapse: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
