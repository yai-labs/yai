#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path
from typing import Any, Dict, List, Set

REPO_ROOT = Path(__file__).resolve().parents[4]
DEFAULT_MANIFEST = REPO_ROOT / "docs" / "proof" / "PP-FOUNDATION-0001" / "pp-foundation-0001.manifest.v1.json"
SHA40_RE = re.compile(r"^[0-9a-f]{40}$")


def die(msg: str, code: int = 2) -> None:
    raise SystemExit(f"[proof-pack] ERROR: {msg}")


def sh(cmd: List[str], cwd: Path = REPO_ROOT) -> str:
    p = subprocess.run(cmd, cwd=str(cwd), stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if p.returncode != 0:
        die(f"command failed: {' '.join(cmd)}\\n{p.stderr.strip()}")
    return p.stdout.strip()


def read_json(path: Path) -> Dict[str, Any]:
    if not path.exists():
        die(f"manifest not found: {path.as_posix()}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as e:
        die(f"invalid JSON in {path.as_posix()}: {e}")


def get_nested(obj: Dict[str, Any], keys: List[str]) -> Any:
    cur: Any = obj
    for k in keys:
        if not isinstance(cur, dict) or k not in cur:
            return None
        cur = cur[k]
    return cur


def require_str(obj: Dict[str, Any], keys: List[str], errs: List[str]) -> str:
    val = get_nested(obj, keys)
    label = ".".join(keys)
    if not isinstance(val, str) or val.strip() == "":
        errs.append(f"missing/invalid string: {label}")
        return ""
    return val


def require_list(obj: Dict[str, Any], keys: List[str], errs: List[str]) -> List[Any]:
    val = get_nested(obj, keys)
    label = ".".join(keys)
    if not isinstance(val, list):
        errs.append(f"missing/invalid list: {label}")
        return []
    return val


def validate_schema(doc: Dict[str, Any]) -> List[str]:
    errs: List[str] = []

    require_str(doc, ["id"], errs)
    require_str(doc, ["schema_version"], errs)
    require_str(doc, ["status"], errs)
    require_str(doc, ["created_at"], errs)

    canonical_repo = require_str(doc, ["canonical_source", "repo"], errs)
    canonical_path = require_str(doc, ["canonical_source", "path"], errs)
    ssot = get_nested(doc, ["canonical_source", "single_source_of_truth"])
    if ssot is not True:
        errs.append("canonical_source.single_source_of_truth must be true")
    if canonical_repo and canonical_repo != "yai":
        errs.append("canonical_source.repo must be 'yai'")
    if canonical_path and not canonical_path.startswith("docs/proof/"):
        errs.append("canonical_source.path must start with docs/proof/")

    include_repos = require_list(doc, ["program_scope", "include_repos"], errs)
    excluded_changes = require_list(doc, ["program_scope", "excluded_changes"], errs)
    if include_repos and "yai" not in include_repos:
        errs.append("program_scope.include_repos must include 'yai'")
    if excluded_changes and "yai-specs" not in excluded_changes:
        errs.append("program_scope.excluded_changes should include 'yai-specs'")

    for repo_key in ["yai", "yai_specs", "yai_cli", "yai_mind"]:
        require_str(doc, ["pins", repo_key, "commit"], errs)

    existing = require_list(doc, ["evidence", "existing"], errs)
    missing = require_list(doc, ["evidence", "missing"], errs)

    existing_ids: Set[str] = set()
    for i, e in enumerate(existing):
        if not isinstance(e, dict):
            errs.append(f"evidence.existing[{i}] must be an object")
            continue
        eid = e.get("id")
        if not isinstance(eid, str) or not eid:
            errs.append(f"evidence.existing[{i}].id is required")
        elif eid in existing_ids:
            errs.append(f"duplicate evidence.existing id: {eid}")
        else:
            existing_ids.add(eid)
        for field in ["claim", "run", "result"]:
            if not isinstance(e.get(field), str) or not e.get(field):
                errs.append(f"evidence.existing[{i}].{field} is required")
        if not isinstance(e.get("paths"), list) or len(e.get("paths", [])) == 0:
            errs.append(f"evidence.existing[{i}].paths must be a non-empty list")

    missing_ids: Set[str] = set()
    for i, m in enumerate(missing):
        if not isinstance(m, dict):
            errs.append(f"evidence.missing[{i}] must be an object")
            continue
        mid = m.get("id")
        if not isinstance(mid, str) or not mid:
            errs.append(f"evidence.missing[{i}].id is required")
        elif mid in missing_ids:
            errs.append(f"duplicate evidence.missing id: {mid}")
        else:
            missing_ids.add(mid)
        if not isinstance(m.get("claim"), str) or not m.get("claim"):
            errs.append(f"evidence.missing[{i}].claim is required")
        if not isinstance(m.get("required_paths"), list) or len(m.get("required_paths", [])) == 0:
            errs.append(f"evidence.missing[{i}].required_paths must be a non-empty list")

    non_skip = require_list(doc, ["gates", "non_skip"], errs)
    skip = require_list(doc, ["gates", "skip"], errs)

    non_skip_ids: Set[str] = set()
    for i, g in enumerate(non_skip):
        if not isinstance(g, dict):
            errs.append(f"gates.non_skip[{i}] must be an object")
            continue
        gid = g.get("id")
        status = g.get("status")
        if not isinstance(gid, str) or not gid:
            errs.append(f"gates.non_skip[{i}].id is required")
        elif gid in non_skip_ids:
            errs.append(f"duplicate gates.non_skip id: {gid}")
        else:
            non_skip_ids.add(gid)
        if status not in ("pass", "fail"):
            errs.append(f"gates.non_skip[{i}].status must be pass|fail, got {status!r}")

    skip_ids: Set[str] = set()
    for i, g in enumerate(skip):
        if not isinstance(g, dict):
            errs.append(f"gates.skip[{i}] must be an object")
            continue
        gid = g.get("id")
        status = g.get("status")
        if not isinstance(gid, str) or not gid:
            errs.append(f"gates.skip[{i}].id is required")
        elif gid in skip_ids:
            errs.append(f"duplicate gates.skip id: {gid}")
        else:
            skip_ids.add(gid)
        if status != "skip":
            errs.append(f"gates.skip[{i}].status must be skip, got {status!r}")

    overlap = non_skip_ids.intersection(skip_ids)
    if overlap:
        errs.append(f"gate ids cannot appear in both non_skip and skip: {sorted(overlap)}")

    return errs


def read_cli_ref(path: Path) -> str:
    if not path.exists():
        die(f"missing deps/yai-cli.ref: {path.as_posix()}")
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith("cli_sha="):
            sha = line.split("=", 1)[1].strip()
            if SHA40_RE.match(sha):
                return sha
            die(f"invalid cli_sha in {path.as_posix()}: {sha}")
    die(f"missing cli_sha=... line in {path.as_posix()}")


def validate_pins(doc: Dict[str, Any], manifest: Path) -> List[str]:
    errs: List[str] = []

    yai_head = sh(["git", "rev-parse", "HEAD"], cwd=REPO_ROOT)
    specs_head = sh(["git", "-C", str(REPO_ROOT / "deps" / "yai-specs"), "rev-parse", "HEAD"], cwd=REPO_ROOT)
    cli_ref = read_cli_ref(REPO_ROOT / "deps" / "yai-cli.ref")

    declared_yai = str(get_nested(doc, ["pins", "yai", "commit"]) or "")
    declared_specs = str(get_nested(doc, ["pins", "yai_specs", "commit"]) or "")
    declared_cli = str(get_nested(doc, ["pins", "yai_cli", "commit"]) or "")
    declared_mind = str(get_nested(doc, ["pins", "yai_mind", "commit"]) or "")

    for label, sha in [
        ("pins.yai.commit", declared_yai),
        ("pins.yai_specs.commit", declared_specs),
        ("pins.yai_cli.commit", declared_cli),
        ("pins.yai_mind.commit", declared_mind),
    ]:
        if not SHA40_RE.match(sha):
            errs.append(f"{label} must be a 40-char lowercase git sha")

    # Self-pin cannot be kept equal to HEAD across new commits without immediate drift.
    # Keep format validation for pins.yai.commit and enforce cross-repo pins (specs/cli).

    if SHA40_RE.match(declared_specs) and declared_specs != specs_head:
        errs.append(
            f"pins.yai_specs.commit mismatch (manifest={declared_specs[:12]} actual={specs_head[:12]})"
        )

    if SHA40_RE.match(declared_cli) and declared_cli != cli_ref:
        errs.append(
            f"pins.yai_cli.commit mismatch (manifest={declared_cli[:12]} actual={cli_ref[:12]})"
        )

    mind_pin_source = str(get_nested(doc, ["pins", "yai_mind", "pin_source"]) or "")
    if mind_pin_source == "":
        errs.append("pins.yai_mind.pin_source is required")

    canonical_path = str(get_nested(doc, ["canonical_source", "path"]) or "")
    expected_manifest = REPO_ROOT / canonical_path / manifest.name
    if canonical_path and expected_manifest.resolve() != manifest.resolve():
        errs.append(
            "manifest path mismatch with canonical_source.path "
            f"(declared={expected_manifest.relative_to(REPO_ROOT).as_posix()} actual={manifest.relative_to(REPO_ROOT).as_posix()})"
        )

    return errs


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate proof pack manifest schema and pins")
    parser.add_argument(
        "--manifest",
        type=Path,
        default=DEFAULT_MANIFEST,
        help="Path to proof pack manifest JSON",
    )
    parser.add_argument(
        "--allow-skip",
        action="store_true",
        help="Allow SKIP for private/missing manifests (local draft mode)",
    )
    args = parser.parse_args()

    manifest = args.manifest if args.manifest.is_absolute() else (REPO_ROOT / args.manifest)
    rel_manifest = manifest.relative_to(REPO_ROOT).as_posix()
    if "/.private/" in f"/{rel_manifest}":
        if args.allow_skip:
            print(f"[proof-pack] SKIP: private draft manifest ({rel_manifest})")
            print("[proof-pack] SKIP: publish under docs/proof/<PACK-ID>/ to enforce proof-pack gates")
            return 0
        print(f"[proof-pack] FAIL: private draft manifest is not allowed for mandatory checks ({rel_manifest})")
        print("[proof-pack] FAIL: publish under docs/proof/<PACK-ID>/ or pass --allow-skip for local draft checks")
        return 2
    if not manifest.exists() and manifest.resolve() == DEFAULT_MANIFEST.resolve():
        if args.allow_skip:
            print(f"[proof-pack] SKIP: default manifest not found ({rel_manifest})")
            print("[proof-pack] SKIP: keep draft packs under docs/proof/.private/ until publication")
            return 0
        print(f"[proof-pack] FAIL: default manifest not found ({rel_manifest})")
        print("[proof-pack] FAIL: add the public manifest under docs/proof/<PACK-ID>/ or use --allow-skip for local draft checks")
        return 2
    doc = read_json(manifest)

    errors = validate_schema(doc)
    errors.extend(validate_pins(doc, manifest.resolve()))

    if errors:
        print("[proof-pack] FAIL")
        for e in errors:
            print(f" - {e}")
        return 2

    print("[proof-pack] PASS")
    print(f"manifest={manifest.relative_to(REPO_ROOT).as_posix()}")
    print(f"yai_commit={get_nested(doc, ['pins', 'yai', 'commit'])}")
    print(f"yai_specs_commit={get_nested(doc, ['pins', 'yai_specs', 'commit'])}")
    print(f"yai_cli_commit={get_nested(doc, ['pins', 'yai_cli', 'commit'])}")
    print(f"yai_mind_commit={get_nested(doc, ['pins', 'yai_mind', 'commit'])}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
