# Traceability Map

This is the navigable index that prevents docs drift.
It maps proposal and decision artifacts to contract anchors and delivery evidence.

## How to use this map
- Add a row when a new Proposal, ADR, or Runbook is introduced.
- Keep links as repo-relative paths.
- Anchor every row to real `deps/yai-specs` paths.

## Proposal -> ADR -> Runbook -> MP map

| Proposal (L2) | ADR targets (L3) | Runbook (L4) | Milestone Packs (L5) |
|---|---|---|---|
| `docs/design/proposals/PRP-001-runtime-topology-and-authority.md` | `docs/design/adr/ADR-001-single-runtime.md`<br/>`docs/design/adr/ADR-002-root-entrypoint.md`<br/>`docs/design/adr/ADR-003-kernel-authority.md`<br/>`docs/design/adr/ADR-004-engine-execution.md`<br/>`docs/design/adr/ADR-005-mind-proposer.md` | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` |
| `docs/design/proposals/PRP-002-unified-rpc-and-cli-contract.md` | `docs/design/adr/ADR-006-unified-rpc.md`<br/>`docs/design/adr/ADR-011-contract-baseline-lock.md` | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md`<br/>`docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.1.md` |
| `docs/design/proposals/PRP-003-workspace-lifecycle-and-isolation.md` | `docs/design/adr/ADR-007-workspace-isolation.md`<br/>`docs/design/adr/ADR-008-connection-lifecycle.md`<br/>`docs/design/adr/ADR-009-engine-attachment.md`<br/>`docs/design/adr/ADR-010-boot-entrypoint.md` | `docs/runbooks/workspaces-lifecycle.md`<br/>`docs/runbooks/engine-attach.md` | *(TBD for workspace/engine tracks)* |
| `docs/design/proposals/PRP-004-contract-baseline-lock-and-pin-policy.md` | `docs/design/adr/ADR-011-contract-baseline-lock.md` | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.0.md` |
| `docs/design/proposals/PRP-005-formal-coverage-roadmap.md` | `docs/design/adr/ADR-006-unified-rpc.md`<br/>`docs/design/adr/ADR-011-contract-baseline-lock.md` | `docs/runbooks/root-hardening.md` | `docs/milestone-packs/root-hardening/MP-ROOT-HARDENING-0.1.5.md` |

## Delivery evidence pointers (L6)

| Track | Tests / Gates / Evidence |
|---|---|
| Root hardening | `docs/test-plans/hardfail.md`, runbook commands in `docs/runbooks/root-hardening.md`, CI logs/artifacts |
| Workspaces lifecycle | `docs/test-plans/hardfail.md`, runbook commands in `docs/runbooks/workspaces-lifecycle.md`, CI logs/artifacts |

Notes:
- Keep this map synchronized whenever proposal scope or ADR targets change.
- Do not invent new anchors: always anchor to `deps/yai-specs` paths.
