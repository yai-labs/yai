---
id: ADR-015
status: accepted
effective_date: 2026-03-11
supersedes: []
applies_to:
  phase: YD-1
  anchor: "#daemon-architecture-refoundation-slice"
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

- `docs/program/22-adr/ADR-013-distributed-acquisition-centralized-control.md`
- `docs/architecture/daemon-architecture-refoundation-model.md`
- `docs/architecture/exec-source-plane-role.md`
