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
- `yai-dev-issue`: issue body generator (manual issue creation remains maintainer-owned).
- `yai-dev-branch`: alias of `yai-branch`.
- `yai-dev-pr-body`: alias of `yai-pr-body`.
- `yai-dev-pr-check`: alias of `yai-pr-check`.

## Quick Start

- `tools/bin/yai-dev-branch --type feat --issue 123 --area root --desc hardening-forward`
- `tools/bin/yai-dev-pr-body --template default --issue 123 --mp-id MP-ROOT-HARDENING-0.1.0 --runbook docs/runbooks/root-hardening.md#phase-0-1-0-protocol-guardrails --out .pr/PR_BODY.md`
- `tools/bin/yai-dev-pr-check .pr/PR_BODY.md`
