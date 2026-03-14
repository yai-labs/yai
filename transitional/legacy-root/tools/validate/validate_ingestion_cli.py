#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CLI = ROOT / "tools" / "bin" / "yai-govern"
CID = "enterprise.sample.src-sample-digital-outbound.candidate.v1"
REVIEW_FILE = ROOT / "governance" / "ingestion" / "review" / f"{CID}.review.v1.json"


def run(*args: str) -> int:
    cmd = [str(CLI), *args]
    return subprocess.call(cmd, cwd=str(ROOT))


def main() -> int:
    if not CLI.exists():
        print(f"[ingestion-cli] FAIL: missing {CLI.relative_to(ROOT)}")
        return 1

    # Keep validation deterministic even if previous runs advanced review lifecycle.
    if REVIEW_FILE.exists():
        REVIEW_FILE.unlink()

    checks = [
        ("source", "list"),
        ("source", "inspect", "src.sample.digital-outbound"),
        ("parse", "src.sample.digital-outbound"),
        ("parsed", "inspect", "src.sample.digital-outbound"),
        ("normalize", "src.sample.digital-outbound"),
        ("normalized", "inspect", "norm.src-sample-digital-outbound"),
        ("build", "norm.src-sample-digital-outbound"),
        ("candidate", "inspect", CID),
        ("validate", CID),
        ("status", CID),
    ]

    for c in checks:
        rc = run(*c)
        if rc != 0:
            print(f"[ingestion-cli] FAIL: {' '.join(c)} rc={rc}")
            return 1

    print("[ingestion-cli] OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
