#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

REQUIRED_DIRS = [
    "docs/architecture/runtime/core",
    "docs/architecture/runtime/resolution",
    "docs/architecture/runtime/enforcement",
    "docs/architecture/runtime/data-sinks",
    "docs/architecture/protocol/control-plane",
    "docs/architecture/protocol/transport",
    "docs/architecture/protocol/source-plane",
    "docs/architecture/distributed-runtime/topology",
    "docs/architecture/distributed-runtime/peering",
    "docs/architecture/distributed-runtime/policy",
    "docs/guides/developer/operational-guides",
    "docs/program/milestone-packs/runtime-baselines/workspace",
    "docs/program/milestone-packs/runtime-baselines/distributed-runtime",
    "docs/program/milestone-packs/runtime-baselines/data-runtime",
    "docs/program/milestone-packs/runtime-baselines/governance-runtime",
    "docs/program/milestone-packs/runtime-baselines/operations-foundation",
    "docs/reference/protocol/cli",
    "docs/reference/protocol/control",
    "docs/reference/protocol/protocol",
    "docs/reference/protocol/providers",
    "docs/reference/protocol/vault",
    "docs/reference/protocol/compliance",
]

REQUIRED_READMES = [
    "docs/architecture/runtime/README.md",
    "docs/architecture/runtime/core/README.md",
    "docs/architecture/runtime/resolution/README.md",
    "docs/architecture/runtime/enforcement/README.md",
    "docs/architecture/runtime/data-sinks/README.md",
    "docs/architecture/protocol/README.md",
    "docs/architecture/protocol/control-plane/README.md",
    "docs/architecture/protocol/transport/README.md",
    "docs/architecture/protocol/source-plane/README.md",
    "docs/architecture/distributed-runtime/README.md",
    "docs/architecture/distributed-runtime/topology/README.md",
    "docs/architecture/distributed-runtime/peering/README.md",
    "docs/architecture/distributed-runtime/policy/README.md",
    "docs/guides/developer/operational-guides/README.md",
    "docs/archive/legacy/program/milestone-packs/runtime-baselines/workspace/README.md",
    "docs/archive/legacy/program/milestone-packs/runtime-baselines/distributed-runtime/README.md",
    "docs/archive/legacy/program/milestone-packs/runtime-baselines/data-runtime/README.md",
    "docs/archive/legacy/program/milestone-packs/runtime-baselines/governance-runtime/README.md",
    "docs/archive/legacy/program/milestone-packs/runtime-baselines/operations-foundation/README.md",
]

FORBIDDEN_DIRS = [
    "docs/reference/protocol/contracts/cli",
    "docs/reference/protocol/contracts/control",
    "docs/reference/protocol/contracts/protocol",
    "docs/reference/protocol/contracts/providers",
    "docs/reference/protocol/contracts/vault",
    "docs/reference/protocol/contracts/compliance",
    "docs/runbooks/operations/developer-runbooks",
]

ROOT_MD_ALLOWLIST = {
    "docs/architecture/runtime": {"README.md", "runtime-architecture.md"},
    "docs/architecture/protocol": {"README.md"},
    "docs/architecture/distributed-runtime": {"README.md", "distributed-runtime-architecture.md"},
    "docs/guides/developer": {"README.md"},
    "docs/program/milestone-packs/runtime-baselines": {"README.md"},
}


def main() -> int:
    errors: list[str] = []

    for rel in REQUIRED_DIRS:
        if not (ROOT / rel).is_dir():
            errors.append(f"missing required verticalized dir: {rel}")

    for rel in REQUIRED_READMES:
        path = ROOT / rel
        if not path.is_file():
            errors.append(f"missing required section README: {rel}")
        else:
            text = path.read_text(encoding="utf-8").strip()
            if len(text.splitlines()) < 3:
                errors.append(f"section README too weak: {rel}")

    for rel in FORBIDDEN_DIRS:
        if (ROOT / rel).exists():
            errors.append(f"forbidden pre-verticalization dir still present: {rel}")

    for rel, allowed in ROOT_MD_ALLOWLIST.items():
        section = ROOT / rel
        if not section.is_dir():
            continue
        names = {p.name for p in section.glob("*.md")}
        extra = sorted(names - allowed)
        if extra:
            errors.append(
                f"section root has non-canonical files in {rel}: {', '.join(extra)}"
            )

    if errors:
        print("docs_section_verticalization: FAIL")
        for e in errors:
            print(" -", e)
        return 1

    print("docs_section_verticalization: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
