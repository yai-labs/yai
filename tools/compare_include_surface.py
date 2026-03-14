#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path
from typing import Iterable


def collect_relative_paths(root: Path) -> tuple[set[str], set[str]]:
    dirs: set[str] = set()
    files: set[str] = set()

    for p in root.rglob("*"):
        rel = p.relative_to(root).as_posix()
        if p.is_dir():
            dirs.add(rel)
        elif p.is_file():
            files.add(rel)

    return dirs, files


def write_section(f, title: str, items: Iterable[str]) -> None:
    items = sorted(items)
    f.write(f"{title}\n")
    f.write(f"{'=' * len(title)}\n")
    if not items:
        f.write("(none)\n\n")
        return
    for item in items:
        f.write(f"{item}\n")
    f.write("\n")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Compare Linux include surface against YAI include surface."
    )
    parser.add_argument("--linux-root", required=True)
    parser.add_argument("--yai-root", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()

    linux_root = Path(args.linux_root).resolve()
    yai_root = Path(args.yai_root).resolve()
    output = Path(args.output).resolve()

    if not linux_root.is_dir():
        raise SystemExit(f"Linux include root not found: {linux_root}")
    if not yai_root.is_dir():
        raise SystemExit(f"YAI include root not found: {yai_root}")

    linux_dirs, linux_files = collect_relative_paths(linux_root)
    yai_dirs, yai_files = collect_relative_paths(yai_root)

    linux_only_dirs = linux_dirs - yai_dirs
    yai_only_dirs = yai_dirs - linux_dirs
    linux_only_files = linux_files - yai_files
    yai_only_files = yai_files - linux_files

    output.parent.mkdir(parents=True, exist_ok=True)

    with output.open("w", encoding="utf-8") as f:
        f.write("INCLUDE SURFACE COMPARISON REPORT\n")
        f.write("=================================\n\n")
        f.write(f"Linux root: {linux_root}\n")
        f.write(f"YAI root:   {yai_root}\n\n")

        f.write("COUNTS\n")
        f.write("------\n")
        f.write(f"Linux dirs : {len(linux_dirs)}\n")
        f.write(f"YAI dirs   : {len(yai_dirs)}\n")
        f.write(f"Linux files: {len(linux_files)}\n")
        f.write(f"YAI files  : {len(yai_files)}\n\n")

        write_section(f, "DIRECTORIES ONLY IN LINUX", linux_only_dirs)
        write_section(f, "DIRECTORIES ONLY IN YAI", yai_only_dirs)
        write_section(f, "FILES ONLY IN LINUX", linux_only_files)
        write_section(f, "FILES ONLY IN YAI", yai_only_files)

    print(f"Report written to: {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
