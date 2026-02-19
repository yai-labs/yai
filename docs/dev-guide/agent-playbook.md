---
id: AGENT-PLAYBOOK
status: active
effective_date: 2026-02-19
revision: 1
owner: governance
---

# Agent Playbook

This is the canonical execution guide for code agents working in this repository.

## Objective

Produce repo changes by following existing standards, templates, and traceability rules.
Do not invent structure from scratch.

## Canonical sources (read first)

1. `docs/design/spine.md`
2. `docs/dev-guide/cross-repo-workflow.md`
3. `docs/templates/README.md`
4. `docs/_policy/docs-style.md`
5. `docs/design/traceability.md`
6. `docs/dev-guide/github-templates.md`
7. `docs/dev-guide/agent-contract.md`
8. `tools/README.md`

## Required artifact flow

Use this sequence unless explicitly overridden:

1. Proposal (`docs/design/proposals/PRP-...`)
2. ADR (`docs/design/adr/ADR-...`)
3. Runbook phase (`docs/runbooks/...`)
4. Milestone Pack (`docs/milestone-packs/.../MP-...`)
5. Evidence/Test references (`docs/test-plans/...` + CI/artifacts)
6. Proof pack when needed (`docs/proof/...`)

## Authoring rules

- Start from canonical templates in `docs/templates/`.
- Keep links repo-relative.
- Keep L0 anchors under `deps/yai-specs/...`.
- Keep upward/downward links explicit:
  - Proposal -> ADR
  - ADR -> Runbook
  - Runbook -> MP
  - MP -> Evidence
- Do not copy specs into docs; link to `deps/yai-specs`.

## PR rules (mandatory)

- Use PR templates and generator:
  - `tools/bin/yai-pr-body --template <...> ...`
- Validate PR metadata when available.
- Include issue context (`#NNN` or `N/A` + reason when allowed).

## Automation commands

- `tools/bin/yai-docs-schema-check --changed --base <BASE_SHA> --head <HEAD_SHA>`
- `tools/bin/yai-docs-graph --check`
- `tools/bin/yai-agent-pack --check`
- `tools/bin/yai-docs-doctor --mode ci --base <BASE_SHA> --head <HEAD_SHA>`

## Definition of done for docs-governance changes

A docs change is done only when:

1. Templates/policy rules are respected.
2. Traceability links are complete and valid.
3. Required checks pass in CI (`validate-traceability`, `validate-runbook-adr-links`, PR metadata checks).
4. Changes are committed on dedicated branch and pushed with auditable PR body.

## Audit and proof timing

- Draft audits stay local/private until milestone evidence is closed.
- Canonical public audits are published after milestone closure and human review.
