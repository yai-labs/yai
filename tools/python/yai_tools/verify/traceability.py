#!/usr/bin/env python3
from __future__ import annotations

import argparse
import os
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Tuple, Optional, Any

REPO_ROOT = Path(__file__).resolve().parents[4]  # tools/python/yai_tools/verify/traceability.py -> repo root

DOCS_ROOT = REPO_ROOT / "docs"
ADR_DIR = DOCS_ROOT / "20-governance" / "design" / "adr"
RUNBOOK_DIR = DOCS_ROOT / "20-governance" / "runbooks"
MP_DIR = DOCS_ROOT / "20-governance" / "milestone-packs"

FM_DELIM = "---"

def die(msg: str, code: int = 2) -> None:
    raise SystemExit(f"[traceability] ERROR: {msg}")

def sh(cmd: List[str], cwd: Path = REPO_ROOT) -> str:
    p = subprocess.run(cmd, cwd=str(cwd), stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if p.returncode != 0:
        die(f"command failed: {' '.join(cmd)}\n{p.stderr.strip()}")
    return p.stdout.strip()

def read_text(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8")
    except FileNotFoundError:
        die(f"missing file: {p.as_posix()}")

def parse_frontmatter(md: str) -> Dict[str, Any]:
    """
    Minimal YAML frontmatter parser (no dependencies).
    Supports:
      key: value
      key:
        - item
        - item
    """
    md = md.lstrip("\ufeff")
    if not md.startswith(FM_DELIM):
        return {}

    parts = md.split(FM_DELIM, 2)
    # parts: ["", "\n...frontmatter...\n", "\n...body..."]
    if len(parts) < 3:
        return {}

    fm = parts[1].strip("\n")
    data: Dict[str, Any] = {}
    current_key: Optional[str] = None

    for raw in fm.splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue

        if line.startswith("- "):
            if current_key is None:
                continue
            data.setdefault(current_key, [])
            if not isinstance(data[current_key], list):
                data[current_key] = []
            data[current_key].append(line[2:].strip().strip('"').strip("'"))
            continue

        if ":" in line:
            k, v = line.split(":", 1)
            k = k.strip()
            v = v.strip()
            if v == "":
                data[k] = []
                current_key = k
            else:
                data[k] = v.strip('"').strip("'")
                current_key = k

    return data

def md_body(md: str) -> str:
    md = md.lstrip("\ufeff")
    if not md.startswith(FM_DELIM):
        return md
    parts = md.split(FM_DELIM, 2)
    if len(parts) < 3:
        return md
    return parts[2]

def rel(p: Path) -> str:
    try:
        return p.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return p.as_posix()

@dataclass
class CheckResult:
    ok: bool
    errors: List[str]

def is_md_under(path: Path, base: Path) -> bool:
    try:
        path.relative_to(base)
        return True
    except ValueError:
        return False

def changed_files(base_sha: str, head_sha: str) -> List[Path]:
    out = sh(["git", "diff", "--name-only", f"{base_sha}...{head_sha}"])
    files = []
    for line in out.splitlines():
        if not line.strip():
            continue
        p = (REPO_ROOT / line.strip()).resolve()
        if p.suffix.lower() == ".md":
            files.append(p)
    return files

def classify_changed(files: List[Path]) -> Tuple[List[Path], List[Path], List[Path]]:
    adrs: List[Path] = []
    runbooks: List[Path] = []
    mps: List[Path] = []
    for f in files:
        if is_md_under(f, ADR_DIR):
            adrs.append(f)
        elif is_md_under(f, RUNBOOK_DIR):
            runbooks.append(f)
        elif is_md_under(f, MP_DIR):
            mps.append(f)
    return adrs, runbooks, mps

def ensure_list(v: Any) -> List[str]:
    if v is None:
        return []
    if isinstance(v, list):
        return [str(x) for x in v]
    if isinstance(v, str):
        return [v]
    return []

def check_adr(path: Path) -> CheckResult:
    txt = read_text(path)
    fm = parse_frontmatter(txt)
    errs: List[str] = []

    if not fm:
        errs.append("missing YAML frontmatter (--- ... ---).")
        return CheckResult(False, errs)

    adr_id = str(fm.get("id", "")).strip()
    if not adr_id.startswith("ADR-"):
        errs.append("frontmatter `id` must start with `ADR-`.")
    status = str(fm.get("status", "")).strip()
    if status == "":
        errs.append("frontmatter `status` is required (e.g. active/draft/superseded).")

    law_refs = ensure_list(fm.get("law_refs"))
    if len(law_refs) == 0:
        errs.append("frontmatter `law_refs` must be non-empty and point to deps/yai-specs/...")
    else:
        for r in law_refs:
            if not r.startswith("deps/yai-specs/"):
                errs.append(f"law_ref must start with `deps/yai-specs/` but got: {r}")
            rp = (REPO_ROOT / r).resolve()
            if not rp.exists():
                errs.append(f"law_ref path not found: {r}")

    return CheckResult(len(errs) == 0, errs)

def check_runbook(path: Path) -> CheckResult:
    txt = read_text(path)
    fm = parse_frontmatter(txt)
    body = md_body(txt)
    errs: List[str] = []

    if not fm:
        errs.append("missing YAML frontmatter (--- ... ---).")
        return CheckResult(False, errs)

    rb_id = str(fm.get("id", "")).strip()
    if not (rb_id.startswith("RB-") or rb_id.startswith("RB_")):
        errs.append("frontmatter `id` must start with `RB-` (recommended).")

    status = str(fm.get("status", "")).strip()
    if status == "":
        errs.append("frontmatter `status` is required (e.g. active/draft/superseded).")

    # ops-only exception
    ops_only = str(fm.get("ops_only", "")).lower() in ("true", "1", "yes")

    adr_refs = ensure_list(fm.get("adr_refs"))
    if not ops_only and len(adr_refs) == 0:
        errs.append("frontmatter `adr_refs` required unless ops_only=true.")
    for r in adr_refs:
        rp = (REPO_ROOT / r).resolve()
        if not rp.exists():
            errs.append(f"adr_ref path not found: {r}")

    # runbook must be linkable: it must at least mention "Milestone Pack" section if MP files reference it
    # (hard enforced in MP checker via substring check)

    return CheckResult(len(errs) == 0, errs)

def check_mp(path: Path) -> CheckResult:
    txt = read_text(path)
    fm = parse_frontmatter(txt)
    errs: List[str] = []

    if not fm:
        errs.append("missing YAML frontmatter (--- ... ---).")
        return CheckResult(False, errs)

    mp_id = str(fm.get("id", "")).strip()
    if not mp_id.startswith("MP-"):
        errs.append("frontmatter `id` must start with `MP-`.")
    runbook = str(fm.get("runbook", "")).strip()
    if runbook == "":
        errs.append("frontmatter `runbook` is required (repo-relative path).")
    else:
        rbp = (REPO_ROOT / runbook).resolve()
        if not rbp.exists():
            errs.append(f"runbook path not found: {runbook}")
        else:
            # HARD RULE: runbook must contain the MP id (prevents separation)
            rb_txt = read_text(rbp)
            if mp_id and mp_id not in rb_txt:
                errs.append(f"runbook does not mention MP id `{mp_id}` (must include it to link bidirectionally).")

    phase = str(fm.get("phase", "")).strip()
    if phase == "":
        errs.append("frontmatter `phase` is required (e.g. 0.1.0 — Protocol Guardrails).")

    adrs = ensure_list(fm.get("adrs"))
    if len(adrs) == 0:
        errs.append("frontmatter `adrs` must be a non-empty list of ADR paths.")
    else:
        for a in adrs:
            ap = (REPO_ROOT / a).resolve()
            if not ap.exists():
                errs.append(f"adr path not found: {a}")

    spec_anchors = ensure_list(fm.get("spec_anchors"))
    if len(spec_anchors) == 0:
        errs.append("frontmatter `spec_anchors` must be non-empty and point to deps/yai-specs/...")

    for s in spec_anchors:
        if not s.startswith("deps/yai-specs/"):
            errs.append(f"spec_anchor must start with deps/yai-specs/ but got: {s}")
        sp = (REPO_ROOT / s).resolve()
        if not sp.exists():
            errs.append(f"spec_anchor path not found: {s}")

    issues = ensure_list(fm.get("issues"))
    if len(issues) == 0:
        errs.append("frontmatter `issues` required: list of #NNN or N/A with reason.")
    else:
        if any(x.upper() == "N/A" for x in issues):
            reason = str(fm.get("issue_reason", "")).strip()
            if reason == "":
                errs.append("issues includes N/A but `issue_reason` is missing.")

    return CheckResult(len(errs) == 0, errs)

def main() -> int:
    ap = argparse.ArgumentParser(prog="yai-docs-trace-check")
    ap.add_argument("--all", action="store_true", help="check all ADR/Runbook/MP docs (strict)")
    ap.add_argument("--changed", action="store_true", help="check only changed docs between base..head")
    ap.add_argument("--base", default="", help="base sha for --changed")
    ap.add_argument("--head", default="", help="head sha for --changed (defaults to HEAD)")
    args = ap.parse_args()

    if args.all and args.changed:
        die("choose one: --all OR --changed")

    to_check: List[Path] = []

    if args.all:
        to_check += list(ADR_DIR.rglob("*.md"))
        to_check += list(RUNBOOK_DIR.rglob("*.md"))
        to_check += list(MP_DIR.rglob("*.md"))
    else:
        # default = changed mode (safer for early adoption)
        if not args.changed:
            args.changed = True
        base = args.base.strip()
        head = args.head.strip() or "HEAD"
        if base == "":
            die("--changed requires --base <sha> (in CI use PR base sha).")

        files = changed_files(base, head)
        adrs, runbooks, mps = classify_changed(files)
        to_check += adrs + runbooks + mps

    # If nothing relevant changed, pass.
    relevant = [p for p in to_check if p.exists()]
    if len(relevant) == 0:
        print("[traceability] OK: no relevant docs changed.")
        return 0

    failures: List[str] = []

    for p in relevant:
        rp = rel(p)
        if is_md_under(p, ADR_DIR):
            res = check_adr(p)
        elif is_md_under(p, RUNBOOK_DIR):
            res = check_runbook(p)
        elif is_md_under(p, MP_DIR):
            res = check_mp(p)
        else:
            continue

        if not res.ok:
            failures.append(f"- {rp}")
            for e in res.errors:
                failures.append(f"  - {e}")

    if failures:
        print("[traceability] FAIL:\n" + "\n".join(failures))
        return 1

    print(f"[traceability] OK: checked {len(relevant)} file(s).")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
