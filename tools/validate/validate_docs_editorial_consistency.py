#!/usr/bin/env python3
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

TARGETS = [
    "docs/architecture/README.md",
    "docs/architecture/overview/repository-scope.md",
    "docs/architecture/runtime/runtime-architecture.md",
    "docs/architecture/workspace/workspace-architecture.md",
    "docs/architecture/governance/governance-architecture.md",
    "docs/architecture/distributed-runtime/distributed-runtime-architecture.md",
    "docs/architecture/protocol/transport/secure-overlay-architecture.md",
    "docs/architecture/data-runtime/canonical-data-plane-architecture.md",
    "docs/architecture/intelligence-runtime/daemon-local-runtime-architecture.md",
    "docs/architecture/system-theory/README.md",
    "docs/guides/README.md",
    "docs/guides/developer/README.md",
    "docs/guides/developer/workflow/README.md",
    "docs/guides/developer/debugging/debugging.md",
    "docs/runbooks/README.md",
    "docs/runbooks/operations/core-operations-runbook.md",
    "docs/runbooks/qualification/core-qualification-runbook.md",
    "docs/runbooks/demos/demo-execution-runbook.md",
    "docs/runbooks/remediation/core-remediation-runbook.md",
    "docs/reference/README.md",
    "docs/reference/protocol/README.md",
    "docs/reference/schemas/README.md",
    "docs/reference/commands/README.md",
    "docs/reference/cli/README.md",
    "docs/reference/sdk/README.md",
    "docs/program/README.md",
    "docs/program/rfc/README.md",
    "docs/program/adr/README.md",
    "docs/program/reports/README.md",
    "docs/program/rfc/rfc-runtime-001-runtime-topology-and-authority.md",
    "docs/program/rfc/rfc-protocol-002-unified-rpc-and-cli-contract.md",
    "docs/program/rfc/rfc-workspace-003-workspace-lifecycle-and-isolation.md",
    "docs/program/rfc/rfc-contracts-004-contract-runbook-lock-and-pin-policy.md",
    "docs/program/rfc/rfc-formal-005-formal-coverage-roadmap.md",
    "docs/program/adr/adr-runtime-001-single-runtime.md",
    "docs/program/adr/adr-protocol-006-unified-rpc.md",
    "docs/program/adr/adr-runtime-003-kernel-authority.md",
    "docs/program/adr/adr-program-012-audit-convergence-gates.md",
]

REQUIRED_FM = ["role:", "status:", "audience:", "owner_domain:"]
REQUIRED_SECTIONS = [
    "# Purpose",
    "# Scope",
    "# Relationships",
    "# Canonical Role",
    "# Main Body",
    "# Related Docs",
]

# Strict tone normalization is enforced on editorial framing blocks only.
FORBIDDEN_TONE = re.compile(
    r"\b(refactor|hardening|closeout|historical commentary|tranche)\b",
    flags=re.IGNORECASE,
)


def main() -> int:
    errors: list[str] = []
    for rel in TARGETS:
        p = ROOT / rel
        if not p.is_file():
            errors.append(f"missing editorial target: {rel}")
            continue

        text = p.read_text(encoding="utf-8", errors="ignore")
        if not text.startswith("---\n"):
            errors.append(f"missing front matter: {rel}")
            continue

        fm_end = text.find("\n---\n", 4)
        if fm_end < 0:
            errors.append(f"invalid front matter block: {rel}")
            continue

        fm = text[: fm_end + 5]
        for key in REQUIRED_FM:
            if key not in fm:
                errors.append(f"missing front matter key '{key[:-1]}' in {rel}")

        for sec in REQUIRED_SECTIONS:
            if sec not in text:
                errors.append(f"missing section '{sec}' in {rel}")

        # Tone normalization is evaluated on editorial body sections only,
        # excluding front matter metadata where legacy identifiers may remain.
        editorial_body = text[fm_end + 5 :]
        pre_main = editorial_body.split("# Main Body", 1)[0]
        if FORBIDDEN_TONE.search(pre_main):
            errors.append(f"forbidden editorial tone term in framing sections: {rel}")

    if errors:
        print("docs_editorial_consistency: FAIL")
        for e in errors:
            print(" -", e)
        return 1

    print("docs_editorial_consistency: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
