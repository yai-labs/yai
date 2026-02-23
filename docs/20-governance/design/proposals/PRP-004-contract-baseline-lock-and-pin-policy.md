---
id: PRP-004
title: Contract baseline lock and cross-repo pin policy
status: draft
owner: release-governance
effective_date: 2026-02-19
revision: 1
supersedes: []
related:
  adr:
    - docs/20-governance/design/adr/ADR-011-contract-baseline-lock.md
  runbooks:
    - docs/20-governance/runbooks/root-hardening.md
  milestone_packs:
    - docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md
  specs:
    - deps/yai-specs/formal/traceability.v1.json
    - deps/yai-specs/formal/spec_map.md
    - deps/yai-specs/contracts/invariants/I-001-traceability.md
    - deps/yai-specs/contracts/invariants/I-007-compliance-context-required.md
tags:
  - baseline
  - pin
  - governance
  - ci
---

# PRP-004 - Contract baseline lock and cross-repo pin policy

## Problem
Cross-repo updates can become inconsistent when pins are not updated in lockstep with contract changes, reducing trust in evidence and delivery claims.

## Scope
- In scope: Baseline definition, pin update rules, CI anti-drift gates, cross-repo coordination points.
- Out of scope: Detailed runtime topology and per-command behavior.

## Proposed Change
Define a formal baseline-lock policy with explicit pin responsibilities for `yai`, `yai-cli`, and `yai-specs`, including required checks before milestone closure.

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

- Spec anchors (if any): `deps/yai-specs/formal/traceability.v1.json`, `deps/yai-specs/formal/spec_map.md`, `deps/yai-specs/contracts/invariants/I-001-traceability.md`, `deps/yai-specs/contracts/invariants/I-007-compliance-context-required.md`
- Targets ADR: `docs/20-governance/design/adr/ADR-011-contract-baseline-lock.md`
- Downstream runbook: `docs/20-governance/runbooks/root-hardening.md`
- Downstream MP: `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

## References
- `docs/20-governance/design/spine.md`
- `docs/20-governance/design/adr/ADR-011-contract-baseline-lock.md`
- `tools/release/sync_specs_refs.sh`
