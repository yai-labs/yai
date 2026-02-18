from __future__ import annotations

from pathlib import Path


def repo_root() -> Path:
    # tools/python/yai_tools/_core/paths.py -> repo root = ../../../..
    here = Path(__file__).resolve()
    return here.parents[4]
