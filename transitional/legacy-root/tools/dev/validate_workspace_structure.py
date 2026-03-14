#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

HOME = Path(os.environ.get("HOME", ""))
RUN_ROOT = HOME / ".yai" / "run"

REQUIRED_DIRS = ["metadata", "state", "traces", "artifacts", "runtime"]
REQUIRED_FILES = {
    "manifest.json",
    "metadata/binding.json",
    "state/workspace-state.json",
    "traces/index.json",
    "artifacts/index.json",
    "runtime/runtime-state.json",
    "runtime/attach-descriptor.json",
    "runtime/execution-profile.json",
}


def check_ws(ws_dir: Path):
    missing = []
    for d in REQUIRED_DIRS:
        if not (ws_dir / d).is_dir():
            missing.append(d + "/")
    for f in REQUIRED_FILES:
        if not (ws_dir / f).is_file():
            missing.append(f)

    manifest = ws_dir / "manifest.json"
    if manifest.is_file():
        try:
            m = json.loads(manifest.read_text())
            cont = m.get("containment", {})
            if cont.get("ready") is not True:
                missing.append("manifest:containment.ready!=true")
            env = m.get("security_envelope", {})
            if env.get("security_envelope_version") != "v1":
                missing.append("manifest:security_envelope.version!=v1")
            if env.get("security_level_declared") not in ("logical", "scoped", "isolated", "sandboxed"):
                missing.append("manifest:security_level_declared invalid")
            if env.get("security_level_effective") not in ("logical", "scoped", "isolated", "sandboxed"):
                missing.append("manifest:security_level_effective invalid")
            profile = m.get("execution_profile", {})
            if profile.get("execution_mode_requested") not in ("logical", "scoped", "isolated", "sandboxed"):
                missing.append("manifest:execution_mode_requested invalid")
            if profile.get("execution_mode_effective") not in ("logical", "scoped", "isolated", "sandboxed"):
                missing.append("manifest:execution_mode_effective invalid")
            if not profile.get("attach_descriptor_ref"):
                missing.append("manifest:attach_descriptor_ref missing")
            if not profile.get("execution_profile_ref"):
                missing.append("manifest:execution_profile_ref missing")
        except Exception:
            missing.append("manifest:invalid_json")
    else:
        missing.append("manifest.json")

    return missing


def main():
    if not RUN_ROOT.exists():
        print("[workspace-structure] FAIL: run root missing", file=sys.stderr)
        return 1

    ws_ids = sys.argv[1:]
    if not ws_ids:
        ws_ids = [p.name for p in RUN_ROOT.iterdir() if p.is_dir()]

    bad = 0
    checked = 0
    for ws_id in ws_ids:
        ws_dir = RUN_ROOT / ws_id
        if not ws_dir.is_dir():
            continue
        checked += 1
        missing = check_ws(ws_dir)
        if missing:
            bad += 1
            print(f"[workspace-structure] FAIL {ws_id}: " + ", ".join(missing))
        else:
            print(f"[workspace-structure] OK {ws_id}")

    if checked == 0:
        print("[workspace-structure] OK: no workspaces to validate")
        return 0

    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main())
