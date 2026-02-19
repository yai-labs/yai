from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from yai_tools.verify.traceability import (
    ADR_DIR,
    MP_DIR,
    REPO_ROOT,
    RUNBOOK_DIR,
    changed_files,
    parse_frontmatter,
)

SCHEMA_DIR = REPO_ROOT / "tools" / "schemas" / "docs"
PROPOSAL_DIR = REPO_ROOT / "docs" / "design" / "proposals"


def _load(name: str) -> dict[str, Any]:
    return json.loads((SCHEMA_DIR / name).read_text(encoding="utf-8"))


def _ensure_list(v: Any) -> list[str]:
    if isinstance(v, list):
        return [str(x) for x in v]
    if isinstance(v, str):
        return [v]
    return []


def _check_enum(val: Any, enum: list[str]) -> bool:
    return str(val) in enum


def _check_pattern(val: Any, pattern: str) -> bool:
    return re.match(pattern, str(val)) is not None


def _validate_frontmatter(fm: dict[str, Any], schema: dict[str, Any], relpath: str) -> list[str]:
    errs: list[str] = []
    required = schema.get("required", [])
    props = schema.get("properties", {})

    for k in required:
        if k not in fm or fm.get(k) in ("", [], None):
            errs.append(f"{relpath}: missing required frontmatter key `{k}`")

    for key, rules in props.items():
        if key not in fm:
            continue
        val = fm[key]
        t = rules.get("type")
        if t == "array":
            arr = _ensure_list(val)
            if rules.get("minItems") and len(arr) < int(rules["minItems"]):
                errs.append(f"{relpath}: `{key}` must contain at least {rules['minItems']} item(s)")
            item_rules = rules.get("items", {})
            pat = item_rules.get("pattern")
            if pat:
                for i in arr:
                    if not _check_pattern(i, pat):
                        errs.append(f"{relpath}: `{key}` item does not match pattern `{pat}`: {i}")
        else:
            if "enum" in rules and not _check_enum(val, rules["enum"]):
                errs.append(f"{relpath}: `{key}` must be one of {rules['enum']}")
            if "pattern" in rules and not _check_pattern(val, rules["pattern"]):
                errs.append(f"{relpath}: `{key}` does not match pattern `{rules['pattern']}`")

    return errs


def _classify(path: Path) -> tuple[str, dict[str, Any]] | None:
    if path.is_relative_to(ADR_DIR):
        return "adr", _load("frontmatter.adr.v1.schema.json")
    if path.is_relative_to(RUNBOOK_DIR):
        return "runbook", _load("frontmatter.runbook.v1.schema.json")
    if path.is_relative_to(MP_DIR):
        return "mp", _load("frontmatter.milestone-pack.v1.schema.json")
    if path.is_relative_to(PROPOSAL_DIR):
        return "proposal", _load("frontmatter.proposal.v1.schema.json")
    return None


def run_schema_check(changed: bool, base: str, head: str) -> int:
    files: list[Path]
    if changed:
        files = changed_files(base, head)
    else:
        files = list(ADR_DIR.rglob("*.md")) + list(RUNBOOK_DIR.rglob("*.md")) + list(MP_DIR.rglob("*.md")) + list(
            PROPOSAL_DIR.rglob("*.md")
        )

    failures: list[str] = []
    for p in sorted(set(files)):
        c = _classify(p)
        if not c:
            continue
        _, schema = c
        relpath = p.relative_to(REPO_ROOT).as_posix()
        fm = parse_frontmatter(p.read_text(encoding="utf-8"))
        if not fm:
            failures.append(f"{relpath}: missing YAML frontmatter")
            continue
        failures.extend(_validate_frontmatter(fm, schema, relpath))

    if failures:
        print("[docs-schema] FAIL:")
        for e in failures:
            print(f"- {e}")
        return 1

    print("[docs-schema] OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(prog="yai-docs-schema-check")
    ap.add_argument("--changed", action="store_true")
    ap.add_argument("--base", default="")
    ap.add_argument("--head", default="HEAD")
    args = ap.parse_args()

    if args.changed and not args.base:
        print("[docs-schema] ERROR: --changed requires --base")
        return 2

    return run_schema_check(args.changed, args.base, args.head)


if __name__ == "__main__":
    raise SystemExit(main())
