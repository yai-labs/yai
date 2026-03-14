#!/usr/bin/env python3
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

TEST_PATTERNS = [
    re.compile(r"_v\d+\.sh$"),
    re.compile(r"_dp\d+[a-z]*"),
    re.compile(r"_wsv\d+"),
    re.compile(r"(^|/)ql_"),
    re.compile(r"(^|/)run_qw\d"),
]

TOOL_PATTERNS = [
    re.compile(r"validate_unified_repo_root_framing"),
    re.compile(r"validate_governance_manifest_spine"),
    re.compile(r"validate_no_legacy_tooling_references"),
    re.compile(r"build_overlay_compliance_runtime_view"),
]

def find_violations(base: str, globs: list[str], patterns: list[re.Pattern[str]]) -> list[str]:
    out: list[str] = []
    for g in globs:
        for p in sorted((ROOT / base).glob(g)):
            rel = p.relative_to(ROOT).as_posix()
            for rx in patterns:
                if rx.search(rel):
                    out.append(rel)
                    break
    return out


def main() -> int:
    violations: list[str] = []

    violations.extend(find_violations("tests", ["**/*.sh"], TEST_PATTERNS))
    violations.extend(find_violations("tools", ["**/*.py", "**/*.sh", "**/*"], TOOL_PATTERNS))
    if violations:
        print("aux naming validation: FAIL")
        for v in sorted(set(violations)):
            print(f" - {v}")
        return 1

    print("aux naming validation: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
