---
role: support
status: accepted
audience: governance
owner_domain: program-adr
id: ADR-015
decision_id: ADR-015
supersedes: []
superseded_by: []
implements: []
evidenced_by: [docs/program/reports/runtime-convergence-report.md]
related: []
anchor: "#daemon-architecture-refoundation-slice"
applies_to: 
effective_date: 2026-03-11
phase: YD-1
---
# ADR-015 - Daemon Architecture Refoundation Slice

## Context

After introducing distributed source-plane work, implementation slices were at
risk of drifting into ambiguous owner/daemon boundaries.

We need a hard lock that applies before YD-2..YD-7 and prevents:

- daemon as implicit second owner runtime,
- source-plane mediation bypassing `exec`,
- naming drift across docs/build/help surfaces.

## Decision

YAI adopts a fixed YD-1 topology contract:

- `yai` is the owner runtime and only canonical workspace truth.
- `yai-daemon` is the standalone subordinate edge runtime binary.
- topology is `distributed acquisition / centralized control`.
- `exec` is the active owner/daemon mediation layer for source-plane runtime
  transport/routing/gating handoff.
- edge observation/action mediation/enforcement are delegated and owner-scoped.

## Rationale

- Stabilizes architecture before behavioral slices.
- Reduces corrective refactor debt in ingest/transport/CLI/SDK waves.
- Keeps source-plane scaling aligned with centralized governance truth.

## Consequences

### Positive

- deterministic boundary for all downstream slices
- clear operator/maintainer naming (`yai` vs `yai-daemon`)
- explicit `core` vs `exec` role separation

### Negative

- edge-side autonomy remains intentionally constrained in v1
- early docs and help text must be kept in strict alignment

## Non-goals

- secure peering implementation details
- full daemon behavior implementation
- multi-peer conflict/scheduling engines

## References

- `docs/program/adr/adr-013-acquisition-control.md`
- `docs/architecture/daemon-architecture-refoundation-model.md`
- `docs/architecture/protocol/source-plane.md`
