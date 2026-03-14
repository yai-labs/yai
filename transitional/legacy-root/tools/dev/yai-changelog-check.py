#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import subprocess
from pathlib import Path
from typing import Dict, List, Optional, Set

REPO_ROOT = Path(__file__).resolve().parents[2]
ALLOWED_KAC_SECTIONS = {"Added", "Changed", "Deprecated", "Removed", "Fixed", "Security"}
PLACEHOLDER_RE = re.compile(r"\b(TODO|TBD|lorem ipsum|to be done)\b|<[^>]+>|\.\.\.", re.IGNORECASE)
VAGUE_BULLET_RE = re.compile(r"^\s*-\s*(update stuff|misc)\s*$", re.IGNORECASE)

META_PREFIXES = ("docs/", ".github/", "tools/")
META_FILES = {
    "README.md",
    "LICENSE",
    "NOTICE",
    "CODE_OF_CONDUCT.md",
    "SECURITY.md",
    "CONTRIBUTING.md",
}


def run(cmd: List[str]) -> str:
    p = subprocess.run(cmd, cwd=REPO_ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if p.returncode != 0:
        raise SystemExit(f"[changelog] ERROR: command failed: {' '.join(cmd)}\n{p.stderr.strip()}")
    return p.stdout


def read_at_ref(ref: str, path: str) -> Optional[str]:
    p = subprocess.run(["git", "show", f"{ref}:{path}"], cwd=REPO_ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if p.returncode != 0:
        return None
    return p.stdout


def read_file(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def changed_files(base: str, head: str) -> List[str]:
    out = run(["git", "diff", "--name-only", f"{base}...{head}"])
    return [x.strip() for x in out.splitlines() if x.strip()]


def is_meta_docs_only(files: List[str]) -> bool:
    if not files:
        return True
    for f in files:
        if f in META_FILES:
            continue
        if any(f.startswith(prefix) for prefix in META_PREFIXES):
            continue
        return False
    return True


def extract_section_block(md: str, section_name: str) -> str:
    lines = md.splitlines()
    start = None
    section_re = re.compile(rf"^##\s*\[{re.escape(section_name)}\](?:\s*-\s*\d{{4}}-\d{{2}}-\d{{2}})?\s*$", re.IGNORECASE)
    any_section_re = re.compile(r"^##\s*\[[^\]]+\]")
    for i, line in enumerate(lines):
        if section_re.match(line.strip()):
            start = i + 1
            break
    if start is None:
        return ""
    end = len(lines)
    for j in range(start, len(lines)):
        if any_section_re.match(lines[j].strip()):
            end = j
            break
    return "\n".join(lines[start:end]).strip("\n")


def parse_kac_subsections(block: str) -> Dict[str, List[str]]:
    current = None
    data: Dict[str, List[str]] = {}
    for raw in block.splitlines():
        line = raw.rstrip()
        m = re.match(r"^###\s+(.+?)\s*$", line.strip())
        if m:
            current = m.group(1).strip()
            data.setdefault(current, [])
            continue
        if current and re.match(r"^\s*-\s+.+", line):
            data[current].append(line.strip())
    return data


def added_lines(base: str, head: str, path: str) -> List[str]:
    diff = run(["git", "diff", "--unified=0", f"{base}...{head}", "--", path])
    out: List[str] = []
    for ln in diff.splitlines():
        if ln.startswith("+++") or ln.startswith("@@"):
            continue
        if ln.startswith("+"):
            out.append(ln[1:])
    return out


def is_real_bullet(line: str) -> bool:
    if not re.match(r"^\s*-\s+.+", line):
        return False
    if PLACEHOLDER_RE.search(line):
        return False
    if VAGUE_BULLET_RE.match(line):
        return False
    return True


def validate_added_content(lines: List[str]) -> List[str]:
    errs: List[str] = []
    for ln in lines:
        s = ln.strip()
        if not s:
            continue
        if s.startswith("### "):
            sec = s[4:].strip()
            if sec not in ALLOWED_KAC_SECTIONS:
                errs.append(f"invalid subsection heading introduced: {sec}")
        if PLACEHOLDER_RE.search(s):
            errs.append(f"placeholder text introduced: {s}")
        if VAGUE_BULLET_RE.match(s):
            errs.append(f"vague bullet introduced: {s}")
    return errs


def validate_pr_mode(base: str, head: str, changelog_path: Path) -> int:
    files = changed_files(base, head)
    only_meta = is_meta_docs_only(files)
    changed_changelog = "CHANGELOG.md" in files

    if not only_meta and not changed_changelog:
        print("[changelog] FAIL: non meta/docs-only PR must update CHANGELOG.md")
        return 1

    if not changed_changelog:
        print("[changelog] OK: meta/docs-only PR without changelog update")
        return 0

    current = read_file(changelog_path)
    old = read_at_ref(base, "CHANGELOG.md") or ""

    unreleased_now = extract_section_block(current, "Unreleased")
    if not unreleased_now:
        print("[changelog] FAIL: missing section ## [Unreleased]")
        return 1

    unreleased_old = extract_section_block(old, "Unreleased") if old else ""
    parsed_now = parse_kac_subsections(unreleased_now)
    parsed_old = parse_kac_subsections(unreleased_old)

    bad_sections = [k for k in parsed_now.keys() if k not in ALLOWED_KAC_SECTIONS]
    if bad_sections:
        print("[changelog] FAIL: invalid Keep a Changelog subsections in Unreleased: " + ", ".join(sorted(bad_sections)))
        return 1

    now_bullets: Set[str] = set()
    old_bullets: Set[str] = set()
    for _, bullets in parsed_now.items():
        now_bullets.update(bullets)
    for _, bullets in parsed_old.items():
        old_bullets.update(bullets)

    new_bullets = [b for b in now_bullets if b not in old_bullets and is_real_bullet(b)]
    if not new_bullets:
        print("[changelog] FAIL: changelog changed but no new real bullet in Unreleased")
        return 1

    added = added_lines(base, head, "CHANGELOG.md")
    errs = validate_added_content(added)
    if errs:
        print("[changelog] FAIL:")
        for e in errs:
            print(f"- {e}")
        return 1

    print("[changelog] OK: PR changelog validation passed")
    return 0


def validate_tag_mode(version: str, changelog_path: Path, version_path: Path) -> int:
    v_file = read_file(version_path).strip()
    if v_file != version:
        print(f"[changelog] FAIL: VERSION ({v_file}) != tag version ({version})")
        return 1

    md = read_file(changelog_path)
    rel_block = extract_section_block(md, version)
    if not rel_block:
        print(f"[changelog] FAIL: missing section ## [{version}] - YYYY-MM-DD")
        return 1

    parsed = parse_kac_subsections(rel_block)
    if not parsed:
        print(f"[changelog] FAIL: release section [{version}] has no Keep a Changelog subsections")
        return 1

    bad_sections = [k for k in parsed.keys() if k not in ALLOWED_KAC_SECTIONS]
    if bad_sections:
        print("[changelog] FAIL: invalid subsection(s): " + ", ".join(sorted(bad_sections)))
        return 1

    bullets = [b for _, vals in parsed.items() for b in vals]
    real_bullets = [b for b in bullets if is_real_bullet(b)]
    if not real_bullets:
        print(f"[changelog] FAIL: release section [{version}] has no real bullets")
        return 1

    for ln in rel_block.splitlines():
        if PLACEHOLDER_RE.search(ln) or VAGUE_BULLET_RE.match(ln):
            print(f"[changelog] FAIL: placeholder/vague content in [{version}] section: {ln.strip()}")
            return 1

    print(f"[changelog] OK: tag changelog validation passed for {version}")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(prog="yai-changelog-check")
    mode = ap.add_mutually_exclusive_group(required=True)
    mode.add_argument("--pr", action="store_true", help="validate PR-mode changelog rules")
    mode.add_argument("--tag", action="store_true", help="validate tag-mode release rules")

    ap.add_argument("--base", default="", help="base sha for PR mode")
    ap.add_argument("--head", default="HEAD", help="head sha for PR mode")
    ap.add_argument("--version", default="", help="version X.Y.Z for tag mode")
    ap.add_argument("--file", default="CHANGELOG.md", help="changelog file path")
    ap.add_argument("--version-file", default="VERSION", help="version file path")
    args = ap.parse_args()

    changelog_path = REPO_ROOT / args.file
    version_path = REPO_ROOT / args.version_file

    if args.pr:
        if not args.base:
            print("[changelog] ERROR: --pr requires --base <sha>")
            return 2
        return validate_pr_mode(args.base, args.head, changelog_path)

    if args.tag:
        if not args.version:
            print("[changelog] ERROR: --tag requires --version X.Y.Z")
            return 2
        return validate_tag_mode(args.version, changelog_path, version_path)

    return 2


if __name__ == "__main__":
    raise SystemExit(main())
