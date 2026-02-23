---
id: PRP-002
title: Unified RPC envelope and CLI contract alignment
status: draft
owner: core-protocol
effective_date: 2026-02-19
revision: 1
supersedes: []
related:
  adr:
    - docs/design/adr/ADR-006-unified-rpc.md
    - docs/design/adr/ADR-011-contract-baseline-lock.md
  runbooks:
    - docs/runbooks/root-hardening.md
  milestone_packs:
    - docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md
  specs:
    - deps/yai-specs/specs/protocol/include/transport.h
    - deps/yai-specs/specs/protocol/include/protocol.h
    - deps/yai-specs/specs/protocol/runtime/include/rpc_runtime.h
    - deps/yai-specs/specs/cli/schema/commands.v1.json
tags:
  - rpc
  - cli
  - contract
  - anti-drift
---

# PRP-002 - Unified RPC envelope and CLI contract alignment

## Problem
Contract drift between protocol/runtime headers and CLI command definitions can produce green CI with incomplete semantic alignment.

## Scope
- In scope: Envelope contract, command-surface alignment, deterministic reject semantics, anti-drift controls.
- Out of scope: Workspace lifecycle internals and engine attach strategy.

## Proposed Change
Adopt one canonical RPC surface and enforce CLI-to-spec alignment through explicit baseline checks and mandatory non-skip gate policy.

## Options Compared
- Option A: Strict single-surface contract with CI anti-drift enforcement.
- Option B: Multi-surface transition period with compatibility adapters.

## Risks
- Short-term CI noise due to stricter checks. Mitigation: phased enforcement with visible failure taxonomy.
- Cross-repo sync overhead. Mitigation: pin policy and sync scripts.

## Rollout Sketch
1. Freeze baseline artifact list.
2. Add contract comparison checks in CI.
3. Enforce deterministic reject matrix for mandatory steps.

## Exit Criteria
- [ ] Baseline artifact list is explicit and versioned.
- [ ] CI detects spec/CLI drift deterministically.
- [ ] Mandatory gates fail on missing capability (no pass-on-skip).

## Traceability

- Spec anchors (if any): `deps/yai-specs/specs/protocol/include/transport.h`, `deps/yai-specs/specs/protocol/include/protocol.h`, `deps/yai-specs/specs/protocol/runtime/include/rpc_runtime.h`, `deps/yai-specs/specs/cli/schema/commands.v1.json`
- Targets ADR: `docs/design/adr/ADR-006-unified-rpc.md`, `docs/design/adr/ADR-011-contract-baseline-lock.md`
- Downstream runbook: `docs/runbooks/root-hardening.md`
- Downstream MP: `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`

## References
- `docs/design/spine.md`
- `docs/design/adr/ADR-006-unified-rpc.md`
- `docs/design/adr/ADR-011-contract-baseline-lock.md`
