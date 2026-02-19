---
id: ADR-001
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/runbooks/root-hardening.md
  phase: 0.1.0
  anchor: "#phase-0-1-0-protocol-guardrails"
law_refs:
  - deps/yai-specs/contracts/axioms/A-002-authority.md
  - deps/yai-specs/contracts/invariants/I-003-governance.md
  - deps/yai-specs/contracts/invariants/I-006-external-effect-boundary.md
  - deps/yai-specs/contracts/boundaries/L1-kernel.md
  - deps/yai-specs/contracts/boundaries/L2-engine.md
---
# ADR-001 - Single Runtime Per Machine

## Context

YAI is evolving from a mixed execution model toward a machine-level runtime. The previous shape allowed multiple implicit entry paths and per-workspace execution assumptions that made governance and evidence weaker.

## Decision

YAI adopts one canonical machine runtime composed of:

- Root control plane
- Kernel (L1 authority plane)
- Engine (L2 execution plane)

Workspaces are logical tenants managed by this runtime, not independent daemon stacks.

## Rationale

A single runtime reduces authority ambiguity, improves observability, and strengthens deterministic enforcement of workspace boundaries.

## Consequences

- Positive:
  - One authoritative ingress and lifecycle model.
  - Better cross-workspace governance and auditable routing.
- Negative:
  - Legacy assumptions around direct workspace access must be removed.
  - Migration work is required in boot/routing and operational docs.

## Traceability

- Proposals:
  - `docs/design/proposals/PRP-001-runtime-topology-and-authority.md`
- Implemented by runbooks:
  - `docs/runbooks/root-hardening.md`
- Milestone packs:
  - `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

## Status

Accepted and active.
