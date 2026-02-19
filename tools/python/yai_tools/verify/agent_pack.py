from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from yai_tools.verify.generated_sync import check_json_synced, write_json

REPO_ROOT = Path(__file__).resolve().parents[4]
OUT = REPO_ROOT / "docs" / "_generated" / "agent-pack.v1.json"


def build_pack() -> dict[str, Any]:
    return {
        "version": 1,
        "canonical_sources": [
            "docs/design/spine.md",
            "docs/dev-guide/cross-repo-workflow.md",
            "docs/templates/README.md",
            "docs/_policy/docs-style.md",
            "docs/design/traceability.md",
            "docs/dev-guide/github-templates.md",
            "tools/README.md",
            "docs/dev-guide/agent-contract.md",
            "docs/dev-guide/agent-playbook.md"
        ],
        "artifact_flow": ["proposal", "adr", "runbook", "milestone_pack", "evidence", "proof_pack"],
        "required_commands": [
            "tools/bin/yai-docs-trace-check --changed --base <BASE_SHA> --head <HEAD_SHA>",
            "tools/bin/yai-docs-schema-check --changed --base <BASE_SHA> --head <HEAD_SHA>",
            "tools/bin/yai-docs-graph --check",
            "tools/bin/yai-agent-pack --check",
            "tools/bin/yai-docs-doctor --mode ci --base <BASE_SHA> --head <HEAD_SHA>",
            "tools/bin/yai-pr-body --template <template> ..."
        ],
        "quality_gates": [
            "validate-pr-metadata",
            "validate-traceability",
            "validate-runbook-adr-links",
            "validate-agent-pack",
            "validate-changelog"
        ],
        "path_policy": {
            "relative_only": True,
            "allowed_prefixes": ["docs/", "deps/", "tools/", ".github/"],
            "forbid_absolute_paths": True
        },
        "audit_policy": {
            "draft_private_path": "docs/audits/.private/",
            "public_path": "docs/audits/",
            "promotion_requires": ["milestone_closed", "evidence_complete", "human_review"]
        }
    }


def run_agent_pack(write: bool) -> int:
    obj = build_pack()
    if write:
        write_json(OUT, obj)
        print("[agent-pack] OK: generated pack updated")
        return 0

    ok, msg = check_json_synced(OUT, obj)
    if not ok:
        print(f"[agent-pack] FAIL: {msg}")
        return 1

    print("[agent-pack] OK")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(prog="yai-agent-pack")
    mode = ap.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    args = ap.parse_args()
    return run_agent_pack(write=args.write)


if __name__ == "__main__":
    raise SystemExit(main())
