#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

REQUIRED_DIRS = [
    "docs/architecture",
    "docs/guides",
    "docs/runbooks",
    "docs/reference",
    "docs/program",
    "docs/product",
    "docs/generated",
    "docs/archive",
    "docs/reference/sdk",
    "docs/archive/migration",
    "docs/archive/legacy",
    "docs/archive/historical-architecture",
]

REQUIRED_READMES = [
    "docs/README.md",
    "docs/architecture/README.md",
    "docs/guides/README.md",
    "docs/runbooks/README.md",
    "docs/reference/README.md",
    "docs/program/README.md",
    "docs/product/README.md",
    "docs/generated/README.md",
    "docs/archive/README.md",
    "docs/runbooks/operations/README.md",
    "docs/guides/developer/README.md",
]

FORBIDDEN_DOC_DIRS = [
    "docs/developer",
    "docs/developers",
    "docs/interfaces",
    "docs/platform",
    "docs/pointers",
    "docs/operators",
    "docs/program/21-rfc",
    "docs/program/22-adr",
    "docs/program/23-runbooks",
    "docs/program/24-milestone-packs",
    "docs/program/25-templates",
    "docs/program/26-policies",
    "docs/program/27-security",
    "docs/_generated",
    "docs/runbooks/operations/program",
    "docs/guides/developer/dev-guide",
]

FORBIDDEN_DOC_FILES = [
    "docs/reference/interfaces-legacy-notes.md",
]


def main() -> int:
    errors: list[str] = []

    for rel in REQUIRED_DIRS:
        if not (ROOT / rel).is_dir():
            errors.append(f"missing required docs dir: {rel}")

    for rel in REQUIRED_READMES:
        if not (ROOT / rel).is_file():
            errors.append(f"missing required docs README: {rel}")

    for rel in FORBIDDEN_DOC_DIRS:
        if (ROOT / rel).exists():
            errors.append(f"forbidden legacy docs area still present: {rel}")

    for rel in FORBIDDEN_DOC_FILES:
        if (ROOT / rel).exists():
            errors.append(f"forbidden legacy docs file still present: {rel}")

    if errors:
        print("docs_hierarchy: FAIL")
        for e in errors:
            print(f" - {e}")
        return 1

    print("docs_hierarchy: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
