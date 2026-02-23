---
id: PRP-001
title: Runtime topology and authority boundaries
status: draft
owner: core-architecture
effective_date: 2026-02-19
revision: 1
supersedes: []
related:
  adr:
    - docs/20-governance/design/adr/ADR-001-single-runtime.md
    - docs/20-governance/design/adr/ADR-002-root-entrypoint.md
    - docs/20-governance/design/adr/ADR-003-kernel-authority.md
    - docs/20-governance/design/adr/ADR-004-engine-execution.md
    - docs/20-governance/design/adr/ADR-005-mind-proposer.md
  runbooks:
    - docs/20-governance/runbooks/root-hardening.md
  milestone_packs:
    - docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md
  specs:
    - deps/yai-specs/contracts/axioms/A-002-authority.md
    - deps/yai-specs/contracts/boundaries/L1-kernel.md
    - deps/yai-specs/contracts/boundaries/L2-engine.md
tags:
  - topology
  - authority
  - runtime
---

# PRP-001 - Runtime topology and authority boundaries

## Problem
Architecture decisions around runtime scope and authority are spread across multiple ADRs. The project needs one pre-decision framing that explains why a machine-level runtime is preferred and how authority boundaries are enforced end-to-end.

## Scope
- In scope: Runtime topology options, authority plane boundaries, workspace-to-runtime routing model.
- Out of scope: Protocol wire-format details and CLI command taxonomy.

## Proposed Change
Define and document the rationale for one machine-level runtime with strict Root -> Kernel -> Engine authority flow and workspace isolation by design.

## Options Compared
- Option A: Single machine-level runtime with multi-workspace routing.
- Option B: Per-workspace daemon model with coordination layer.

## Risks
- Migration complexity from workspace-first habits. Mitigation: phased rollout through runbook phases.
- Confusion on authority ownership. Mitigation: explicit law anchors and boundary diagrams.

## Rollout Sketch
1. Publish topology proposal with explicit L0 anchors.
2. Confirm ADR mapping set (001..005).
3. Gate implementation phases through root-hardening milestones.

## Exit Criteria
- [ ] Proposal clearly maps alternatives and selects a preferred topology.
- [ ] Law anchors for authority boundaries are explicit and complete.
- [ ] Target ADR set is confirmed and linked.

## Traceability

- Spec anchors (if any): `deps/yai-specs/contracts/axioms/A-002-authority.md`, `deps/yai-specs/contracts/boundaries/L1-kernel.md`, `deps/yai-specs/contracts/boundaries/L2-engine.md`
- Targets ADR: `docs/20-governance/design/adr/ADR-001-single-runtime.md`, `docs/20-governance/design/adr/ADR-002-root-entrypoint.md`, `docs/20-governance/design/adr/ADR-003-kernel-authority.md`, `docs/20-governance/design/adr/ADR-004-engine-execution.md`, `docs/20-governance/design/adr/ADR-005-mind-proposer.md`
- Downstream runbook: `docs/20-governance/runbooks/root-hardening.md`
- Downstream MP: `docs/20-governance/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

## References
- `docs/20-governance/design/spine.md`
- `docs/20-governance/design/traceability.md`
- `docs/20-governance/design/adr/ADR-001-single-runtime.md`
