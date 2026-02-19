from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path
from typing import Any

from yai_tools._core.paths import repo_root
from yai_tools.verify.generated_sync import check_json_synced, write_json
from yai_tools.verify.traceability import parse_frontmatter

REPO_ROOT = repo_root()
ARCH_DIR = REPO_ROOT / "docs" / "architecture"
COMPONENTS_DIR = ARCH_DIR / "components"
TRACEABILITY_DOC = ARCH_DIR / "traceability.md"
OVERVIEW_DOC = ARCH_DIR / "overview.md"
RUNTIME_MODEL_DOC = ARCH_DIR / "runtime-model.md"
GENERATED_ALIGNMENT = REPO_ROOT / "docs" / "_generated" / "architecture-alignment.v1.json"
SCHEMA_PATH = REPO_ROOT / "tools" / "schemas" / "docs" / "architecture.alignment.v1.schema.json"

ALLOWED_COMPONENT_STATUS = {"implemented", "partial", "planned/external"}
REQUIRED_FRONTMATTER_KEYS = ["id", "status", "effective_date", "revision", "owner", "law_refs"]
REQUIRED_COMPONENT_SECTIONS = [
    "Role",
    "Current Implementation Status",
    "Interfaces and Entry Points",
    "Authority and Boundary Rules",
    "Traceability",
    "Known Drift / Gaps",
    "Next Alignment Steps",
]


def _run_git(args: list[str]) -> str:
    proc = subprocess.run(["git", *args], cwd=str(REPO_ROOT), text=True, capture_output=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or "git command failed")
    return proc.stdout.strip()


def _md_body(text: str) -> str:
    text = text.lstrip("\ufeff")
    if not text.startswith("---"):
        return text
    parts = text.split("---", 2)
    if len(parts) < 3:
        return text
    return parts[2]


def _section_map(md_body: str) -> dict[str, str]:
    sections: dict[str, list[str]] = {}
    current: str | None = None
    for line in md_body.splitlines():
        if line.startswith("## "):
            current = line[3:].strip()
            sections[current] = []
            continue
        if current is not None:
            sections[current].append(line)
    return {k: "\n".join(v).strip() for k, v in sections.items()}


def _extract_backtick_refs(text: str) -> list[str]:
    return [m.group(1).strip() for m in re.finditer(r"`([^`]+)`", text)]


def _normalize_ref(ref: str) -> str:
    out = ref.strip()
    if "#" in out:
        out = out.split("#", 1)[0]
    return out.strip()


def _is_absolute_ref(ref: str) -> bool:
    return ref.startswith("/") or bool(re.match(r"^[A-Za-z]:\\\\", ref))


def _path_exists(ref: str) -> bool:
    if not ref or ref.startswith("~") or "..." in ref or "<" in ref or ">" in ref:
        return True
    if "*" in ref:
        return any(REPO_ROOT.glob(ref))
    return (REPO_ROOT / ref).exists()


def _extract_refs_by_prefix(text: str, prefixes: tuple[str, ...]) -> list[str]:
    refs = []
    for raw in _extract_backtick_refs(text):
        r = _normalize_ref(raw)
        if not r:
            continue
        if r.startswith(prefixes):
            refs.append(r)
    return sorted(set(refs))


def _topology_line(path: Path) -> str:
    text = path.read_text(encoding="utf-8")
    for line in text.splitlines():
        if line.strip().startswith("Canonical Topology:"):
            return re.sub(r"\s+", " ", line.strip())
    return ""


def _changed_paths(base: str, head: str) -> list[str]:
    out = _run_git(["diff", "--name-only", f"{base}...{head}"])
    return [x.strip() for x in out.splitlines() if x.strip()]


def _mind_impl_present() -> bool:
    mind_dir = REPO_ROOT / "mind"
    if not mind_dir.exists():
        return False
    for sub in ["src", "include"]:
        p = mind_dir / sub
        if p.exists() and any(x.is_file() for x in p.rglob("*")):
            return True
    return False


def _parse_component_doc(path: Path) -> dict[str, Any]:
    rel = path.relative_to(REPO_ROOT).as_posix()
    text = path.read_text(encoding="utf-8")
    fm = parse_frontmatter(text)
    body = _md_body(text)
    sections = _section_map(body)

    impl_status = sections.get("Current Implementation Status", "").strip().lower()
    traceability = sections.get("Traceability", "")

    return {
        "name": path.stem,
        "path": rel,
        "frontmatter": fm,
        "sections": sections,
        "impl_status": impl_status,
        "adr_refs": _extract_refs_by_prefix(traceability, ("docs/design/adr/",)),
        "runbook_refs": _extract_refs_by_prefix(traceability, ("docs/runbooks/",)),
        "mp_refs": _extract_refs_by_prefix(traceability, ("docs/milestone-packs/",)),
        "l0_refs": _extract_refs_by_prefix(traceability, ("deps/yai-specs/",)),
    }


