#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

TARGET_DIRS = [
    "docs/architecture",
    "docs/guides",
    "docs/reference",
    "docs/runbooks",
    "docs/program",
]

REQUIRED_AUDITS = [
    "docs/archive/migration/c17.7-docs-surface-hardening-audit.md",
    "docs/archive/migration/c17.7-docs-residual-naming-plan.md",
]

README_CONTRACT_TARGETS = [
    "docs/README.md",
    "docs/architecture/README.md",
    "docs/guides/README.md",
    "docs/guides/developer/README.md",
    "docs/reference/README.md",
    "docs/reference/protocol/README.md",
    "docs/runbooks/README.md",
    "docs/program/README.md",
]

README_CONTRACT_SECTIONS = [
    "## Scope",
    "## What Belongs Here",
    "## What Does Not Belong Here",
    "## Navigation Order",
    "## Extension Rules",
]


def main() -> int:
    errors: list[str] = []

    for rel in REQUIRED_AUDITS:
        if not (ROOT / rel).is_file():
            errors.append(f"missing required C17.7 audit file: {rel}")

    # Naming hardening: block very long/mechanical live names.
    # Non-archive docs only.
    for d in TARGET_DIRS:
        base = ROOT / d
        if not base.is_dir():
            continue
        for p in base.rglob("*.md"):
            if "archive" in p.parts:
                continue
            name = p.name
            if len(name) > 64:
                errors.append(
                    f"filename too long for hardened surface ({len(name)}): {p.relative_to(ROOT)}"
                )
            if "-and-and-" in name or "--" in name:
                errors.append(f"mechanical filename pattern: {p.relative_to(ROOT)}")

    # README contract checks on key section spines.
    for rel in README_CONTRACT_TARGETS:
        p = ROOT / rel
        if not p.is_file():
            errors.append(f"missing README contract target: {rel}")
            continue
        text = p.read_text(encoding="utf-8", errors="ignore")
        for sec in README_CONTRACT_SECTIONS:
            if sec not in text:
                errors.append(f"README missing '{sec}': {rel}")

    if errors:
        print("docs_surface_hardening: FAIL")
        for e in errors:
            print(" -", e)
        return 1

    print("docs_surface_hardening: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
