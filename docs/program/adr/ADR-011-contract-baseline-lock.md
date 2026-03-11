---
id: ADR-011
status: draft
effective_date: 2026-02-19
supersedes: []
applies_to:
  runbook: docs/program/milestone-packs/runtime-baselines/contract-baseline-lock.md
  phase: 0.1.0
  anchor: "#phase-0-1-0-pin-baseline-freeze"
law_refs:
  - ../law/foundation/invariants/I-001-traceability.md
  - ../law/foundation/invariants/I-002-determinism.md
  - ../law/foundation/invariants/I-003-governance.md
  - ../law/foundation/invariants/I-006-external-effect-boundary.md
  - ../law/foundation/boundaries/L1-kernel.md
---
# ADR-011 - Contract Baseline Lock for Milestone 1

## Context

Milestone 1 exposed three recurring risks:

- Contract drift between specs and CLI/runtime behavior
- Green pipelines with skipped mandatory proof steps
- Inconsistent evidence quality for TRL-facing claims

## Decision

Milestone 1 enforces a contract baseline lock across `law`, `yai`, and `cli`.

Controls:

1. CI parity checks between pinned specs contract and CLI/runtime behavior
2. Mandatory gates fail on missing capability (no pass-on-skip)
3. Formal/core verify updates are required when contract deltas affect authority/envelope invariants
4. Cross-repo pins remain explicit and auditable

## Rationale

A lock provides a stable legal/technical floor so later runbook phases can evolve without losing evidence integrity.

## Consequences

- Positive:
  - Stronger confidence in cross-repo compatibility.
  - Better audit quality and clearer TRL narrative.
- Negative:
  - Higher short-term coordination cost for contract-touching changes.

## Traceability

- Proposals:
  - `docs/program/rfc/RFC-002-unified-rpc-and-cli-contract.md`
  - `docs/program/rfc/RFC-004-contract-baseline-lock-and-pin-policy.md`
  - `docs/program/rfc/RFC-005-formal-coverage-roadmap.md`
- Implemented by runbooks:
  - `docs/program/milestone-packs/runtime-baselines/contract-baseline-lock.md`
  - `docs/program/milestone-packs/runtime-baselines/root-hardening.md` (downstream hardening)
- Milestone packs:
  - `docs/program/milestone-packs/contract-baseline-lock/MP-CONTRACT-BASELINE-LOCK-0.1.0.md` (planned)

## Status

Draft; intended for acceptance at Milestone 1 governance kickoff.
