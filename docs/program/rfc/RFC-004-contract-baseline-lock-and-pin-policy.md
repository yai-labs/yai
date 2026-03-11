---
id: RFC-004
title: Contract baseline lock and cross-repo pin policy
status: draft
owners:
  - "@francescomaiomascio"
legacy_owner: release-governance
links:
  replaces: PRP-004

effective_date: 2026-02-19
revision: 1
supersedes: []
related:
  adr:
    - docs/program/adr/ADR-011-contract-baseline-lock.md
  runbooks:
    - docs/program/milestone-packs/runtime-baselines/root-hardening.md
  milestone_packs:
    - docs/program/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md
  specs:
    - ../law/formal/traceability.v1.json
    - ../law/formal/spec_map.md
    - ../law/foundation/invariants/I-001-traceability.md
    - ../law/foundation/invariants/I-007-compliance-context-required.md
tags:
  - baseline
  - pin
  - governance
  - ci
---

# RFC-004 - Contract baseline lock and cross-repo pin policy

## Problem
Cross-repo updates can become inconsistent when pins are not updated in lockstep with contract changes, reducing trust in evidence and delivery claims.

## Scope
- In scope: Baseline definition, pin update rules, CI anti-drift gates, cross-repo coordination points.
- Out of scope: Detailed runtime topology and per-command behavior.

## Proposed Change
Define a formal baseline-lock policy with explicit pin responsibilities for `yai`, `cli`, and `law`, including required checks before milestone closure.

## Options Compared
- Option A: Strict lockstep pin policy with mandatory checks.
- Option B: Soft pin policy with best-effort sync windows.

## Risks
- Coordination cost across repos. Mitigation: scripted sync + clear owner model.
- Slower merge velocity for contract-touching changes. Mitigation: narrow baseline scopes per milestone.

## Rollout Sketch
1. Define baseline contract manifest.
2. Define owner/responsibility matrix for pin updates.
3. Enforce checks before merge on contract-touching PRs.

## Exit Criteria
- [ ] Pin policy is explicit for each consumer repo.
- [ ] Mandatory pre-merge checks are documented.
- [ ] ADR-011 acceptance criteria can reference this proposal directly.

## Traceability

- Spec anchors (if any): `../law/formal/traceability.v1.json`, `../law/formal/spec_map.md`, `../law/foundation/invariants/I-001-traceability.md`, `../law/foundation/invariants/I-007-compliance-context-required.md`
- Targets ADR: `docs/program/adr/ADR-011-contract-baseline-lock.md`
- Downstream runbook: `docs/program/milestone-packs/runtime-baselines/root-hardening.md`
- Downstream MP: `docs/program/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

## References
- `docs/program/spine.md`
- `docs/program/adr/ADR-011-contract-baseline-lock.md`
- `../infra/tools/bin/law-sync`
