#!/usr/bin/env python3
import json
from pathlib import Path


def main() -> int:
    root = Path(__file__).resolve().parents[3]
    embed = root / "embedded" / "law"

    publish_idx = json.loads((embed / "manifests" / "publish.index.json").read_text(encoding="utf-8"))
    layers = json.loads((embed / "manifests" / "publish.layers.json").read_text(encoding="utf-8"))

    runtime = None
    for t in publish_idx.get("targets", []):
        if t.get("target") == "runtime-embedded":
            runtime = t
            break
    if runtime is None:
        raise SystemExit("embedded_surface_contract: missing runtime-embedded target")

    for rel in runtime.get("includes", []):
        if not (embed / rel).exists():
            raise SystemExit(f"embedded_surface_contract: missing runtime include: {rel}")

    runtime_surface = layers.get("runtime_surface", {})
    required = [
        runtime_surface.get("classification"),
        runtime_surface.get("control_families"),
        runtime_surface.get("domain_specializations"),
        runtime_surface.get("manifests"),
        runtime_surface.get("overlays", {}).get("regulatory"),
        runtime_surface.get("overlays", {}).get("sector"),
        runtime_surface.get("overlays", {}).get("contextual"),
    ]
    for rel in required:
        if not isinstance(rel, str) or not rel:
            raise SystemExit("embedded_surface_contract: invalid runtime_surface map")
        if not (embed / rel).exists():
            raise SystemExit(f"embedded_surface_contract: missing runtime surface path: {rel}")

    print("embedded_surface_contract: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
