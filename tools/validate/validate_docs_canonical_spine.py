#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

REQUIRED = [
    "docs/architecture/workspace/workspace-architecture.md",
    "docs/architecture/workspace/workspace-boundaries-and-containment-architecture.md",
    "docs/architecture/workspace/workspace-state-and-lifecycle-architecture.md",
    "docs/architecture/workspace/workspace-security-and-scope-architecture.md",
    "docs/architecture/workspace/workspace-peer-and-distribution-architecture.md",
    "docs/architecture/governance/governance-architecture.md",
    "docs/architecture/governance/governance-source-policy-integration-architecture.md",
    "docs/architecture/distributed-runtime/distributed-runtime-architecture.md",
    "docs/program/reports/runtime-convergence-report.md",
    "docs/program/reports/workspace-verticalization-report.md",
    "docs/program/reports/data-plane-convergence-report.md",
    "docs/program/reports/filesystem-governance-convergence-report.md",
    "docs/program/reports/cross-repo-convergence-report.md",
    "docs/guides/developer/developer-operational-walkthroughs-guide.md",
    "docs/guides/developer/developer-workspace-operations-guide.md",
    "docs/runbooks/operations/core-operations-runbook.md",
    "docs/runbooks/qualification/core-qualification-runbook.md",
    "docs/runbooks/demos/demo-execution-runbook.md",
    "docs/runbooks/remediation/core-remediation-runbook.md",
]

FORBIDDEN = [
    "docs/architecture/workspace/workspace-validation-matrix.md",
    "docs/architecture/workspace/workspace-closeout-ws6.md",
    "docs/architecture/governance/source-plane-model-refoundation-rf01.md",
    "docs/guides/developer/workspace-governance-apply-walkthrough.md",
    "docs/runbooks/demos/demo-workspace-runbook.md",
    "docs/runbooks/qualification/qualification-lan-qualification.md",
    "docs/program/reports/workspace-verticalization-gap-classification-v0.1.0.md",
    "docs/program/reports/audit-convergence/UNIFIED-RUNTIME-MANUAL-TEST-COMMAND-PACK-v0.1.0.md",
]


def main() -> int:
    errs: list[str] = []
    for rel in REQUIRED:
        if not (ROOT / rel).is_file():
            errs.append(f"missing required canonical spine doc: {rel}")

    for rel in FORBIDDEN:
        if (ROOT / rel).exists():
            errs.append(f"forbidden overlap/satellite doc still present: {rel}")

    if errs:
        print("docs_canonical_spine: FAIL")
        for e in errs:
            print(" -", e)
        return 1

    print("docs_canonical_spine: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
