from __future__ import annotations

import subprocess
from typing import List


def _run_git(args: List[str]) -> str:
    p = subprocess.run(["git", *args], check=True, capture_output=True, text=True)
    return p.stdout.strip()


def head_sha() -> str:
    return _run_git(["rev-parse", "HEAD"])


def checkout_new_branch(name: str) -> None:
    subprocess.run(["git", "checkout", "-b", name], check=True)
