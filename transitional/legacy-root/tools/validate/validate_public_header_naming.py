#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

SCAN_ROOTS = [
    "include",
    "lib",
    "tests",
    "cmd",
    "tools",
]

LEGACY_HEADER_PATHS = [
    "include/yai/edge/action_point.h",
    "include/yai/edge/edge_binding.h",
    "include/yai/edge/edge_services.h",
    "include/yai/edge/edge_state.h",
    "include/yai/edge/local_runtime.h",
    "include/yai/edge/source_ids.h",
    "include/yai/edge/source_plane_model.h",
    "include/yai/orchestration/agent_contract.h",
    "include/yai/orchestration/engine_bridge.h",
    "include/yai/orchestration/engine_cortex.h",
    "include/yai/orchestration/network_gate.h",
    "include/yai/orchestration/provider_gate.h",
    "include/yai/orchestration/resource_gate.h",
    "include/yai/orchestration/storage_gate.h",
    "include/yai/orchestration/source_ingest.h",
    "include/yai/providers/providers.h",
]

LEGACY_INCLUDE_TOKENS = [
    "<yai/edge/action_point.h>",
    "<yai/edge/edge_binding.h>",
    "<yai/edge/edge_services.h>",
    "<yai/edge/edge_state.h>",
    "<yai/edge/local_runtime.h>",
    "<yai/edge/source_ids.h>",
    "<yai/edge/source_plane_model.h>",
    "<yai/orchestration/agent_contract.h>",
    "<yai/orchestration/engine_bridge.h>",
    "<yai/orchestration/engine_cortex.h>",
    "<yai/orchestration/network_gate.h>",
    "<yai/orchestration/provider_gate.h>",
    "<yai/orchestration/resource_gate.h>",
    "<yai/orchestration/storage_gate.h>",
    "<yai/orchestration/source_ingest.h>",
    "<yai/providers/providers.h>",
]


def iter_files(root: Path):
    if root.is_file():
        yield root
        return
    for p in root.rglob("*"):
        if p.is_file():
            yield p


def main() -> int:
    repo = Path(__file__).resolve().parents[2]
    errors: list[str] = []

    for rel in LEGACY_HEADER_PATHS:
        if (repo / rel).exists():
            errors.append(f"legacy public header still present: {rel}")

    for rel in SCAN_ROOTS:
        root = repo / rel
        if not root.exists():
            continue
        for f in iter_files(root):
            fr = f.relative_to(repo).as_posix()
            if fr == "tools/validate/validate_public_header_naming.py":
                continue
            text = f.read_text(encoding="utf-8", errors="ignore")
            for token in LEGACY_INCLUDE_TOKENS:
                if token in text:
                    errors.append(f"{fr}: legacy include token '{token}'")

    if errors:
        print("public_header_naming: FAIL")
        for err in errors:
            print(" -", err)
        return 1

    print("public_header_naming: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
