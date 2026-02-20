# tools/bin

Canonical entrypoints for operational tooling.

## Purpose

Provide stable, easy-to-remember commands for verify, gate, suite, diagnostics, and PR/issue workflow helpers.

## Commands

- `yai-verify`: runs checks from `tools/ops/verify/`.
- `yai-gate`: runs gates from `tools/ops/gate/`.
- `yai-suite`: runs suites from `tools/ops/suite/`.
- `yai-doctor`: local environment diagnostics.
- `yai-purge`: local cleanup.
- `yai-branch`: canonical branch-name generator.
- `yai-pr-body`: PR body generator from templates.
- `yai-pr-check`: strict PR body metadata validator.
- `yai-dev-issue`: phase issue + MP closure creator (with legacy issue-body mode).
- `yai-dev-milestone-body`: canonical PHASE milestone body generator.
- `yai-dev-fix-phase`: dry-run/apply fixer for phase naming/labels/milestone alignment.
- `yai-dev-label-sync`: dry-run/apply sync for label color palette (phase/track/class/type/area/work-type + core governance labels).
- `yai-dev-branch`: alias of `yai-branch`.
- `yai-dev-branch-sync`: create/check out the same branch across `yai`, `yai-cli`, `yai-mind`.
- `yai-dev-pr-body`: alias of `yai-pr-body`.
- `yai-dev-pr-check`: alias of `yai-pr-check`.
- `yai-specs-sync`: sync `deps/yai-specs` pin and proof-pack refs (`manifest` + `README`).
- `yai-docs-schema-check`: validate docs frontmatter schema.
- `yai-docs-graph`: generate/check docs traceability graph + lock.
- `yai-agent-pack`: generate/check canonical machine-readable agent pack.
- `yai-docs-doctor`: run end-to-end docs-governance checks for CI/local.
- `yai-architecture-check`: hard-fail architecture alignment checker (`--changed`, `--all`, `--write`).

## Quick Start

- `tools/bin/yai-dev-branch --type feat --issue 123 --area root --desc hardening-forward`
- `tools/bin/yai-dev-pr-body --template default --issue 123 --mp-id MP-ROOT-HARDENING-0.1.0 --runbook docs/runbooks/root-hardening.md#phase-0-1-0-protocol-guardrails --out .pr/PR_BODY.md`
- `tools/bin/yai-dev-pr-check .pr/PR_BODY.md`
- `tools/bin/yai-dev-milestone-body --track contract-baseline-lock --phase 0.1.0 --rb-anchor docs/runbooks/contract-baseline-lock.md#0.1.0 --mp-id MP-CONTRACT-BASELINE-LOCK-0.1.0`
- `tools/bin/yai-dev-issue phase --track contract-baseline-lock --phase 0.1.0 --rb-id RB-CONTRACT-BASELINE-LOCK --title \"Pin Baseline Freeze\" --rb-anchor docs/runbooks/contract-baseline-lock.md#0.1.0 --mp-id MP-CONTRACT-BASELINE-LOCK-0.1.0`
- `tools/bin/yai-dev-issue mp-closure --track contract-baseline-lock --phase 0.1.0 --mp-id MP-CONTRACT-BASELINE-LOCK-0.1.0`
- `tools/bin/yai-dev-fix-phase --track contract-baseline-lock --phase 0.1.0 --repo yai-labs/yai`
- `tools/bin/yai-dev-label-sync --repo yai-labs/yai`

- `tools/bin/yai-dev-branch-sync --type chore --issue N/A --reason bootstrap --area governance --desc proof-pack-lock`

- `tools/bin/yai-specs-sync --target origin/main`
