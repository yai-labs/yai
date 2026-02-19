from __future__ import annotations

import argparse
import subprocess
from pathlib import Path

from yai_tools.verify.agent_pack import run_agent_pack
from yai_tools.verify.frontmatter_schema import run_schema_check
from yai_tools.verify.trace_graph import run_graph

REPO_ROOT = Path(__file__).resolve().parents[4]


def _run_traceability(mode: str, base: str, head: str) -> int:
    if mode == "ci":
        cmd = ["tools/bin/yai-docs-trace-check", "--changed", "--base", base, "--head", head]
    else:
        cmd = ["tools/bin/yai-docs-trace-check", "--all"]
    p = subprocess.run(cmd, cwd=REPO_ROOT)
    return p.returncode


def run_doctor(mode: str, base: str, head: str) -> int:
    if mode == "ci" and not base:
        print("[docs-doctor] ERROR: --mode ci requires --base")
        return 2

    # 1) existing traceability gate
    rc = _run_traceability(mode=mode, base=base, head=head)
    if rc != 0:
        return rc

    # 2) schema checks
    rc = run_schema_check(changed=(mode == "ci"), base=base if mode == "ci" else "", head=head if mode == "ci" else "HEAD")
    if rc != 0:
        return rc

    # 3) generated graph sync
    rc = run_graph(write=False)
    if rc != 0:
        return rc

    # 4) agent pack sync
    rc = run_agent_pack(write=False)
    if rc != 0:
        return rc

    print("[docs-doctor] OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(prog="yai-docs-doctor")
    ap.add_argument("--mode", choices=["ci", "all"], default="ci")
    ap.add_argument("--base", default="")
    ap.add_argument("--head", default="HEAD")
    args = ap.parse_args()
    return run_doctor(args.mode, args.base, args.head)


if __name__ == "__main__":
    raise SystemExit(main())
