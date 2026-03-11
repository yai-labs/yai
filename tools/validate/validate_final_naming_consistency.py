#!/usr/bin/env python3
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

FILENAME_RULES = [
    (Path("tests/integration/workspace"), re.compile(r"_v\d+\.sh$"), "workspace integration scripts must not use version suffixes"),
    (Path("tests/integration/source-plane"), re.compile(r"_v\d+\.sh$"), "source-plane integration scripts must not use version suffixes"),
    (Path("tests/integration/qualification/lan"), re.compile(r"(^ql_|^run_qw|_v\d+\.sh$)"), "LAN qualification scripts must use canonical lan_* / run_lan_wave naming"),
]

FORBIDDEN_PATH_TOKENS = [
    "tests/integration/workspace_lifecycle/",
    "tests/integration/source_plane/",
    "validate_overlay_compliance_runtime_view.py",
    "validate_unified_repo_root_framing.py",
    "validate_governance_manifest_spine.py",
    "validate_no_legacy_tooling_references.py",
    "build_overlay_compliance_runtime_view.py",
]


def scan_filename_rules() -> list[str]:
    errors: list[str] = []
    for base, pattern, msg in FILENAME_RULES:
        full = ROOT / base
        if not full.exists():
            continue
        for p in sorted(full.glob("*.sh")):
            if pattern.search(p.name):
                errors.append(f"{p.relative_to(ROOT).as_posix()}: {msg}")
    return errors


def scan_path_tokens() -> list[str]:
    errors: list[str] = []
    scan_roots = [
        ROOT / "Makefile",
        ROOT / "tests",
        ROOT / "tools",
        ROOT / "docs",
    ]
    files: list[Path] = []
    for r in scan_roots:
        if not r.exists():
            continue
        if r.is_file():
            files.append(r)
        else:
            files.extend([p for p in r.rglob("*") if p.is_file()])

    for f in files:
        rel = f.relative_to(ROOT).as_posix()
        if rel == "tools/validate/validate_final_naming_consistency.py":
            continue
        text = f.read_text(encoding="utf-8", errors="ignore")
        for token in FORBIDDEN_PATH_TOKENS:
            if token in text:
                errors.append(f"{rel}: contains forbidden legacy naming token '{token}'")
    return errors


def main() -> int:
    errors = scan_filename_rules() + scan_path_tokens()
    if errors:
        print("final_naming_consistency: FAIL")
        for e in sorted(set(errors)):
            print(f" - {e}")
        return 1

    print("final_naming_consistency: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
