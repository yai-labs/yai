from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def dumps_canonical(obj: Any) -> str:
    return json.dumps(obj, indent=2, sort_keys=True) + "\n"


def write_json(path: Path, obj: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(dumps_canonical(obj), encoding="utf-8")


def check_json_synced(path: Path, obj: Any) -> tuple[bool, str]:
    expected = dumps_canonical(obj)
    if not path.exists():
        return False, f"missing generated file: {path.as_posix()}"
    current = path.read_text(encoding="utf-8")
    if current != expected:
        return False, f"stale generated file: {path.as_posix()} (run --write to refresh)"
    return True, "ok"
