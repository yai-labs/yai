---
id: DEV-GUIDE-README
status: active
effective_date: 2026-02-19
revision: 1
owner: governance
---

# Dev Guide

Canonical developer and maintainer guidance for repository workflow, tooling, and release operations.

## Start here

1. `docs/dev-guide/agent-playbook.md`
2. `docs/dev-guide/agent-contract.md`
3. `docs/dev-guide/repo-tooling.md`
4. `docs/dev-guide/github-templates.md`
5. `docs/dev-guide/cross-repo-workflow.md`

## Core guides

- `docs/dev-guide/repo-workflow.md`: end-to-end git/release runbook.
- `docs/dev-guide/release.md`: release mechanics and tag flow.
- `docs/dev-guide/testing.md`: testing execution guidance.
- `docs/dev-guide/debugging.md`: troubleshooting and diagnostics.
- `docs/dev-guide/tooling-layout.md`: canonical tooling tree.
- `docs/dev-guide/toolchain-contract-v1.md`: non-negotiable workflow contract.

## Agent governance bundle

- `docs/dev-guide/agent-playbook.md`
- `docs/dev-guide/agent-contract.md`
- `docs/dev-guide/checklists/proposal-checklist.md`
- `docs/dev-guide/checklists/adr-checklist.md`
- `docs/dev-guide/checklists/runbook-checklist.md`
- `docs/dev-guide/checklists/milestone-pack-checklist.md`
- `docs/dev-guide/checklists/proof-pack-checklist.md`

## CI-aligned local commands

- `tools/bin/yai-docs-trace-check --changed --base <BASE_SHA> --head <HEAD_SHA>`
- `tools/bin/yai-docs-schema-check --changed --base <BASE_SHA> --head <HEAD_SHA>`
- `tools/bin/yai-docs-graph --check`
- `tools/bin/yai-agent-pack --check`
- `tools/bin/yai-docs-doctor --mode ci --base <BASE_SHA> --head <HEAD_SHA>`

