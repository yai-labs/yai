---
id: ADR-001
status: accepted
effective_date: 2026-02-18
supersedes: []
applies_to:
  runbook: docs/program/milestone-packs/runtime-baselines/root-hardening.md
  phase: 0.1.0
  anchor: "#phase-0-1-0-protocol-guardrails"
law_refs:
  - ../law/foundation/axioms/A-002-authority.md
  - ../law/foundation/invariants/I-003-governance.md
  - ../law/foundation/invariants/I-006-external-effect-boundary.md
  - ../law/foundation/boundaries/L1-kernel.md
  - ../law/foundation/boundaries/L2-engine.md
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

Distributed source acquisition is allowed only as an edge feed into this owner
runtime model (see `ADR-013`). It does not introduce additional owner runtimes.

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
  - `docs/program/rfc/RFC-001-runtime-topology-and-authority.md`
- Implemented by runbooks:
  - `docs/program/milestone-packs/runtime-baselines/root-hardening.md`
- Milestone packs:
  - `docs/program/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

## Status

Accepted and active.
