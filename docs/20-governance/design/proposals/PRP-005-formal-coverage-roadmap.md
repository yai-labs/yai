---
id: PRP-005
title: Formal coverage roadmap for spec-critical domains
status: draft
owner: formal-methods
effective_date: 2026-02-19
revision: 1
supersedes: []
related:
  adr:
    - docs/20-governance/design/adr/ADR-006-unified-rpc.md
    - docs/20-governance/design/adr/ADR-011-contract-baseline-lock.md
  runbooks:
    - docs/20-governance/runbooks/root-hardening.md
  milestone_packs:
    - docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md
  specs:
    - deps/yai-specs/formal/spec_map.md
    - deps/yai-specs/formal/tla/YAI_KERNEL.tla
    - deps/yai-specs/formal/bindings/BINDING_PROTOCOL.md
    - deps/yai-specs/formal/bindings/BINDING_CLI.md
tags:
  - formal
  - roadmap
  - coverage
---

# PRP-005 - Formal coverage roadmap for spec-critical domains

## Problem
Current formal coverage is uneven across domains. Some areas are modeled while others remain smoke/none, which limits confidence for stronger TRL claims.

## Scope
- In scope: Coverage priorities, domain gap list, staged property roadmap, evidence expectations.
- Out of scope: Rewriting the whole TLA model in one phase.

## Proposed Change
Define a prioritized formal roadmap that starts from protocol/control criticality, then addresses CLI, vault, and graph gaps with explicit property targets.

## Options Compared
- Option A: Risk-first roadmap based on invariant and boundary criticality.
- Option B: Surface-first roadmap by component ownership.

## Risks
- Over-expansion of formal scope. Mitigation: milestone-based slices with hard acceptance criteria.
- Weak adoption if disconnected from delivery gates. Mitigation: link roadmap outputs to CI and milestone packs.

## Rollout Sketch
1. Publish current modeled/smoke/none matrix.
2. Select next two domains for explicit property additions.
3. Bind roadmap outputs to release evidence expectations.

## Exit Criteria
- [ ] Coverage matrix and priority order are explicit.
- [ ] Next formal increments define target properties and artifacts.
- [ ] Proposal links to ADR and milestone evidence strategy.

## Traceability

- Spec anchors (if any): `deps/yai-specs/formal/spec_map.md`, `deps/yai-specs/formal/tla/YAI_KERNEL.tla`, `deps/yai-specs/formal/bindings/BINDING_PROTOCOL.md`, `deps/yai-specs/formal/bindings/BINDING_CLI.md`
- Targets ADR: `docs/20-governance/design/adr/ADR-006-unified-rpc.md`, `docs/20-governance/design/adr/ADR-011-contract-baseline-lock.md`
- Downstream runbook: `docs/20-governance/runbooks/root-hardening.md`
- Downstream MP: `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md`

## References
- `docs/20-governance/design/spine.md`
- `deps/yai-specs/formal/spec_map.md`
- `deps/yai-specs/formal/traceability.v1.json`