def _parse_traceability_rows(path: Path) -> list[dict[str, Any]]:
    text = path.read_text(encoding="utf-8")
    rows: list[dict[str, Any]] = []
    for raw in text.splitlines():
        line = raw.strip()
        if not line.startswith("|"):
            continue
        if line.startswith("|---") or line.startswith("| Component"):
            continue
        cols = [c.strip() for c in line.strip("|").split("|")]
        if len(cols) < 6:
            continue
        rows.append(
            {
                "component": cols[0],
                "status": cols[1].lower(),
                "adr_refs": _extract_refs_by_prefix(cols[2], ("docs/design/adr/",)),
                "runbook_refs": _extract_refs_by_prefix(cols[3], ("docs/runbooks/",)),
                "mp_refs": _extract_refs_by_prefix(cols[4], ("docs/milestone-packs/",)),
                "l0_refs": _extract_refs_by_prefix(cols[5], ("deps/yai-specs/",)),
                "is_planned_marker": cols[1].strip().lower() == "planned/external",
            }
        )
    return rows


def _validate_schema_like(obj: dict[str, Any], errors: list[str]) -> None:
    required = ["version", "canonical_topology", "components", "traceability_rows"]
    for key in required:
        if key not in obj:
            errors.append(f"generated alignment missing key: {key}")

    if not isinstance(obj.get("version"), int):
        errors.append("generated alignment `version` must be integer")
    if not isinstance(obj.get("canonical_topology"), str) or not obj["canonical_topology"].startswith("Canonical Topology:"):
        errors.append("generated alignment `canonical_topology` must be Canonical Topology line")

    comps = obj.get("components")
    if not isinstance(comps, list):
        errors.append("generated alignment `components` must be array")
        comps = []

    for c in comps:
        if not isinstance(c, dict):
            errors.append("generated alignment component entry must be object")
            continue
        for key in ["name", "path", "status", "adr_refs", "runbook_refs", "mp_refs", "l0_refs"]:
            if key not in c:
                errors.append(f"generated alignment component missing key `{key}`")
        if c.get("status") not in ALLOWED_COMPONENT_STATUS:
            errors.append(f"generated alignment component status invalid: {c.get('status')}")


