---
id: AGENT-CONTRACT
status: active
effective_date: 2026-02-19
revision: 1
owner: governance
---

# Agent Contract

This document is normative for code agents.

## Mandatory rules

- MUST follow canonical source order from `docs/dev-guide/agent-playbook.md`.
- MUST create docs artifacts from templates in `docs/templates/`.
- MUST keep repository links relative.
- MUST reference normative anchors in `deps/yai-specs/`.
- MUST preserve the chain: `proposal -> ADR -> runbook -> MP -> evidence`.
- MUST generate PR body through `tools/bin/yai-pr-body`.
- MUST pass docs gates before proposing merge.

## Prohibited

- MUST NOT invent alternative folder taxonomies.
- MUST NOT duplicate contract/spec text from `deps/yai-specs` into docs.
- MUST NOT use absolute filesystem paths inside authored docs.
- MUST NOT mark audits canonical without milestone closure and evidence.

## Required checks before merge

- `tools/bin/yai-docs-trace-check --changed --base <BASE_SHA> --head <HEAD_SHA>`
- `tools/bin/yai-docs-schema-check --changed --base <BASE_SHA> --head <HEAD_SHA>`
- `tools/bin/yai-docs-graph --check`
- `tools/bin/yai-agent-pack --check`
- `tools/bin/yai-docs-doctor --mode ci --base <BASE_SHA> --head <HEAD_SHA>`