def build_alignment_snapshot() -> tuple[dict[str, Any], list[str]]:
    errors: list[str] = []

    if not SCHEMA_PATH.exists():
        errors.append(f"missing schema: {SCHEMA_PATH.relative_to(REPO_ROOT).as_posix()}")
    else:
        try:
            json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            errors.append(f"invalid schema JSON: {exc}")

    component_paths = sorted(COMPONENTS_DIR.glob("*.md"))
    if not component_paths:
        errors.append("no component docs found under docs/architecture/components")

    overview_topology = _topology_line(OVERVIEW_DOC)
    runtime_topology = _topology_line(RUNTIME_MODEL_DOC)
    if not overview_topology or not runtime_topology:
        errors.append("missing Canonical Topology line in overview.md or runtime-model.md")
    elif overview_topology != runtime_topology:
        errors.append("overview.md and runtime-model.md disagree on Canonical Topology")

    component_entries: list[dict[str, Any]] = []
    for path in component_paths:
        doc = _parse_component_doc(path)
        rel = doc["path"]
        fm = doc["frontmatter"]
        sections = doc["sections"]

        for key in REQUIRED_FRONTMATTER_KEYS:
            if key not in fm or fm[key] in (None, "", []):
                errors.append(f"{rel}: missing required frontmatter key `{key}`")

        law_refs = fm.get("law_refs", [])
        if not isinstance(law_refs, list) or not law_refs:
            errors.append(f"{rel}: frontmatter `law_refs` must be non-empty list")
        else:
            for ref in law_refs:
                if not isinstance(ref, str):
                    errors.append(f"{rel}: non-string law_ref")
                    continue
                if _is_absolute_ref(ref):
                    errors.append(f"{rel}: absolute path not allowed in law_refs: {ref}")
                elif not _path_exists(ref):
                    errors.append(f"{rel}: law_ref path not found: {ref}")

        for req in REQUIRED_COMPONENT_SECTIONS:
            if req not in sections or not sections[req].strip():
                errors.append(f"{rel}: missing required section `## {req}`")

        impl_status = doc["impl_status"]
        if impl_status not in ALLOWED_COMPONENT_STATUS:
            errors.append(f"{rel}: invalid implementation status `{impl_status}`")

        if path.stem == "mind" and impl_status == "implemented" and not _mind_impl_present():
            errors.append(f"{rel}: claims implemented but local `mind` implementation is absent")

        trace_refs = doc["adr_refs"] + doc["runbook_refs"] + doc["mp_refs"] + doc["l0_refs"]
        for ref in trace_refs:
            if _is_absolute_ref(ref):
                errors.append(f"{rel}: absolute path not allowed: {ref}")
            elif not _path_exists(ref):
                errors.append(f"{rel}: traceability path not found: {ref}")

        # validate interface entry paths when they look like repo paths
        interfaces = sections.get("Interfaces and Entry Points", "")
        for token in _extract_backtick_refs(interfaces):
            ref = _normalize_ref(token)
            if not ref or ref.startswith("~"):
                continue
            if "/" not in ref:
                continue
            if _is_absolute_ref(ref):
                errors.append(f"{rel}: absolute path not allowed in interfaces: {ref}")
                continue
            # only enforce existence for obvious repo-file patterns
            if any(ref.endswith(sfx) for sfx in [".md", ".c", ".h", ".rs", ".json", ".sh"]) or "*" in ref:
                if not _path_exists(ref):
                    errors.append(f"{rel}: interface path not found: {ref}")

        component_entries.append(
            {
                "name": path.stem,
                "path": rel,
                "status": impl_status,
                "adr_refs": doc["adr_refs"],
                "runbook_refs": doc["runbook_refs"],
                "mp_refs": doc["mp_refs"],
                "l0_refs": doc["l0_refs"],
            }
        )

    trace_rows = _parse_traceability_rows(TRACEABILITY_DOC)
    if not trace_rows:
        errors.append("docs/architecture/traceability.md: no alignment table rows found")

    component_names = {c["name"].lower() for c in component_entries}
    trace_names = {r["component"].lower() for r in trace_rows}

    missing_rows = sorted(component_names - trace_names)
    for name in missing_rows:
        errors.append(f"traceability table missing component row: {name}")

    for row in trace_rows:
        if row["status"] not in ALLOWED_COMPONENT_STATUS:
            errors.append(f"traceability row `{row['component']}` has invalid status `{row['status']}`")

        for ref in row["adr_refs"] + row["runbook_refs"] + row["mp_refs"] + row["l0_refs"]:
            if _is_absolute_ref(ref):
                errors.append(f"traceability row `{row['component']}` uses absolute path: {ref}")
            elif not _path_exists(ref):
                errors.append(f"traceability row `{row['component']}` broken path: {ref}")

        if not row["adr_refs"] and not row["is_planned_marker"]:
            errors.append(
                f"traceability row `{row['component']}` is orphan: missing ADR refs without planned/external marker"
            )

    # global path checks for architecture docs
    for md_path in sorted(ARCH_DIR.rglob("*.md")):
        rel = md_path.relative_to(REPO_ROOT).as_posix()
        txt = md_path.read_text(encoding="utf-8")

        for bt in _extract_backtick_refs(txt):
            ref = _normalize_ref(bt)
            if _is_absolute_ref(ref):
                errors.append(f"{rel}: absolute path not allowed: {ref}")

        for m in re.finditer(r"\]\((/[^)]+)\)", txt):
            errors.append(f"{rel}: absolute markdown link not allowed: {m.group(1)}")

        # validate all listed ADR/Runbook/MP/L0 refs anywhere in architecture docs
        for ref in _extract_refs_by_prefix(txt, ("docs/design/adr/", "docs/runbooks/", "docs/milestone-packs/", "deps/yai-specs/")):
            if not _path_exists(ref):
                errors.append(f"{rel}: referenced path not found: {ref}")

    snapshot = {
        "version": 1,
        "canonical_topology": overview_topology,
        "components": sorted(component_entries, key=lambda x: x["name"]),
        "traceability_rows": sorted(trace_rows, key=lambda x: x["component"].lower()),
    }
    _validate_schema_like(snapshot, errors)
    return snapshot, sorted(set(errors))


def run_architecture_alignment(mode: str, base: str, head: str, write: bool) -> int:
    if mode == "changed" and not base:
        print("[architecture-check] ERROR: --changed requires --base <sha>")
        return 2

    if mode == "changed":
        # keep interface parity and basic signal in logs; check remains full to prevent drift.
        changed = _changed_paths(base=base, head=head)
        print(f"[architecture-check] changed files: {len(changed)}")

    snapshot, errors = build_alignment_snapshot()

    if errors:
        print("[architecture-check] FAIL:")
        for err in errors:
            print(f"- {err}")
        return 1

    if write:
        write_json(GENERATED_ALIGNMENT, snapshot)
        print("[architecture-check] OK: generated alignment snapshot updated")
        return 0

    ok, msg = check_json_synced(GENERATED_ALIGNMENT, snapshot)
    if not ok:
        print("[architecture-check] FAIL:")
        print(f"- {msg}")
        return 1

    print("[architecture-check] OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(prog="yai-architecture-check")
    mode = ap.add_mutually_exclusive_group(required=True)
    mode.add_argument("--changed", action="store_true")
    mode.add_argument("--all", action="store_true")
    mode.add_argument("--write", action="store_true")
    ap.add_argument("--base", default="")
    ap.add_argument("--head", default="HEAD")
    args = ap.parse_args()

    run_mode = "all"
    if args.changed:
        run_mode = "changed"

    return run_architecture_alignment(mode=run_mode, base=args.base, head=args.head, write=args.write)


if __name__ == "__main__":
    raise SystemExit(main())
